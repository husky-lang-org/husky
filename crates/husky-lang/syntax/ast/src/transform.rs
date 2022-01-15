mod impl_parse_expr;
mod impl_parse_func_decl;
mod impl_parse_stmt;
mod impl_symbol_proxy;
mod utils;

use fold::{FoldedList, LocalStack, LocalValue};
use syntax_types::*;
use text::TextRanged;
use token::*;
use word::*;

use crate::{
    atom::symbol_proxy::Symbol,
    query::{AstQueryGroup, AstText},
    transform::utils::*,
    *,
};

pub struct AstTransformer<'a> {
    db: &'a dyn AstQueryGroup,
    arena: RawExprArena,
    folded_results: FoldedList<AstResult<Ast>>,
    symbols: LocalStack<Symbol>,
    env: LocalValue<syntax_types::Env>,
}

impl<'a> AstTransformer<'a> {
    pub(crate) fn new(db: &'a dyn AstQueryGroup, module: scope::PackageOrModule) -> Self {
        Self {
            db,
            arena: RawExprArena::new(),
            folded_results: FoldedList::new(),
            symbols: LocalStack::new(),
            env: LocalValue::new(match db.id_to_scope(module.scope_id()).route {
                scope::ScopeRoute::Builtin(_) => todo!(),
                scope::ScopeRoute::Package(_, _) => Env::Package,
                scope::ScopeRoute::ChildScope(_, _) => Env::Module,
            }),
        }
    }

    pub(crate) fn finish(self) -> AstText {
        AstText {
            arena: self.arena,
            folded_results: self.folded_results,
        }
    }

    fn env(&self) -> Env {
        self.env.value()
    }
}

impl<'a> fold::Transformer<[Token], TokenizedText, AstResult<Ast>> for AstTransformer<'a> {
    fn _enter_block(&mut self) {
        self.env.enter();
        self.symbols.enter();
    }

    fn _exit_block(&mut self) {
        self.env.exit();
        self.symbols.exit();
    }

    fn transform(
        &mut self,
        _indent: fold::Indent,
        tokens: &[Token],
        enter_block: &mut impl FnOnce(&mut Self),
    ) -> AstResult<Ast> {
        if let TokenKind::Keyword(keyword) = tokens[0].kind {
            match keyword {
                Keyword::Func(func_kw) => match func_kw {
                    word::FuncKeyword::Main => {
                        self.env.set_value(Env::Main);
                        Ok(Ast::MainDef)
                    }
                    word::FuncKeyword::Test => {
                        self.env.set_value(Env::Test);
                        todo!()
                    }
                    word::FuncKeyword::Proc => {
                        self.env.set_value(Env::Proc);
                        todo!()
                    }
                    word::FuncKeyword::Func => Ok(Ast::FuncDef {
                        kind: FuncKind::PureFunc,
                        decl: self.parse_func_decl(trim!(tokens; keyword, colon))?,
                    }),
                    word::FuncKeyword::Def => todo!(),
                },
                Keyword::Type(ty_kw) => match ty_kw {
                    word::TypeKeyword::Struct => {
                        expect_len!(tokens, 3);
                        expect_head!(tokens);
                        Ok(Ast::TypeDef {
                            ident: identify!(tokens[1]),
                            kind: TyKind::Struct,
                            generics: Vec::new(),
                        })
                    }
                    word::TypeKeyword::Rename => todo!(),
                    word::TypeKeyword::Enum => todo!(),
                    word::TypeKeyword::Props => todo!(),
                },
                Keyword::Use | Keyword::Mod => todo!(),
                Keyword::Stmt(kw) => self
                    .parse_stmt(Some((kw, tokens[0].range.clone())), &tokens[1..])
                    .map(|stmt| stmt.into()),
                Keyword::Config(cfg) => match cfg {
                    ConfigKeyword::Dataset => {
                        self.env.set_value(Env::DatasetConfig);
                        Ok(Ast::DatasetConfig)
                    }
                },
            }
        } else {
            if tokens.len() >= 2 && tokens[1].kind == TokenKind::Special(Special::Colon) {
                if tokens.len() == 2 {
                    todo!()
                }
                let ident = match tokens[0].kind {
                    TokenKind::Identifier(ident) => match ident {
                        Identifier::Builtin(_) => ast_err!(
                            tokens[0].text_range(),
                            "expect custom identifier but got builtin"
                        )?,
                        Identifier::Custom(custom_ident) => custom_ident,
                    },
                    _ => ast_err!(tokens[0].text_range(), "expect custom identifier")?,
                };
                let ty = atom::parse_ty(self.symbol_proxy(), &tokens[2..])?;
                Ok(Ast::MembDef {
                    ident,
                    kind: MembKind::MembVar {
                        ty: MembType {
                            contract: InputContract::Own,
                            ty,
                        },
                    },
                })
            } else {
                self.parse_stmt(None, tokens).map(|stmt| stmt.into())
            }
        }
    }

    fn folded_output_mut(&mut self) -> &mut FoldedList<AstResult<Ast>> {
        &mut self.folded_results
    }
}
