mod impl_necessary;
mod impl_train;
mod query;
mod utils;
mod variant;

pub use husky_compile_time::*;
pub use husky_feature_gen::{
    AllocateUniqueFeature, FeatureGenQueryGroup, FeatureGenQueryGroupStorage,
};
pub use husky_instruction_gen::InstructionGenQueryGroup;
pub use query::*;
pub use utils::*;

use husky_check_utils::*;
use husky_compile_time::*;
use husky_datasets_interface::LabeledData;
use husky_feature_eval::*;
use husky_feature_eval::{EvalFeature, Session};
use husky_file::{FilePtr, FileQueryGroup};
use husky_print_utils::*;
use husky_text::TextQueryGroupStorage;
use husky_trace_protocol::*;
use indexmap::IndexMap;
use json_map::JsonListMap;
use json_result::JsonResult;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, Mutex},
};
use variant::*;
use vm::{Instruction, VMConfig};

#[salsa::database(
    husky_feature_gen::FeatureGenQueryGroupStorage,
    husky_instruction_gen::InstructionGenQueryGroupStorage,
    husky_data_viewer::HuskyDataViewerQueryGroupStorage
)]
pub struct HuskyRuntime {
    storage: salsa::Storage<HuskyRuntime>,
    compile_time: HuskyComptime,
    compile_time_version: usize,
    feature_interner: husky_feature_gen::FeatureInterner,
    variant: HuskyRuntimeVariant,
    package_main: FilePtr,
    config: HuskyRuntimeConfig,
}

#[derive(Debug)]
pub struct HuskyRuntimeConfig {
    pub evaluator: EvaluatorConfig,
    pub compile_time: HuskyCompileTimeConfig,
}

impl HuskyRuntime {
    pub fn new(
        init_compile_time: impl FnOnce(&mut HuskyComptime),
        config: HuskyRuntimeConfig,
    ) -> HuskyRuntimeSingletonKeeper {
        let mut compile_time = HuskyComptime::new(config.compile_time.clone());
        init_compile_time(&mut compile_time);
        let all_main_files = compile_time.all_main_files();
        should_eq!(all_main_files.len(), 1, "config = {config:?}");
        let package_main = all_main_files[0];
        let feature_interner = husky_feature_gen::new_feature_interner();
        let mut eval_time = Self {
            storage: Default::default(),
            variant: HuskyRuntimeVariant::None,
            compile_time,
            compile_time_version: 0,
            package_main,
            config,
            feature_interner,
        };
        eval_time.init();
        eval_time.into()
    }

    fn init(&mut self) {
        let compile_time = &self.compile_time;
        let all_diagnostics = compile_time.all_diagnostics();
        if all_diagnostics.len() > 0 {
            p!(all_diagnostics);
            panic!("diagnostic errors")
        }
        let package = match compile_time.package(self.package_main) {
            Ok(package) => package,
            Err(error) => {
                compile_time.print_diagnostics();
                p!(error);
                panic!()
            }
        };
        self.variant = HuskyRuntimeVariant::Learning {
            session: Session::new(&package, self, &self.evaluator_config().vm).unwrap(),
        }
    }
}
