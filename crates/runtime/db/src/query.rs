use crate::*;
use datasets::LabeledData;
use defn_head::InputParameter;
use eval_feature::EvalFeature;
use feature::*;
use semantics_eager::ProcStmtVariant;
use semantics_entity::EntityDefnVariant;
use text::TextQueryGroup;
use trace::*;
use upcast::Upcast;
use visual_runtime::VisualQueryGroup;
use vm::{exec_debug, EvalResult, HistoryEntry, InstructionSheet, InterpreterQueryGroup};

#[salsa::query_group(RuntimeQueryGroupStorage)]
pub trait RuntimeQueryGroup:
    AskCompileTime + CreateTrace<'static> + EvalFeature<'static> + VisualQueryGroup
{
    #[salsa::input]
    fn pack_main(&self) -> FilePtr;

    #[salsa::input]
    fn version(&self) -> usize;

    fn subtraces(
        &self,
        trace_id: TraceId,
        effective_opt_input_id: Option<usize>,
    ) -> Arc<Vec<Arc<Trace<'static>>>>;
    fn root_traces(&self) -> Arc<Vec<TraceId>>;

    fn trace_stalk(&self, trace_id: TraceId, input_id: usize) -> Arc<TraceStalk<'static>>;
}

pub fn root_traces(this: &dyn RuntimeQueryGroup) -> Arc<Vec<TraceId>> {
    let compile_time = this.compile_time();
    let pack_main = this.pack_main();
    Arc::new(vec![this
        .new_trace(
            None,
            pack_main,
            0,
            TraceVariant::Main(compile_time.main_feature_repr(pack_main).unwrap()),
        )
        .id()])
}

pub fn subtraces(
    db: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    effective_opt_input_id: Option<usize>,
) -> Arc<Vec<Arc<Trace<'static>>>> {
    let trace: &Trace = &db.trace(trace_id);
    match trace.variant {
        TraceVariant::Main(ref repr) => db.feature_repr_subtraces(&trace, repr),
        TraceVariant::FeatureStmt(_)
        | TraceVariant::FeatureCallInput { .. }
        | TraceVariant::FuncStmt { .. }
        | TraceVariant::CallHead { .. } => Arc::new(vec![]),
        TraceVariant::ProcStmt {
            ref stmt,
            ref history,
        } => match stmt.variant {
            ProcStmtVariant::Init { .. }
            | ProcStmtVariant::Assert { .. }
            | ProcStmtVariant::Execute { .. }
            | ProcStmtVariant::Return { .. } => Arc::new(vec![]),
            ProcStmtVariant::ConditionFlow { .. } => panic!(),
            ProcStmtVariant::Loop { ref stmts, .. } => {
                match history
                    .get(stmt)
                    .expect("if there is no entry, there is no subtraces")
                {
                    HistoryEntry::PureExpr { .. } | HistoryEntry::Exec { .. } => Arc::new(vec![]),
                    HistoryEntry::Loop {
                        control,
                        ref stack_snapshot,
                        body_instruction_sheet: ref body,
                        loop_kind,
                        ..
                    } => db.loop_subtraces(
                        db.compile_time(),
                        trace,
                        *loop_kind,
                        stmt,
                        stmts,
                        stack_snapshot,
                        body,
                    ),
                    HistoryEntry::ControlFlow {
                        opt_branch_entered: enter,
                        ..
                    } => todo!(),
                    HistoryEntry::Break => todo!(),
                    HistoryEntry::PatternMatching { .. } => todo!(),
                }
            }
            ProcStmtVariant::Break => Arc::new(vec![]),
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        },
        TraceVariant::FeatureExpr(ref expr) => {
            db.feature_expr_subtraces(trace, expr, effective_opt_input_id)
        }
        TraceVariant::FeatureBranch(ref branch) => db.feature_branch_subtraces(trace, branch),
        TraceVariant::EagerExpr {
            ref expr,
            ref history,
        } => db.eager_expr_subtraces(trace, expr, history),
        TraceVariant::LoopFrame {
            ref loop_frame_data,
            ref loop_stmt,
            ref body_stmts,
            ref body_instruction_sheet,
        } => db.loop_frame_subtraces(
            loop_stmt,
            body_stmts,
            body_instruction_sheet,
            loop_frame_data,
            trace,
        ),
        TraceVariant::ProcBranch {
            ref stmt,
            branch_idx,
            ref history,
            ref opt_vm_branch,
            ref branch,
            ..
        } => match history.get(stmt).unwrap() {
            HistoryEntry::ControlFlow {
                stack_snapshot,
                opt_branch_entered: branch_entered,
                ..
            } => {
                should_eq!(Some(branch_idx), *branch_entered);
                db.proc_branch_subtraces(
                    &branch.stmts,
                    &opt_vm_branch.as_ref().unwrap().body,
                    stack_snapshot,
                    trace,
                )
            }
            _ => panic!(),
        },
    }
}

pub fn trace_stalk(
    this: &dyn RuntimeQueryGroup,
    trace_id: TraceId,
    input_id: usize,
) -> Arc<TraceStalk<'static>> {
    let trace: &Trace = &this.trace(trace_id);
    Arc::new(match trace.variant {
        TraceVariant::Main(ref repr) => TraceStalk {
            extra_tokens: vec![
                trace::fade!(" = "),
                this.eval_feature_repr(repr, input_id).into(),
            ],
        },
        TraceVariant::FeatureStmt(ref stmt) => match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_feature_expr(value, input_id).into(),
                ],
            },
            FeatureStmtVariant::Assert { ref condition } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_feature_expr(condition, input_id).into(),
                ],
            },
            FeatureStmtVariant::Return { ref result } => TraceStalk {
                extra_tokens: vec![
                    trace::fade!(" = "),
                    this.eval_feature_expr(result, input_id).into(),
                ],
            },
            FeatureStmtVariant::ConditionFlow { ref branches } => panic!(),
        },
        TraceVariant::FeatureBranch(_) => TraceStalk {
            extra_tokens: vec![],
        },
        TraceVariant::FeatureExpr(ref expr) => TraceStalk {
            extra_tokens: vec![
                trace::fade!(" = "),
                this.eval_feature_expr(expr, input_id).into(),
            ],
        },
        TraceVariant::FeatureCallInput { .. } => todo!(),
        TraceVariant::FuncStmt { .. }
        | TraceVariant::ProcStmt { .. }
        | TraceVariant::EagerExpr { .. }
        | TraceVariant::CallHead { .. } => panic!(),
        TraceVariant::LoopFrame {
            loop_frame_data: ref vm_loop_frame,
            ..
        } => match vm_loop_frame.control {
            vm::ControlSnapshot::None => TraceStalk::default(),
            vm::ControlSnapshot::Return(_) => todo!(),
            vm::ControlSnapshot::Break => todo!(),
            vm::ControlSnapshot::Err(_) => todo!(),
        },
        TraceVariant::ProcBranch { .. } => panic!(),
    })
}
