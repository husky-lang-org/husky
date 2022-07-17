mod config;
mod impl_arrival;
mod impl_block;
mod impl_cached;
mod impl_eval_context;
mod impl_expr;
mod impl_repr;
mod impl_stmt;
mod impl_visualize;
mod indicator;
mod sheet;

pub use config::*;
pub use indicator::FeatureEvalIndicator;
pub use sheet::*;

use crate::*;
use husky_feature_gen::FeatureEvalId;
use husky_trace_protocol::SampleId;
use vm::{VMConfig, __AnyValueDyn, __EvalContext, __EvalValue};
use vm::{__EvalResult, __EvalValueResult};

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    pub(crate) sample_id: SampleId,
    pub(crate) eval_input: __EvalValue<'eval>,
    pub(crate) sheet: &'a EvalSheet<'eval>,
    pub(crate) db: &'a dyn FeatureGenQueryGroup,
    pub(crate) evaluator_config: &'a EvaluatorConfig,
    pub(crate) opt_static_husky_feature_eval: Option<&'a dyn EvalFeature<'static>>,
}

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub unsafe fn some_ctx(&self) -> Option<&'a __EvalContext<'eval>> {
        let ptr: *const Self = self;
        let ptr: *const __EvalContext<'eval> = ptr as *const __EvalContext<'eval>;
        Some(&*ptr)
    }

    fn vm_config(&self) -> &'a VMConfig {
        &self.evaluator_config.vm
    }

    fn cache(
        &mut self,
        eval_key: EvalKey<'eval>,
        compute_value: impl FnOnce(&mut Self) -> __EvalValueResult<'eval>,
    ) -> __EvalValueResult<'eval> {
        if let Some(value) = self.sheet.cached_value(eval_key) {
            value
        } else {
            let value = compute_value(self);
            self.sheet.cache(eval_key, value)
        }
    }

    fn as_static(&self) -> FeatureEvaluator<'a, 'static> {
        self.opt_static_husky_feature_eval
            .unwrap()
            .evaluator(self.sample_id)
    }
}
