use crate::*;
use token::*;
use vm::{FieldLiason, InputLiason};
use word::Paradigm;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_record_item(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        match token_group[0].kind {
            TokenKind::Keyword(keyword) => match keyword {
                Keyword::Config(_) => todo!(),
                Keyword::Paradigm(_) => {
                    // Keyword::Def => self.parse_record_derived_field(token_group, enter_block),
                    todo!()
                }
                Keyword::Type(_) => todo!(),
                Keyword::Stmt(_) => todo!(),
                Keyword::Use => todo!(),
                Keyword::Mod => todo!(),
                Keyword::Main => todo!(),
                Keyword::Visual => todo!(),
            },
            TokenKind::Identifier(_) => self.parse_record_original_field(token_group),
            TokenKind::Special(_) => todo!(),
            TokenKind::PrimitiveLiteral(_) => todo!(),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
            TokenKind::WordOpr(_) => todo!(),
            TokenKind::Decorator(_) => todo!(),
        }
    }

    fn parse_record_original_field(&mut self, token_group: &[Token]) -> AstResult<AstKind> {
        if token_group.len() >= 2 && token_group[1].kind == TokenKind::Special(Special::Colon) {
            if token_group.len() == 2 {
                todo!()
            }
            let ident = identify_token!(self, &token_group[0], SemanticTokenKind::Field);
            let symbol_context = self.symbol_context();
            let ty = atom::parse_route(&symbol_context, &token_group[2..])?;
            emsg_once!("field contract");
            Ok(AstKind::FieldDefnHead(FieldDefnHead {
                ident,
                contract: FieldLiason::Own,
                ty,
                kind: FieldKind::RecordOriginal,
            }))
        } else {
            p!(token_group);
            todo!()
        }
    }

    fn parse_record_derived_field(
        &mut self,
        token_group: &[Token],
        enter_block: impl FnOnce(&mut Self),
    ) -> AstResult<AstKind> {
        enter_block(self);
        self.context.set(AstContext::Stmt(Paradigm::LazyFunctional));
        self.opt_this_contract.set(Some(InputLiason::GlobalRef));
        let ident = identify_token!(self, &token_group[1], SemanticTokenKind::Field);
        emsg_once!("field contract");
        let ty = atom::parse_route(&self.symbol_context(), &token_group[3..])?;
        Ok(AstKind::FieldDefnHead(FieldDefnHead {
            ident,
            ty,
            kind: FieldKind::RecordDerived,
            contract: FieldLiason::Own,
        }))
    }
}
