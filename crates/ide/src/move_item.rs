use std::{iter::once, mem};

use hir::Semantics;
use ide_db::{base_db::FileRange, helpers::pick_best_token, RootDatabase};
use itertools::Itertools;
use syntax::{ast, SyntaxElement, SyntaxKind, SyntaxNode, TextRange};
use text_edit::{TextEdit, TextEditBuilder};

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
}

// Feature: Move Item
//
// Move item under cursor or selection up and down.
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Move item up**
// | VS Code | **Rust Analyzer: Move item down**
// |===
//
// image::https://user-images.githubusercontent.com/48062697/113065576-04298180-91b1-11eb-91ce-4505e99ed598.gif[]
pub(crate) fn move_item(
    db: &RootDatabase,
    range: FileRange,
    direction: Direction,
) -> Option<TextEdit> {
    todo!()
}

fn find_ancestors(item: SyntaxElement, direction: Direction, range: TextRange) -> Option<TextEdit> {
    todo!()
}

fn move_in_direction(
    node: &SyntaxNode,
    direction: Direction,
    range: TextRange,
) -> Option<TextEdit> {
    todo!()
}

fn replace_nodes<'a>(
    range: TextRange,
    mut first: &'a SyntaxNode,
    mut second: &'a SyntaxNode,
) -> TextEdit {
    todo!()
}
