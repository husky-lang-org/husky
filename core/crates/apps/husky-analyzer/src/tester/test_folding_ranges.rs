use husky_compile_test::test_all_source_files;
use husky_compile_time::{utils::*, *};
use husky_display_utils::HuskyDisplay;
use husky_test_utils::*;
use std::path::Path;

pub(super) fn test_folding_ranges(package_dir: &Path) -> TestResult {
    test_all_source_files(package_dir, "folding_ranges.txt", |compile_time, file| {
        compile_time
            .tokenized_text(file)
            .unwrap()
            .folding_ranges()
            .into_iter()
            .map(|st| FoldingRangeWrapper(st))
            .collect::<Vec<_>>()
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct FoldingRangeWrapper(lsp_types::FoldingRange);

impl HuskyDisplay for FoldingRangeWrapper {
    fn write_inherent(&self, config: husky_display_utils::HuskyDisplayConfig, result: &mut String) {
        todo!()
    }
}
