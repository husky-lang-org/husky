#![feature(trait_upcasting)]
mod atom;
mod entity_path;
mod error;
mod parser;
mod pattern;
mod precedence;
mod sheet;
mod stmt;
#[cfg(test)]
mod tests;
mod variable;

pub use atom::*;
pub use entity_path::*;
pub use error::*;
pub use parser::*;
pub use pattern::*;
pub use sheet::*;
pub use variable::*;

use husky_entity_path::EntityPath;
use husky_opn_syntax::*;
use husky_symbol::VariableIdx;
use husky_text::*;
use husky_token::*;
use husky_word::*;
use precedence::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPath {
    None,
    Some(EntityPath),
    Uncertain,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Literal(TokenIdx),
    EntityPath(EntityPathExprIdx),
    Variable {
        token_idx: TokenIdx,
        variable_idx: VariableIdx,
    },
    Uncertain(Identifier),
    Unrecognized(Identifier),
    BinaryOpn {
        lopd: ExprIdx,
        punctuation: BinaryPunctuation,
        punctuation_token_idx: TokenIdx,
        ropd: ExprIdx,
    },
    PrefixOpn {
        punctuation: PrefixPunctuation,
        punctuation_token_idx: TokenIdx,
        opd: ExprIdx,
    },
    SuffixOpn {
        opd: ExprIdx,
        punctuation: SuffixPunctuation,
        punctuation_token_idx: TokenIdx,
    },
    Field {
        this_expr: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentifierToken,
    },
    MethodCall {
        this_expr: ExprIdx,
        implicit_arguments: Option<ImplicitArgumentList>,
        arguments: ExprIdxRange,
        lpar_token_idx: TokenIdx,
        rpar_token_idx: TokenIdx,
    },
    Application {
        function: ExprIdx,
        argument: ExprIdx,
    },
    NewTuple {
        lpar_token_idx: TokenIdx,
        items: ExprIdxRange,
        rpar_token_idx: TokenIdx,
    },
    NewList {
        lbox_token_idx: TokenIdx,
        items: ExprIdxRange,
        rbox_token_idx: TokenIdx,
    },
    Bracketed(ExprIdx),
    Err(ExprError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitArgumentList {
    langle: TokenIdx,
    arguments: ExprIdxRange,
    rangle: TokenIdx,
}

impl ImplicitArgumentList {
    pub fn langle(&self) -> TokenIdx {
        self.langle
    }

    pub fn arguments(&self) -> &ExprIdxRange {
        &self.arguments
    }

    pub fn rangle(&self) -> TokenIdx {
        self.rangle
    }
}

use idx_arena::{map::ArenaMap, Arena, ArenaIdx, ArenaIdxRange};

pub(crate) type ExprArena = Arena<Expr>;
pub type ExprIdx = ArenaIdx<Expr>;
pub type ExprIdxRange = ArenaIdxRange<Expr>;
pub type ExprMap<V> = ArenaMap<Expr, V>;
