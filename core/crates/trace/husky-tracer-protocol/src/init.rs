use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InitData {
    pub attention: Attention,
    pub trace_init_data: TraceInitState,
    pub figure_canvases: Vec<(FigureCanvasKey, FigureCanvasData)>,
    pub figure_controls: Vec<(FigureControlKey, FigureControlData)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TraceInitState {
    pub trace_nodes: Vec<TraceNodeData>,
    pub opt_active_trace_id: Option<TraceId>,
    pub subtrace_ids_map: Vec<(SubtracesKey, Vec<TraceId>)>,
    pub trace_stalks: Vec<(TraceStalkKey, TraceStalkRawData)>,
    pub root_trace_ids: Vec<TraceId>,
}
