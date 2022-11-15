use crate::*;
use husky_entity_route::Ty;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TraceSketch {
    Main,
    Module(Ty),
    EntityFeature(Ty),
}

impl AsTraceSketch for TraceSketch {
    type Node = TraceNode;

    fn new(node: &Self::Node) -> Option<Self> {
        match node.trace().variant {
            TraceVariant::Main(_) => Some(TraceSketch::Main),
            TraceVariant::Module {
                route,
                file: _,
                range: _,
            } => Some(TraceSketch::Module(route)),
            TraceVariant::EntityFeature { route, repr: _ } => {
                Some(TraceSketch::EntityFeature(route))
            }
            TraceVariant::FeatureStmt(_) => None,
            TraceVariant::FeatureBranch(_) => None,
            TraceVariant::FeatureExpr(_) => None,
            TraceVariant::FeatureCallArgument {
                name: _,
                argument: _,
            } => None,
            TraceVariant::FuncStmt { .. } => None,
            TraceVariant::ProcStmt { .. } => None,
            TraceVariant::ProcBranch { .. } => None,
            TraceVariant::FuncBranch { .. } => None,
            TraceVariant::LoopFrame { .. } => None,
            TraceVariant::EagerExpr { .. } => None,
            TraceVariant::EagerCallArgument { .. } => None,
            TraceVariant::CallHead { .. } => None,
        }
    }
}
