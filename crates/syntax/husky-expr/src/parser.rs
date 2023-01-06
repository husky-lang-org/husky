mod accept;
mod alloc;
mod block;
mod env;
mod expr_stack;
mod list;
mod resolve;
mod unfinished_expr;

pub use block::*;
pub use env::*;

use crate::*;
use expr_stack::*;
use husky_ast::{Ast, AstIdxRange, AstSheet};
use husky_entity_tree::{CratePrelude, EntityTreeDb};
use husky_token::Token;
use husky_token::TokenStream;
use list::*;
use parsec::ParseContext;
use resolve::*;
use salsa::DebugWithDb;
use std::ops::ControlFlow;
use symbol::*;
use unfinished_expr::*;

#[macro_use]
macro_rules! report {
    ($self: expr) => {{
        p!(
            $self.stack,
            $self.parser.entity_path.debug($self.db()) // $self.token_stream.text_range()
        );
    }};
}
use report;

pub struct ExprParser<'a> {
    db: &'a dyn ExprDb,
    entity_path: Option<EntityPath>,
    token_sheet_data: &'a TokenSheetData,
    symbol_sheet: SymbolSheet<'a>,
    expr_arena: ExprArena,
    entity_path_expr_arena: EntityPathExprArena,
    pattern_expr_sheet: PatternExprSheet,
    stmt_arena: StmtArena,
}

impl<'a> ExprParser<'a> {
    pub fn new(
        db: &'a dyn ExprDb,
        entity_path: Option<EntityPath>,
        token_sheet_data: &'a TokenSheetData,
        crate_prelude: CratePrelude<'a>,
    ) -> Self {
        Self {
            db,
            entity_path,
            token_sheet_data,
            symbol_sheet: SymbolSheet::new(crate_prelude),
            expr_arena: Default::default(),
            entity_path_expr_arena: Default::default(),
            pattern_expr_sheet: Default::default(),
            stmt_arena: Default::default(),
        }
    }

    pub fn finish(self) -> ExprSheet {
        ExprSheet::new(
            self.db,
            self.expr_arena,
            self.entity_path_expr_arena,
            self.pattern_expr_sheet,
            self.stmt_arena,
            self.symbol_sheet.variable_sheet(),
        )
    }

    pub fn ctx<'b>(&'b mut self, token_stream: TokenStream<'a>) -> ExprParseContext<'a, 'b>
    where
        'a: 'b,
    {
        ExprParseContext::new(self, token_stream)
    }

    pub(crate) fn pattern_expr_sheet(&self) -> &PatternExprSheet {
        &self.pattern_expr_sheet
    }

    #[inline(always)]
    fn define_variables(&mut self, variables: Vec<Variable>) -> VariableIdxRange {
        self.symbol_sheet.define_variables(variables)
    }
}

pub struct ExprParseContext<'a, 'b> {
    parser: &'b mut ExprParser<'a>,
    env: ExprParseEnvironmentPlace,
    token_stream: TokenStream<'a>,
    stack: ExprStack,
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    fn new(parser: &'b mut ExprParser<'a>, token_stream: TokenStream<'a>) -> Self {
        Self {
            parser,
            env: Default::default(),
            token_stream,
            stack: Default::default(),
        }
    }

    pub(crate) fn db(&self) -> &'a dyn EntityTreeDb {
        self.parser.db
    }

    pub(crate) fn tokens(&self) -> &TokenStream<'a> {
        &self.token_stream
    }

    pub fn parse_expr(&mut self, env: ExprParseEnvironment) -> Option<ExprIdx> {
        self.env.set(env);
        loop {
            let Some((token_idx, token)) = self.token_stream.next_indexed()
                else {
                    break
                };
            match self.resolve_token(token_idx, token) {
                ControlFlow::Continue(resolved_token) => self.accept_token(resolved_token),
                ControlFlow::Break(_) => {
                    self.rollback(token_idx);
                    break;
                }
            }
        }
        self.reduce(Precedence::None);
        self.env.unset();
        self.finish_batch()
    }

    pub(crate) fn pattern_expr_sheet(&self) -> &PatternExprSheet {
        self.parser.pattern_expr_sheet()
    }

    pub(crate) fn define_variables(&mut self, variables: Vec<Variable>) -> VariableIdxRange {
        self.parser.define_variables(variables)
    }

    pub(crate) fn parse_pattern_expr(
        &mut self,
        env: PatternInfo,
    ) -> ExprResult<Option<PatternExprIdx>> {
        if let Some(mut_token) = self.parse::<MutToken>()? {
            let ident_token = self.parse_expected::<IdentifierToken>()?;
            Ok(Some(self.alloc_pattern_expr(
                PatternExpr::Identifier {
                    ident_token,
                    liason: PatternLiason::None,
                },
                env,
            )))
        } else if let Some(ident_token) = self.parse::<IdentifierToken>()? {
            Ok(Some(self.alloc_pattern_expr(
                PatternExpr::Identifier {
                    ident_token,
                    liason: PatternLiason::None,
                },
                env,
            )))
        } else {
            Ok(None)
        }
    }
}

pub fn parse_expr<'a>(
    db: &'a dyn ExprDb,
    entity_path: Option<EntityPath>,
    crate_prelude: CratePrelude<'a>,
    token_sheet_data: &'a TokenSheetData,
    token_iter: TokenStream<'a>,
    env: ExprParseEnvironment,
) -> (ExprSheet, Option<ExprIdx>) {
    let mut expr_parser = ExprParser::new(db, entity_path, token_sheet_data, crate_prelude);
    let expr = expr_parser.ctx(token_iter).parse_expr(env);
    (expr_parser.finish(), expr)
}

impl<'a, 'b> parsec::HasParseError for ExprParseContext<'a, 'b> {
    type Error = ExprError;
}

impl<'a, 'b> std::ops::Deref for ExprParseContext<'a, 'b> {
    type Target = TokenStream<'a>;
    fn deref(&self) -> &Self::Target {
        &self.token_stream
    }
}

impl<'a, 'b> std::ops::DerefMut for ExprParseContext<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.token_stream
    }
}

impl<'a, 'b> std::borrow::Borrow<TokenStream<'a>> for ExprParseContext<'a, 'b> {
    fn borrow(&self) -> &TokenStream<'a> {
        &self.token_stream
    }
}

impl<'a, 'b> std::borrow::BorrowMut<TokenStream<'a>> for ExprParseContext<'a, 'b> {
    fn borrow_mut(&mut self) -> &mut TokenStream<'a> {
        &mut self.token_stream
    }
}

impl<'a, 'b, 'c> parsec::StreamWrapper for ExprParseContext<'a, 'b> {}
