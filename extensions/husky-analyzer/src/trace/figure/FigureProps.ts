import { decode_array, d_memb, d_memb_old, d_string } from "src/decode/decode";
import type { Point2d } from "src/geom2d/geom2d";
import type Graphics2dProps from "./Graphics2d";
import type Color from "./Color";
import { decode_graphics2d } from "./Graphics2d";
import type MutationsFigureProps from "./Mutations";
import { decode_mutation } from "./Mutations";
import type PrimitiveValueFigureProps from "./Primitive";
import { decode_primitive_value } from "./Primitive";

export type PointGroup = {
    points: Point2d[];
    color: Color;
};
export type GalleryProps = { kind: "Gallery" };
export type Plot2dProps = {
    kind: "Plot2d";
    plot_kind: "Scatter";
    groups: PointGroup[];
    xrange: [number, number];
    yrange: [number, number];
};

type FigureProps =
    | GalleryProps
    | Graphics2dProps
    | Plot2dProps
    | PrimitiveValueFigureProps
    | MutationsFigureProps;
export default FigureProps;

export function decode_figure_props(data: unknown): FigureProps {
    let type = d_memb_old(data, "kind", d_string);
    switch (type) {
        case "Graphics2d":
            return decode_graphics2d(data);
        case "Primitive":
            return {
                kind: "Primitive",
                value: decode_primitive_value(d_memb(data, "value")),
            };
        case "Mutations":
            return {
                kind: "Mutations",
                mutations: decode_array(
                    d_memb(data, "mutations"),
                    decode_mutation
                ),
            };
        default:
            console.log("data is ", data);
            throw new Error("Todo");
    }
}
