mod impl_block;
mod impl_expr;
mod impl_repr;
mod impl_stmt;
mod impl_visualize;
mod indicator;
mod sheet;

use husky_tracer_protocol::SampleId;
pub use indicator::FeatureEvalIndicator;
pub use sheet::*;

use crate::*;
use feature_gen::FeatureEvalId;
use vm::EvalResult;
use vm::{AnyValueDyn, EvalValue};

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    pub(crate) sample_id: SampleId,
    pub(crate) eval_input: EvalValue<'eval>,
    pub(crate) sheet: &'a EvalSheet<'eval>,
    pub(crate) db: &'a dyn FeatureEvalQueryGroup,
    pub(crate) verbose: bool,
    pub(crate) opt_static_eval_feature: Option<&'a dyn EvalFeature<'static>>,
}

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    fn cache(
        &mut self,
        eval_key: EvalKey<'eval>,
        compute_value: impl FnOnce(&mut Self) -> EvalResult<'eval>,
    ) -> EvalResult<'eval> {
        if let Some(value) = self.sheet.cached_value(eval_key) {
            value
        } else {
            let value = compute_value(self);
            self.sheet.cache(eval_key, value)
        }
    }

    fn as_static(&self) -> FeatureEvaluator<'a, 'static> {
        self.opt_static_eval_feature
            .unwrap()
            .evaluator(self.sample_id)
    }
}
