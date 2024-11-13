use super::*;
use latex_prelude::mode::LxMode;

fn t(input: &str, expected_display_tree: &Expect, expected_fmt: &Expect) {
    let db = &DB::default();
    let example = VdLeanTranspilationExample::new(input, LxMode::Rose, &[], &[], db);
    expected_display_tree.assert_eq(&example.show_display_tree(db));
    expected_fmt.assert_eq(&example.show_fmt(db));
}

#[test]
fn basic_visored_clause_to_lean_works() {
    t(
        "Let $x\\in\\mathbb{N}$.",
        &expect![[r#"
            defns
            └─ group: `paragraph`
              └─ group: `sentence`
                └─ variable: `x`
        "#]],
        &expect!["variable x : ℕ"],
    );
    t(
        "Let $x\\in\\mathbb{Z}$.",
        &expect![[r#"
            defns
            └─ group: `paragraph`
              └─ group: `sentence`
                └─ variable: `x`
        "#]],
        &expect!["variable x : ℤ"],
    );
    t(
        "Let $x\\in\\mathbb{Q}$.",
        &expect![[r#"
            defns
            └─ group: `paragraph`
              └─ group: `sentence`
                └─ variable: `x`
        "#]],
        &expect!["variable x : ℚ"],
    );
    t(
        "Let $x\\in\\mathbb{R}$.",
        &expect![[r#"
            defns
            └─ group: `paragraph`
              └─ group: `sentence`
                └─ variable: `x`
        "#]],
        &expect!["variable x : ℝ"],
    );
    t(
        "Let $x\\in\\mathbb{C}$.",
        &expect![[r#"
            defns
            └─ group: `paragraph`
              └─ group: `sentence`
                └─ variable: `x`
        "#]],
        &expect!["variable x : ℂ"],
    );
}
