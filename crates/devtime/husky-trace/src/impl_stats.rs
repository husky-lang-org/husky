use crate::*;

use husky_print_utils::msg_once;

use husky_trace_protocol_old::TraceStats;
use husky_vm::{RegularValue, VMResult};

impl TraceVariant {
    pub fn opt_stats_result(
        &self,
        runtime: &dyn TraceRuntime,
        partitions: &Partitions,
    ) -> VMResult<Option<TraceStats>> {
        match self {
            TraceVariant::Main(..) => todo!(),
            // val_repr_opt_stats(runtime, partitions, repr, None),
            TraceVariant::Module { .. } => Ok(None),
            TraceVariant::EntityVal { .. } => {
                todo!()
                // val_repr_opt_stats(runtime, partitions, repr, None)
            }
            TraceVariant::ValStmt(stmt) => feature_stmt_opt_stats(runtime, partitions, stmt),
            TraceVariant::ValBranch(branch) => {
                todo!()
                // feature_branch_opt_stats(runtime, partitions, branch)
            }
            TraceVariant::LazyExpr(expr) => feature_expr_opt_stats(runtime, partitions, *expr),
            TraceVariant::ValCallArgument { .. }
            | TraceVariant::EagerStmt { .. }
            | TraceVariant::EagerBranch { .. }
            | TraceVariant::LoopFrame { .. }
            | TraceVariant::EagerExpr { .. }
            | TraceVariant::EagerCallArgument { .. }
            | TraceVariant::CallHead { .. } => Ok(None),
        }
    }
}

fn val_repr_opt_stats(
    db: &dyn TraceRuntime,
    partitions: &Partitions,
    // repr: &ValRepr,
    opt_arrival_indicator: Option<&ValDomain>,
) -> VMResult<Option<TraceStats>> {
    todo!()
    // feature_opt_stats(
    //     db,
    //     partitions,
    //     repr.ty(),
    //     |sample_id| db.eval_feature_repr_cached(repr, sample_id),
    //     opt_arrival_indicator,
    // )
}

fn feature_stmt_opt_stats(
    _db: &dyn TraceRuntime,
    _partitions: &Partitions,
    _stmt: &ValStmt,
) -> VMResult<Option<TraceStats>> {
    todo!()
    // match stmt.variant {
    //     ValStmtData::Init { .. } | ValStmtData::Assert { .. } => Ok(None),
    //     ValStmtData::Require { return_context, .. }
    //     | ValStmtData::ReturnUnveil { return_context, .. } => feature_opt_stats(
    //         db,
    //         partitions,
    //         return_context.return_ty(),
    //         |sample_id| db.eval_feature_stmt(stmt, sample_id),
    //         stmt.opt_arrival_indicator.as_ref(),
    //     ),
    //     ValStmtData::Return { ref result } => {
    //         feature_expr_opt_stats(db, partitions, result)
    //     }
    //     ValStmtData::ReturnHtml { .. } => todo!(),
    //     ValStmtData::ConditionFlow { .. } => todo!(),
    // }
}

fn feature_branch_opt_stats(
    db: &dyn TraceRuntime,
    partitions: &Partitions,
    // branch: ValBranch,
) -> VMResult<Option<TraceStats>> {
    todo!()
    // msg_once!("consider whether condition is satisfied");
    // feature_opt_stats(
    //     db,
    //     partitions,
    //     branch.block.return_ty.route,
    //     |sample_id| -> __VMResult<__RegularValue> {
    //         match branch.variant {
    //             FeatureLazyBranchVariant::If { ref condition } => {
    //                 if !db.eval_feature_expr(condition, sample_id)?.to_bool() {
    //                     return Ok(__RegularValue::unreturned());
    //                 }
    //             }
    //             FeatureLazyBranchVariant::Elif { ref condition } => {
    //                 if !db.eval_feature_expr(condition, sample_id)?.to_bool() {
    //                     return Ok(__RegularValue::unreturned());
    //                 }
    //             }
    //             FeatureLazyBranchVariant::Else => (),
    //         }
    //         db.eval_feature_lazy_block(&branch.block, sample_id)
    //     },
    //     branch.opt_arrival_indicator.as_ref(),
    // )
}

fn feature_expr_opt_stats(
    db: &dyn TraceRuntime,
    partitions: &Partitions,
    expr: ValExpr,
) -> VMResult<Option<TraceStats>> {
    todo!()
    // feature_opt_stats(
    //     db,
    //     partitions,
    //     expr.expr.intrinsic_ty(),
    //     |sample_id| db.eval_feature_expr(expr, sample_id),
    //     expr.opt_domain_indicator.as_ref(),
    // )
}

fn feature_opt_stats(
    _db: &dyn TraceRuntime,
    _partitions: &Partitions,
    _feature_ty: EtherealTerm,
    _compute_value: impl Fn(SampleId) -> VMResult<RegularValue>,
    _opt_arrival_indicator: Option<&ValDomain>,
) -> VMResult<Option<TraceStats>> {
    todo!()
    // let target_return_ty = db.target_return_ty().unwrap();
    // // todo check this could cause some problem
    // if !db.is_implicitly_castable(feature_ty, target_return_ty) {
    //     return Ok(None);
    // }
    // let mut dev_samples = 0;
    // let mut dev_arrivals = 0;
    // let mut dev_unreturneds = 0;
    // let mut dev_nones = 0;
    // let mut dev_trues = 0;
    // let mut dev_falses = 0;
    // let mut dev_partition_noness = partitions.init_partition_values();
    // let convert_register_to_label = db.register_to_label_converter();
    // for labeled_data in db.session().dev().each_labeled_data() {
    //     if dev_samples >= MAX_SAMPING_SIZE {
    //         break;
    //     }
    //     dev_samples += 1;
    //     let sample_id = labeled_data.sample_id;
    //     if !db
    //         .eval_opt_domain_indicator_cached(opt_arrival_indicator, sample_id)
    //         .map_err(|e| -> __VMError { (sample_id.0, e).into() })?
    //     {
    //         continue;
    //     }
    //     dev_arrivals += 1;
    //     let value = compute_value(sample_id)
    //         .map_err(|e| -> __VMError { (labeled_data.sample_id.0, e).into() })?;
    //     match convert_register_to_label(&value) {
    //         __RegisterDowncastResult::Value(prediction) => match prediction == labeled_data.label {
    //             true => dev_trues += 1,
    //             false => dev_falses += 1,
    //         },
    //         __RegisterDowncastResult::None { .. } => {
    //             dev_nones += 1;
    //             let idx = partitions.partition_idx(labeled_data.label);
    //             dev_partition_noness[idx].1 += 1;
    //         }
    //         __RegisterDowncastResult::Unreturned => dev_unreturneds += 1,
    //     }
    // }
    // Ok(Some(TraceStats::Classification {
    //     dev_samples,
    //     dev_arrivals,
    //     dev_unreturneds,
    //     dev_nones,
    //     dev_trues,
    //     dev_falses,
    //     dev_partition_noness,
    // }))
}
