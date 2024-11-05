//! means the prose mode
pub mod helpers;
#[cfg(test)]
pub mod tests;

use super::*;
use helpers::LxRoseAstChild;
use husky_coword::Coword;
use latex_rose_punctuation::LxRosePunctuation;
use latex_token::{data::rose::LxRoseTokenData, idx::LxRoseTokenIdx};

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LxRoseAstData {
    TextEdit { buffer: String },
    Word(LxRoseTokenIdx, Coword),
    Punctuation(LxRoseTokenIdx, LxRosePunctuation),
}

pub type LxRoseAstArena = Arena<LxRoseAstData>;
pub type LxRoseAstArenaRef<'a> = ArenaRef<'a, LxRoseAstData>;
pub type LxRoseAstArenaMap<T> = ArenaMap<LxRoseAstData, T>;
pub type LxRoseAstIdx = ArenaIdx<LxRoseAstData>;
pub type LxRoseAstIdxRange = ArenaIdxRange<LxRoseAstData>;

impl LxRoseAstData {
    pub fn children(&self) -> Vec<LxRoseAstChild> {
        match self {
            LxRoseAstData::TextEdit { .. } => vec![],
            LxRoseAstData::Word(_, _) => vec![],
            LxRoseAstData::Punctuation(_, _) => vec![],
        }
    }
}

impl<'a> LxAstParser<'a> {
    pub(crate) fn parse_rose_asts(&mut self) -> LxRoseAstIdxRange {
        let mut asts = vec![];
        while let Some(ast) = self.parse_rose_ast() {
            asts.push(ast)
        }
        self.alloc_rose_asts(asts)
    }

    fn parse_rose_ast(&mut self) -> Option<LxRoseAstData> {
        let (token_idx, token) = self.next_rose_token()?;
        match token {
            LxRoseTokenData::Word(coword) => Some(LxRoseAstData::Word(token_idx, coword)),
            LxRoseTokenData::Command(_) => todo!(),
            LxRoseTokenData::Dollar => todo!(),
            LxRoseTokenData::EscapedLpar => todo!(),
            LxRoseTokenData::EscapedLbox => todo!(),
            LxRoseTokenData::Nat32(_) => todo!(),
            LxRoseTokenData::NewParagraph => todo!(),
            LxRoseTokenData::Punctuation(lx_rose_punctuation) => {
                Some(LxRoseAstData::Punctuation(token_idx, lx_rose_punctuation))
            }
        }
    }
}