use super::*;
use crate::helpers::tracker::VdSynExprTracker;
use eterned::db::EternerDb;
use expect_test::{expect, Expect};
use latex_prelude::{helper::tracker::LxPageInput, mode::LxMode};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;
use visored_annotation::annotation::{space::VdSpaceAnnotation, token::VdTokenAnnotation};

fn t(
    models: &VdModels,
    content: &str,
    token_annotations: &[((&str, &str), VdTokenAnnotation)],
    space_annotations: &[((&str, &str), VdSpaceAnnotation)],
    expected: &Expect,
) {
    use crate::helpers::show::display_tree::VdSynExprDisplayTreeBuilder;
    use husky_path_utils::HuskyLangDevPaths;

    let db = &EternerDb::default();
    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()), db);
    let vibe = VdSynExprVibe::ROOT_CNL;
    let tracker = VdSynExprTracker::new(
        LxPageInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        token_annotations,
        space_annotations,
        models,
        vibe,
        db,
    );
    expected.assert_eq(&tracker.show_display_tree(db));
}

#[test]
fn let_clause_parsing_works() {
    let models = &VdModels::new();
    t(
        models,
        "Let $x = 1$.",
        &[],
        &[],
        &expect![[r#"
            Let $x = 1$.
            └─ "Let $x = 1$." stmt.paragraph
              └─ "Let $x = 1$." sentence.clauses
                └─ "Let $x = 1$" clause.let
                  └─ "x = 1" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "1" expr.literal
        "#]],
    );
    t(
        models,
        "Let $x \\in \\mathbb{N}$.",
        &[],
        &[],
        &expect![[r#"
            Let $x \in \mathbb{N}$.
            └─ "Let $x \\in \\mathbb{N}$." stmt.paragraph
              └─ "Let $x \\in \\mathbb{N}$." sentence.clauses
                └─ "Let $x \\in \\mathbb{N}$" clause.let
                  └─ "x \\in \\mathbb{N}" expr.separated_list
                    ├─ "x" expr.letter
                    └─ "\\mathbb{N}" expr.letter
        "#]],
    );
}
