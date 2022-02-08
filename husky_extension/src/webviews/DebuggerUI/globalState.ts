import type { Trace, FigureProps } from "server/types";
import type { Readable, Writable } from "svelte/store";
import { writable, get } from "svelte/store";
import DebuggerResponse, { tDebuggerResponse } from "./DebuggerResponse";
import { isRight } from "fp-ts/Either";
import { PathReporter } from "io-ts/PathReporter";

const version: Writable<number> = writable(0);

const activeTraceIdx_: Writable<number | null> = writable(null);
export const activeTraceIdx: Readable<number | null> = activeTraceIdx_;

let figures: { [id: number]: FigureProps | null } = {};
const figure_: Writable<FigureProps | null> = writable(null);
export const figure: Readable<FigureProps | null> = figure_;

let expansions: { [id: number]: Writable<boolean> } = {};

let subtraces_table: { [id: number]: Writable<Trace[] | null> } = {};
let traces: { [id: number]: Trace } = {};
const rootTraces_: Writable<Trace[]> = writable([]);
export const rootTraces: Readable<Trace[]> = rootTraces_;

let traceList: number[] = [];

const websocket = new WebSocket("ws://localhost:51617/query");
init_websocket(websocket);
function init_websocket(websocket: WebSocket) {
    websocket.addEventListener("open", (event: Event) => {
        websocket.send(rootTracesQuery);
    });
    websocket.addEventListener("message", function (event: MessageEvent) {
        let data: DebuggerResponse = JSON.parse(event.data);
        const result = tDebuggerResponse.decode(data);
        if (isRight(result)) {
            console.log(data);
            switch (data.type) {
                case "RootTraces":
                    setRootTraces(data.root_traces);
                    break;
                case "Subtraces":
                    setSubtraces(data.id, data.subtraces);
                    break;
                case "Figure":
                    setFigure(data.id, data.figure);
                    break;
                case "DidActivate":
                    didActivate(data.id);
                    break;
                case "DidToggleExpansion":
                    didToggleExpansion(data.id);
                    break;
            }
        } else {
            console.error("invalid response: ", data);
            console.log(PathReporter.report(result));
        }
    });

    const rootTracesQuery = JSON.stringify({ type: "RootTraces" });

    function setRootTraces(traces: Trace[]) {
        rootTraces_.set(traces);
        addTraces(traces);
    }

    function setSubtraces(id: number, subtraces: Trace[]) {
        if (!(id in subtraces_table)) {
            console.error(id, subtraces_table);
        }
        subtraces_table[id].set(subtraces);
        addTraces(subtraces);
    }

    function setFigure(id: number, figure: FigureProps) {
        figures[id] = figure;
        if (id === get(activeTraceIdx)) {
            figure_.set(figure);
        }
    }

    function didActivate(id: number) {
        activeTraceIdx_.set(id);
        figure_.set(figures[id]);
    }

    function didToggleExpansion(id: number) {
        expansions[id].update((expanded) => !expanded);
        updateTraceList();
    }
}

function updateTraceList() {
    console.log("root traces: ", get(rootTraces));
    for (const trace of get(rootTraces)) {
        updateTraceListDfs(trace.id);
    }

    function updateTraceListDfs(id: number) {
        traceList.push(id);
        if (isExpanded(id)) {
            let subtraces: Trace[] = get(getSubtraces(id)) || [];
            for (const trace of subtraces) {
                updateTraceListDfs(trace.id);
            }
        }
    }
}

export function toggleExpansion(id: number) {
    if (hasChildren(id)) {
        websocket.send(JSON.stringify({ type: "ToggleExpansion", id }));
    }
}

export function activate(id: number) {
    if (get(activeTraceIdx) !== id) {
        prepareFigure(id);
        websocket.send(JSON.stringify({ type: "Activate", id }));
    }

    function prepareFigure(id: number) {
        if (!(id in figures)) {
            figures[id] = null;
            websocket.send(JSON.stringify({ type: "Figure", id }));
        }
    }
}

export function onKeyDown(e: KeyboardEvent) {
    switch (e.code) {
        case "KeyH":
            moveLeft();
            break;
        case "KeyL":
            moveRight();
            break;
        case "KeyJ":
            moveDown();
            break;
        case "KeyK":
            moveUp();
            break;
        case "KeyS":
            console.error("TODO");
            break;
        default:
    }

    function moveUp() {
        const id = get(activeTraceIdx);
        if (id !== null) {
            const before = idxBefore(id);
            if (before !== undefined) {
                return activate(before);
            }
        }

        function idxBefore(id: number): number | undefined {
            return traceList[traceList.indexOf(id) - 1];
        }
    }

    function moveDown() {
        const id = get(activeTraceIdx);
        if (id !== null) {
            const after = idxAfter(id);
            if (after !== undefined) {
                return activate(after);
            }
        }

        function idxAfter(id: number): number | undefined {
            return traceList[traceList.indexOf(id) + 1];
        }
    }

    function moveRight() {
        const id = get(activeTraceIdx);
        if (id !== null) {
            if (!isExpanded(id) && hasChildren(id)) {
                toggleExpansion(id);
                moveDown();
            }
        }
    }

    function moveLeft() {
        let id = get(activeTraceIdx);
        if (id !== null) {
            const trace = traces[id];
            if (trace.parent !== null) {
                toggleExpansion(trace.parent);
                activate(trace.parent);
            }
        }
    }
}

function hasChildren(id: number) {
    const children = get(getSubtraces(id));
    return children !== null && children.length > 0;
}

export function getDummy() {}

export function isExpanded(id: number): Readable<boolean> {
    if (!(id in expansions)) {
        // console.error("id ", id, " is not in expansions");
        throw new Error(`id ${id} is not in expansions`);
    }
    return expansions[id];
}

export function getSubtraces(id: number | null): Readable<Trace[] | null> {
    if (id === null) {
        return rootTraces;
    }
    if (id in subtraces_table) {
        return subtraces_table[id];
    } else {
        const subtraces = writable(null);
        subtraces_table[id] = subtraces;
        querySubtraces(id);
        return subtraces;
    }

    function querySubtraces(id: number) {
        websocket.send(JSON.stringify({ type: "Subtraces", id }));
    }
}

function addTraces(new_traces: Trace[]) {
    for (const trace of new_traces) {
        traces[trace.id] = trace;
        expansions[trace.id] = writable(false);
    }
}
