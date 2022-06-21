use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub struct HuskyTracerGuiMessage {
    pub opt_request_id: Option<usize>,
    pub variant: HuskyTracerGuiMessageVariant,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "kind")]
pub enum HuskyTracerGuiMessageVariant {
    InitDataRequest,
    Activate {
        trace_id: TraceId,
        opt_attention_for_figure: Option<Attention>,
    },
    ToggleExpansion {
        trace_id: TraceId,
    },
    ToggleShow {
        trace_id: TraceId,
    },
    Trace {
        id: TraceId,
    },
    LockAttention {
        attention: Attention,
        opt_active_trace_id_for_request: Option<TraceId>,
        request_figure: bool,
        request_stalk: bool,
    },
    TraceStalk {
        trace_id: TraceId,
        input_id: usize,
    },
    UpdateFigureControlData {
        trace_id: TraceId,
        attention: Attention,
        figure_control_props: FigureControlData,
    },
}
