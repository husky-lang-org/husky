use super::*;
use crate::helpers::tracker::VdSemExprTracker;
use eterned::db::EternerDb;
use expect_test::{expect, Expect};
use latex_prelude::helper::tracker::LxFormulaInput;
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;
use visored_syn_expr::vibe::VdSynExprVibe;

pub(crate) fn t(content: &str, expected: &Expect) {
    use husky_path_utils::HuskyLangDevPaths;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()), db);
    let models = &VdModels::new();
    let tracker = VdSemExprTracker::new(
        LxFormulaInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        &[],
        &[],
        models,
        VdSynExprVibe::ROOT_CNL,
        db,
    );
    expected.assert_eq(&tracker.show_display_tree(db))
}

#[test]
fn basic_vd_sem_expr_works() {
    t(
        "1",
        &expect![[r#"
            "1" expr.literal
        "#]],
    );
    t(
        "+1",
        &expect![[r#"
            "+1" expr.prefix
            └─ "1" expr.literal
        "#]],
    );
    t(
        "-1",
        &expect![[r#"
            "-1" expr.prefix
            └─ "1" expr.literal
        "#]],
    );
    t(
        "1+1",
        &expect![[r#"
            "1+1" expr.folding_separated_list
            ├─ "1" expr.literal
            └─ "1" expr.literal
        "#]],
    );
    t(
        "1+1=2",
        &expect![[r#"
            "1+1=2" expr.chaining_separated_list
            ├─ "1+1" expr.folding_separated_list
            │ ├─ "1" expr.literal
            │ └─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
    t(
        "1<2",
        &expect![[r#"
            "1<2" expr.chaining_separated_list
            ├─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
}

#[test]
fn frac_vd_sem_expr_works() {
    t(
        "\\frac{1}{2}",
        &expect![[r#"
            "{2}" fraction
            ├─ "1" expr.literal
            └─ "2" expr.literal
        "#]],
    );
}

#[test]
fn sqrt_vd_sem_expr_works() {
    t(
        "\\sqrt{1}",
        &expect![[r#"
            "{1}" sqrt
            └─ "1" expr.literal
        "#]],
    );
}
