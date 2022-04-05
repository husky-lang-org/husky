mod kind;
mod query;

pub use kind::*;
pub use query::{EntityQueryGroup, EntityQueryGroupStorage};

use entity_route::{EntityRoutePtr, InputPlaceholder, RangedScope};
use file::FilePtr;
use semantics_eager::*;
use semantics_lazy::{LazyExpr, LazyExprKind, LazyOpnKind, LazyStmt, LazyStmtKind};
use std::sync::Arc;
use text::TextRange;
use unique_vector::UniqVec;
use vc::{Uid, VersionControl};
use word::CustomIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    pub ident: CustomIdentifier,
    pub kind: Arc<EntityKind>,
    pub subentities: Arc<Vec<Arc<Entity>>>,
    pub scope: EntityRoutePtr,
    pub file: FilePtr,
    pub range: TextRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityInnerId {
    raw: usize,
}

pub type EntityVersionControl = VersionControl<EntityRoutePtr, EntityKind>;
pub type EntityUid = Uid;

pub trait ControlEntityVersion {
    fn entity_vc(&self) -> &EntityVersionControl;
    fn entity_uid(&self, scope: EntityRoutePtr) -> Uid {
        self.entity_vc().uid(scope)
    }
}

impl Entity {
    pub fn kind(&self) -> &EntityKind {
        &self.kind
    }

