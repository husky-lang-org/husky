#![feature(trait_upcasting)]
mod db;
mod entity_path;
mod error;
mod parser;
mod pattern;
mod precedence;
mod sheet;
mod snippet;
mod stmt;
mod symbol;
#[cfg(test)]
mod tests;
mod variable;

pub use db::*;
pub use entity_path::*;
pub use error::*;
pub use parser::*;
pub use pattern::*;
pub use sheet::*;
pub use stmt::*;
pub use variable::*;

use husky_entity_path::EntityPath;
use husky_opn_syntax::*;
use husky_text::*;
use husky_token::*;
use husky_word::*;
use precedence::*;
use snippet::*;

#[salsa::jar(db = ExprDb)]
pub struct ExprJar(ExprSheet, parse_expr_from_snippet);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPath {
    None,
    Some(EntityPath),
    Uncertain {
        inclination: BaseEntityPathInclination,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BaseEntityPathInclination {
    GlobalValue,
    FunctionOrLocalValue,
    TypeOrVariant,
}

impl BaseEntityPathInclination {
    pub fn from_case(case: IdentifierCase) -> Self {
        match case {
            IdentifierCase::SingleCapital | IdentifierCase::PascalCase => {
                BaseEntityPathInclination::TypeOrVariant
            }
            IdentifierCase::AllCapital => BaseEntityPathInclination::GlobalValue,
            IdentifierCase::SnakeCase => BaseEntityPathInclination::FunctionOrLocalValue,
            _ => todo!(),
        }
    }
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
        punctuation: BinaryOpr,
        punctuation_token_idx: TokenIdx,
        ropd: ExprIdx,
    },
    Be {
        src: ExprIdx,
        be_token_idx: TokenIdx,
        target: ExprResult<BePattern>,
    },
    PrefixOpn {
        punctuation: PrefixOpr,
        punctuation_token_idx: TokenIdx,
        opd: ExprIdx,
    },
    SuffixOpn {
        opd: ExprIdx,
        punctuation: SuffixOpr,
        punctuation_token_idx: TokenIdx,
    },
    FunctionCall {
        function: ExprIdx,
        implicit_arguments: Option<ImplicitArgumentList>,
        lpar_token_idx: TokenIdx,
        arguments: ExprIdxRange,
        rpar_token_idx: TokenIdx,
    },
    Field {
        this_expr: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentifierToken,
    },
    MethodCall {
        this_expr: ExprIdx,
        dot_token_idx: TokenIdx,
        ident_token: IdentifierToken,
        implicit_arguments: Option<ImplicitArgumentList>,
        lpar_token_idx: TokenIdx,
        arguments: ExprIdxRange,
        rpar_token_idx: TokenIdx,
    },
    TemplateInstantiation {
        template: ExprIdx,
        implicit_arguments: ImplicitArgumentList,
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
    Block {
        stmts: StmtIdxRange,
    },
    Err(ExprError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct ImplicitArgumentList {
    langle: TokenIdx,
    arguments: ExprIdxRange,
    rangle: TokenIdx,
}

impl ImplicitArgumentList {
    pub(crate) fn new(langle: TokenIdx, arguments: ExprIdxRange, rangle: TokenIdx) -> Self {
        Self {
            langle,
            arguments,
            rangle,
        }
    }

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

pub type ExprArena = Arena<Expr>;
pub type ExprIdx = ArenaIdx<Expr>;
pub type ExprIdxRange = ArenaIdxRange<Expr>;
pub type ExprMap<V> = ArenaMap<Expr, V>;
