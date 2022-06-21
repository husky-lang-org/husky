use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubtracesKey {
    Simple { trace_id: TraceId },
    FeatureExprStalk { trace_id: TraceId, input_id: usize },
    Null,
}

impl SubtracesKey {
    pub fn new(attention: &Attention, trace_kind: TraceKind, trace_id: TraceId) -> SubtracesKey {
        match trace_kind {
            TraceKind::Main
            | TraceKind::FeatureStmt
            | TraceKind::FeatureBranch
            | TraceKind::FuncStmt
            | TraceKind::ProcStmt
            | TraceKind::ProcBranch
            | TraceKind::LoopFrame
            | TraceKind::EagerExpr => SubtracesKey::Simple { trace_id },
            TraceKind::FeatureCallInput | TraceKind::CallHead => SubtracesKey::Null,
            TraceKind::FeatureExpr => match attention {
                Attention::Specific { input_id } => SubtracesKey::FeatureExprStalk {
                    trace_id,
                    input_id: *input_id,
                },
                Attention::Generic { .. } => SubtracesKey::Null,
            },
        }
    }
}