    fn dependees(kind: &EntityKind) -> UniqVec<EntityRoutePtr> {
        return match kind {
            EntityKind::Module { .. } => Default::default(),
            EntityKind::Feature { ty, lazy_stmts } => {
                let mut v = UniqVec::new();
                v.push(ty.scope);
                extract_lazy_stmts_dependees(lazy_stmts, &mut v);
                v
            }
            EntityKind::Pattern { .. } => todo!(),
            EntityKind::Func {
                input_placeholders: inputs,
                output,
                stmts,
            } => {
                let mut v = UniqVec::new();
                extract_routine_head_dependees(inputs, output, &mut v);
                extract_decl_stmts_dependees(stmts, &mut v);
                v
            }
            EntityKind::Proc {
                input_placeholders: inputs,
                output,
                stmts,
            } => {
                let mut v = UniqVec::new();
                extract_routine_head_dependees(inputs, output, &mut v);
                extract_impr_stmts_dependees(stmts, &mut v);
                v
            }
            EntityKind::Ty(ty) => match ty.kind {
                TyDefnKind::Enum { ref variants } => {
                    let mut v = UniqVec::new();
                    variants.iter().for_each(|(_ident, variant_kind)| {
                        extract_enum_variant_dependees(variant_kind, &mut v)
                    });
                    v
                }
                TyDefnKind::Struct { ref memb_vars, .. } => memb_vars
                    .iter()
                    .map(|(_ident, memb_var)| memb_var.ty)
                    .into(),
                TyDefnKind::Record {
                    ref memb_vars,
                    ref memb_features,
                } => {
                    let mut v = UniqVec::new();
                    memb_vars
                        .iter()
                        .for_each(|(_ident, memb_var_decl)| v.push(memb_var_decl.ty));
                    memb_features
                        .iter()
                        .for_each(|(_ident, memb_feature_defn)| v.push(memb_feature_defn.ty));
                    v
                }
            },
            EntityKind::Main(_) => todo!(),
        };

        fn extract_routine_head_dependees(
            inputs: &[InputPlaceholder],
            output: &RangedScope,
            v: &mut UniqVec<EntityRoutePtr>,
        ) {
            for input_placeholder in inputs.iter() {
                v.push(input_placeholder.ranged_ty.scope)
            }
            v.push(output.scope);
        }

        fn extract_lazy_stmts_dependees(stmts: &[Arc<LazyStmt>], v: &mut UniqVec<EntityRoutePtr>) {
            for stmt in stmts {
                match stmt.kind {
                    LazyStmtKind::Init { varname, ref value } => todo!(),
                    LazyStmtKind::Assert { ref condition } => todo!(),
                    LazyStmtKind::Return { ref result } => extract_lazy_expr_dependees(result, v),
                    LazyStmtKind::Branches { kind, ref branches } => todo!(),
                }
            }
        }

        fn extract_decl_stmts_dependees(stmts: &[Arc<FuncStmt>], v: &mut UniqVec<EntityRoutePtr>) {
            for stmt in stmts {
                match stmt.kind {
                    FuncStmtKind::Init {
                        varname,
                        initial_value: ref value,
                    } => extract_eager_expr_dependees(value, v),
                    FuncStmtKind::Assert { ref condition } => {
                        extract_eager_expr_dependees(condition, v)
                    }
                    FuncStmtKind::Return { ref result } => extract_eager_expr_dependees(result, v),
                    FuncStmtKind::Branches { kind, ref branches } => {
                        for branch in branches {
                            extract_decl_stmts_dependees(&branch.stmts, v)
                        }
                    }
                }
            }
        }

        fn extract_impr_stmts_dependees(stmts: &[Arc<ProcStmt>], v: &mut UniqVec<EntityRoutePtr>) {
            for stmt in stmts {
                match stmt.kind {
                    ProcStmtKind::Init {
                        varname,
                        ref initial_value,
                        ..
                    } => extract_eager_expr_dependees(initial_value, v),
                    ProcStmtKind::Assert { ref condition } => {
                        extract_eager_expr_dependees(condition, v)
                    }
                    ProcStmtKind::Return { ref result } => extract_eager_expr_dependees(result, v),
                    ProcStmtKind::Execute { ref expr } => extract_eager_expr_dependees(expr, v),
                    ProcStmtKind::BranchGroup { kind, ref branches } => {
                        for branch in branches {
                            extract_impr_stmts_dependees(&branch.stmts, v)
                        }
                    }
                    ProcStmtKind::Loop {
                        ref loop_kind,
                        ref stmts,
                    } => {
                        match loop_kind {
                            LoopKind::For {
                                ref initial_boundary,
                                ref final_boundary,
                                ..
                            } => {
                                extract_boundary_dependees(initial_boundary, v);
                                extract_boundary_dependees(final_boundary, v);
                            }
                            LoopKind::ForExt {
                                ref final_boundary, ..
                            } => {
                                extract_boundary_dependees(final_boundary, v);
                            }
                            LoopKind::While { condition } => {
                                extract_eager_expr_dependees(condition, v)
                            }
                            LoopKind::DoWhile { condition } => {
                                extract_eager_expr_dependees(condition, v)
                            }
                        }
                        extract_impr_stmts_dependees(stmts, v)
                    }
                }
            }
        }

        fn extract_lazy_expr_dependees(expr: &LazyExpr, v: &mut UniqVec<EntityRoutePtr>) {
            match expr.kind {
                LazyExprKind::Variable(_) | LazyExprKind::PrimitiveLiteral(_) => (),
                LazyExprKind::Scope { scope, .. } => v.push(scope),
                LazyExprKind::EnumLiteral { scope, ref value } => todo!(),
                LazyExprKind::Bracketed(_) => todo!(),
                LazyExprKind::Opn {
                    opn_kind,
                    compiled,
                    ref opds,
                } => {
                    match opn_kind {
                        LazyOpnKind::Binary { .. }
                        | LazyOpnKind::Prefix(_)
                        | LazyOpnKind::MembAccess { .. }
                        | LazyOpnKind::MembCall { .. } => (),
                        LazyOpnKind::RoutineCall(routine) => v.push(routine.scope),
                        LazyOpnKind::StructCall(ty) => v.push(ty.scope),
                        LazyOpnKind::ClassCall(ty) => v.push(ty.scope),
                        LazyOpnKind::PatternCall => todo!(),
                        LazyOpnKind::ElementAccess => todo!(),
                    }
                    for opd in opds {
                        extract_lazy_expr_dependees(opd, v)
                    }
                }
                LazyExprKind::Lambda(_, _) => todo!(),
                LazyExprKind::This => todo!(),
                LazyExprKind::ScopedFeature { scope } => todo!(),
            }
        }

        fn extract_eager_expr_dependees(expr: &EagerExpr, v: &mut UniqVec<EntityRoutePtr>) {
            match expr.kind {
                EagerExprKind::Variable(_) => (),
                EagerExprKind::Scope { scope, compiled } => v.push(scope),
                EagerExprKind::PrimitiveLiteral(_) => (),
                EagerExprKind::Bracketed(ref expr) => extract_eager_expr_dependees(expr, v),
                EagerExprKind::Opn {
                    ref opn_kind,
                    ref opds,
                    ..
                } => {
                    match opn_kind {
                        EagerOpnKind::Binary { .. }
                        | EagerOpnKind::Prefix { .. }
                        | EagerOpnKind::Suffix { .. }
                        | EagerOpnKind::MembVarAccess { .. }
                        | EagerOpnKind::MembRoutineCall { .. }
                        | EagerOpnKind::ElementAccess => (),
                        EagerOpnKind::RoutineCall(routine) => v.push(routine.scope),
                        EagerOpnKind::TypeCall { ranged_ty, .. } => v.push(ranged_ty.scope),
                        EagerOpnKind::PatternCall => todo!(),
                    }
                    for opd in opds {
                        extract_eager_expr_dependees(opd, v)
                    }
                }
                EagerExprKind::Lambda(_, _) => todo!(),
                EagerExprKind::This => todo!(),
            }
        }

        fn extract_boundary_dependees(boundary: &Boundary, v: &mut UniqVec<EntityRoutePtr>) {
            boundary
                .opt_bound
                .as_ref()
                .map(|bound| extract_eager_expr_dependees(bound, v));
        }

        fn extract_enum_variant_dependees(
            variant_kind: &EnumVariantKind,
            v: &mut UniqVec<EntityRoutePtr>,
        ) {
            match variant_kind {
                EnumVariantKind::Constant => (),
            }
        }
    }

    pub(crate) fn new(
        ident: CustomIdentifier,
        kind: EntityKind,
        subentities: Arc<Vec<Arc<Entity>>>,
        scope: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
        vc: &EntityVersionControl,
    ) -> Entity {
        let kind = Arc::new(kind);
        vc.update(scope, kind.clone(), &Self::dependees(&kind));
        Self {
            ident,
            kind,
            subentities,
            scope,
            file,
            range,
        }
    }
}
