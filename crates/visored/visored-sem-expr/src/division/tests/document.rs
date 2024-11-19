use super::*;
use latex_prelude::helper::tracker::LxDocumentInput;

fn t(content: &str, expected: &Expect) {
    use crate::helpers::show::display_tree::VdSemExprDisplayTreeBuilder;

    let db = &DB::default();
    let file_path = LxFilePath::new(db, PathBuf::from(file!()));
    let tracker = VdSemExprTracker::new(LxDocumentInput { file_path, content }, &[], &[], db);
    expected.assert_eq(&tracker.show_display_tree(db));
}

#[test]
fn parse_document_to_vd_sem_works() {
    t(
        r#"\documentclass{article}
\usepackage{amsmath}
\begin{document}
Let $x\in\mathbb{R}$.
\end{document}"#,
        &expect![[r#"
            └─ "Let $x\\in\\mathbb{R}$." division.stmts
              └─ "Let $x\\in\\mathbb{R}$." stmt.paragraph
                └─ "Let $x\\in\\mathbb{R}$." sentence.clauses
                  └─ "Let $x\\in\\mathbb{R}$" clause.let
                    └─ "x\\in\\mathbb{R}" expr.separated_list
                      ├─ "x" expr.letter
                      └─ "\\mathbb{R}" expr.letter
        "#]],
    );
    t(
        r#"\documentclass{article}
\usepackage{amsmath}
\begin{document}
\section{Introduction}Let $x\in\mathbb{R}$.
\end{document}"#,
        &expect![[r#"
            └─ "\\section{Introduction}Let $x\\in\\mathbb{R}$." division.divisions
              ├─ title
              │ └─ "Introduction" stmt.paragraph
              │   └─ "Introduction" sentence.clauses
              │     └─ "Introduction" clause.todo
              └─ "Let $x\\in\\mathbb{R}$." division.stmts
                └─ "Let $x\\in\\mathbb{R}$." stmt.paragraph
                  └─ "Let $x\\in\\mathbb{R}$." sentence.clauses
                    └─ "Let $x\\in\\mathbb{R}$" clause.let
                      └─ "x\\in\\mathbb{R}" expr.separated_list
                        ├─ "x" expr.letter
                        └─ "\\mathbb{R}" expr.letter
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
            └─ "\\section{Introduction}\nLet $x\\in\\mathbb{R}$.\n\\subsection{Hello}\nLet $y\\in\\mathbb{R}$.\n\\subsection{World}\n\\subsection{This}\n\\subsubsection{Is}\n\\subsubsection{Bad}" division.divisions
              ├─ title
              │ └─ "Introduction" stmt.paragraph
              │   └─ "Introduction" sentence.clauses
              │     └─ "Introduction" clause.todo
              ├─ "Let $x\\in\\mathbb{R}$." division.stmts
              │ └─ "Let $x\\in\\mathbb{R}$." stmt.paragraph
              │   └─ "Let $x\\in\\mathbb{R}$." sentence.clauses
              │     └─ "Let $x\\in\\mathbb{R}$" clause.let
              │       └─ "x\\in\\mathbb{R}" expr.separated_list
              │         ├─ "x" expr.letter
              │         └─ "\\mathbb{R}" expr.letter
              ├─ "\\subsection{Hello}\nLet $y\\in\\mathbb{R}$." division.divisions
              │ ├─ title
              │ │ └─ "Hello" stmt.paragraph
              │ │   └─ "Hello" sentence.clauses
              │ │     └─ "Hello" clause.todo
              │ └─ "Let $y\\in\\mathbb{R}$." division.stmts
              │   └─ "Let $y\\in\\mathbb{R}$." stmt.paragraph
              │     └─ "Let $y\\in\\mathbb{R}$." sentence.clauses
              │       └─ "Let $y\\in\\mathbb{R}$" clause.let
              │         └─ "y\\in\\mathbb{R}" expr.separated_list
              │           ├─ "y" expr.letter
              │           └─ "\\mathbb{R}" expr.letter
              ├─ "\\subsection{World}" division.divisions
              │ └─ title
              │   └─ "World" stmt.paragraph
              │     └─ "World" sentence.clauses
              │       └─ "World" clause.todo
              └─ "\\subsection{This}\n\\subsubsection{Is}\n\\subsubsection{Bad}" division.divisions
                ├─ title
                │ └─ "This" stmt.paragraph
                │   └─ "This" sentence.clauses
                │     └─ "This" clause.todo
                ├─ "\\subsubsection{Is}" division.divisions
                │ └─ title
                │   └─ "Is" stmt.paragraph
                │     └─ "Is" sentence.clauses
                │       └─ "Is" clause.todo
                └─ "\\subsubsection{Bad}" division.divisions
                  └─ title
                    └─ "Bad" stmt.paragraph
                      └─ "Bad" sentence.clauses
                        └─ "Bad" clause.todo
        "#]],
    );
}
