use std::sync::Arc;

use vm::{BoundaryKind, LoopStep, StackIdx, VMLoopKind};
use word::RangedCustomIdentifier;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoopVariant {
    For {
        frame_var: RangedCustomIdentifier,
        initial_boundary: Boundary,
        final_boundary: Boundary,
        step: LoopStep,
    },
    ForExt {
        frame_var: RangedCustomIdentifier,
        frame_varidx: StackIdx,
        final_boundary: Boundary,
        step: LoopStep,
    },
    While {
        condition: Arc<EagerExpr>,
    },
    DoWhile {
        condition: Arc<EagerExpr>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Boundary {
    pub opt_bound: Option<Arc<EagerExpr>>,
    pub kind: BoundaryKind,
}

impl Into<VMLoopKind> for &LoopVariant {
    fn into(self) -> VMLoopKind {
        match self {
            LoopVariant::For {
                frame_var,
                initial_boundary,
                final_boundary,
                step,
            } => VMLoopKind::For {
                initial_boundary_kind: initial_boundary.kind,
                final_boundary_kind: final_boundary.kind,
                step: *step,
                frame_var: frame_var.ident,
            },
            LoopVariant::ForExt {
                frame_var,
                final_boundary,
                frame_varidx,
                step,
            } => VMLoopKind::ForExt {
                final_boundary_kind: final_boundary.kind,
                step: *step,
                frame_var: frame_var.ident,
                frame_varidx: *frame_varidx,
            },
            LoopVariant::While { .. } | LoopVariant::DoWhile { .. } => VMLoopKind::Loop,
        }
    }
}
