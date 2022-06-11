use super::*;

impl TreeContext {
    pub(super) fn expanded(&self, trace_id: TraceId) -> bool {
        self.trace_nodes.borrow()[trace_id.0].expanded.get_cloned()
    }

    pub(super) fn expanded_signal(&self, trace_id: TraceId) -> Rc<Signal<bool>> {
        self.trace_nodes.borrow()[trace_id.0].expanded.clone()
    }

    pub(super) fn did_toggle_expansion(&mut self, trace_id: TraceId) {
        todo!()
        // let is_expanded: &mut bool = &mut self.expansion_stores[&trace_id].modify();
        // *is_expanded = !*is_expanded
        // self.update_trace_listing(focus);
    }

    pub(super) fn shown(&self, trace_id: TraceId) -> bool {
        self.trace_nodes.borrow()[trace_id.0].shown.get_cloned()
    }

    pub(super) fn shown_signal(&self, trace_id: TraceId) -> Rc<Signal<bool>> {
        self.trace_nodes.borrow()[trace_id.0].shown.clone()
    }

    pub(super) fn did_toggle_show(&mut self, trace_id: TraceId) {
        todo!()
        // self.shown_stores.update(id, (shown) => !shown);
        // self.update_trace_listing(focus);
    }

    // print_state() {
    //     console.log("trace control:");
    //     console.log("    shown_stores:");
    //     self.shown_stores.print_state(8);
    //     console.log("    expansion_stores:");
    //     self.expansion_stores.print_state(8);
    // }
}
// function load_store_table<T>(value_table: { [id_str: string]: T }): {
//     [id: number]: Writable<T>;
// } {
//     let store_table: { [id: number]: Writable<T> } = {};
//     for (const id_str in value_table.showns) {
//         const id = parseInt(id_str);
//         store_table[id] = writable(value_table[id]);
//     }
//     return store_table;
// }
