use super::*;
use crate::helpers::tracker::VdLeanTranspilationTracker;
use latex_prelude::{
    helper::tracker::{LxDocumentBodyInput, LxDocumentInput, LxPageInput},
    mode::LxMode,
};
use latex_vfs::path::LxFilePath;
use std::path::PathBuf;

fn t(content: &str, expected_display_tree: &Expect, expected_fmt: &Expect) {
    use husky_path_utils::HuskyLangDevPaths;

    let dev_paths = HuskyLangDevPaths::new();
    let file_path = LxFilePath::new(PathBuf::from(file!()));
    let tracker = VdLeanTranspilationTracker::new(
        LxDocumentInput {
            specs_dir: dev_paths.specs_dir(),
            file_path,
            content,
        },
        &[],
        &[],
    );
    expected_display_tree.assert_eq(&tracker.show_display_tree());
    expected_fmt.assert_eq(&tracker.show_fmt());
}

#[test]
fn basic_document_to_vd_mir_works() {
    t(
        r#"\documentclass{article}
\usepackage{amsmath}
\begin{document}
Let $x\in\mathbb{R}$.
\end{document}"#,
        &expect![[r#"
            └─ group: `division`
              └─ group: `paragraph`
                └─ group: `sentence`
                  └─ variable: `x`
        "#]],
        &expect![[r#"
            -- Let $x\in\mathbb{R}$
            variable (x : ℝ)"#]],
    );
    t(
        r#"\documentclass{article}
\usepackage{amsmath}
\begin{document}
\section{Introduction}
Let $x\in\mathbb{R}$.
\end{document}"#,
        &expect![[r#"
            └─ group: `division`
              └─ group: `division`
                └─ group: `paragraph`
                  └─ group: `sentence`
                    └─ variable: `x`
        "#]],
        &expect![[r#"
            namespace Section1
            -- Let $x\in\mathbb{R}$
            variable (x : ℝ)
            end Section1
        "#]],
    );
    t(
        r#"\documentclass{article}
\usepackage{amsmath}
\begin{document}
\section{Introduction}
Let $x\in\mathbb{R}$.
\subsection{Hello}
Let $y\in\mathbb{R}$.
\subsection{World}
\subsection{This}
\subsubsection{Is}
\subsubsection{Bad}
\end{document}"#,
        &expect![[r#"
            └─ group: `division`
              ├─ group: `division`
              │ └─ group: `paragraph`
              │   └─ group: `sentence`
              │     └─ variable: `x`
              ├─ group: `division`
              │ └─ group: `division`
              │   └─ group: `paragraph`
              │     └─ group: `sentence`
              │       └─ variable: `y`
              ├─ group: `division`
              └─ group: `division`
                ├─ group: `division`
                └─ group: `division`
        "#]],
        &expect![[r#"
            namespace Section1
            -- Let $x\in\mathbb{R}$
            variable (x : ℝ)

            namespace Subsection1
            -- Let $y\in\mathbb{R}$
            variable (y : ℝ)
            end Subsection1

            namespace Subsection2
            end Subsection2

            namespace Subsection3
            namespace Subsubsection1
            end Subsubsection1

            namespace Subsubsection2
            end Subsubsection2
            end Subsection3
            end Section1
        "#]],
    );
}

#[test]
fn latex_shorts_to_lean_works() {
    use expect_test::expect_file;
    use husky_path_utils::HuskyLangDevPaths;
    use std::fs;

    let dev_paths = HuskyLangDevPaths::new();
    let projects_dir = dev_paths.projects_dir();

    for file in fs::read_dir(projects_dir.join("ai-math-autoformalization/latex/shorts")).unwrap() {
        let file = file.unwrap();
        let file_path = file.path();
        if file_path.extension() != Some(&std::ffi::OsStr::new("tex")) {
            continue;
        }
        let content = &fs::read_to_string(&file_path).unwrap();
        let filestem = file_path.file_stem().unwrap().to_str().unwrap();
        let file_path = LxFilePath::new(file_path.clone());
        let tracker = VdLeanTranspilationTracker::new(
            LxDocumentInput {
                specs_dir: dev_paths.specs_dir(),
                file_path,
                content,
            },
            &[],
            &[],
        );
        expect_file![projects_dir.join(format!(
            "ai-math-autoformalization/lean/central-46/Central46/Shorts/{}.lean",
            filestem
        ))]
        .assert_eq(&format!(
            r#"import Mathlib.Tactic.Linarith
import Mathlib.Tactic.Ring
import Mathlib.Data.Real.Basic
import Mathlib.Tactic.Explode

{}"#,
            tracker.show_fmt()
        ));
    }
}
