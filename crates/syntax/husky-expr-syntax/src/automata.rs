mod accept;
mod opr;
mod resolve;
mod stack;
mod synthesize;

use crate::*;
use husky_check_utils::should;
use husky_symbol_syntax::SymbolContext;
use husky_token::{Token, TokenKind};
use husky_token_stream::TokenStream;
use opr::*;
use resolve::*;
use stack::*;

pub(crate) struct Automata<'a> {
    ctx: &'a mut SymbolContext<'a>,
    stream: TokenStream<'a>,
    stack: AutomataStack,
    arena: &'a mut ExprArena,
}

impl<'a> Automata<'a> {
    pub(crate) fn stream(&self) -> &TokenStream<'a> {
        &self.stream
    }

    fn new(ctx: &'a mut SymbolContext<'a>, tokens: &'a [Token], arena: &'a mut ExprArena) -> Self {
        Self {
            ctx,
            stream: tokens.into(),
            stack: Default::default(),
            arena,
        }
    }

    fn parse_all(mut self) -> ExprIdx {
        while !self.stream().is_empty() {
            let token = &self.stream.next().unwrap();
            match self.accept_token(self.resolve_token(token)) {
                Ok(()) => (),
                Err(_) => todo!(),
            }
        }
        self.synthesize_all_above(Precedence::None).expect("todo");
        should!(self.stack.number_of_exprs() == 1);
        self.arena.alloc_one(self.stack.pop_expr().unwrap())
    }
}

pub fn parse_raw_expr<'a>(
    ctx: &'a mut SymbolContext<'a>,
    arena: &'a mut ExprArena,
    tokens: &'a [Token],
) -> ExprIdx {
    Automata::new(ctx, tokens, arena).parse_all()
}
