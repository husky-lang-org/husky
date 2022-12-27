#![feature(trait_upcasting)]
#![feature(const_trait_impl)]
#![feature(const_convert)]
mod db;
mod error;
mod iter;
mod kind;
mod literal;
mod parse_utils;
mod sheet;
#[cfg(test)]
mod tests;
mod token_accessibility;
mod tokenize;
mod utils;

pub use db::*;
pub use error::*;
pub use iter::*;
pub use kind::*;
pub use literal::*;
pub use parse_utils::*;
pub use sheet::*;
pub use token_accessibility::*;

use husky_text::{HasTextRange, RangedIdentifier, TextRange};
use husky_vfs::{ModulePath, VfsResult};
use husky_word::Identifier;
#[cfg(test)]
use tests::*;
use tokenize::*;

#[salsa::jar(db = TokenDb)]
pub struct TokenJar(token_sheet, reserved_words);

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub range: TextRange,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(i: u32, start: u32, end: u32, kind: TokenKind) -> Token {
        Token {
            range: husky_text::new_same_line(i, start, end),
            kind,
        }
    }

    pub fn identify(&self) -> Option<Identifier> {
        match self.kind {
            TokenKind::Identifier(ident) => Some(ident),
            _ => None,
        }
    }
}

impl HasTextRange for Token {
    fn text_range(&self) -> TextRange {
        self.range
    }
}
