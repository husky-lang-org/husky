use super::*;

impl<'a, 'b, 'c> ExprParser<'a, 'b, 'c> {
    pub(crate) fn next_token(&mut self) -> Option<&'b Token> {
        self.token_iter.next()
    }

    pub(crate) fn accept_token(&mut self, token: ResolvedToken) -> ExprResult<()> {
        match token.kind() {
            ResolvedTokenKind::Atom(_atom) => {
                Ok(self.accept_atom(token.to_expr(self.sheet.expr_arena())))
            }
            ResolvedTokenKind::BinaryOpr(opr) => self.accept_binary_opr(*opr, token.token_idx()),
            ResolvedTokenKind::Prefix(opr) => Ok(self.accept_prefix_opr(*opr, token.token_idx())),
            ResolvedTokenKind::Suffix(opr) => Ok(self.accept_suffix_opr(*opr, token.token_idx())),
            ResolvedTokenKind::Bra(bra) => {
                Ok(self.accept_list_start(*bra, token.token_idx(), ListStartAttr::None))
            }
            ResolvedTokenKind::Ket(ket) => {
                self.accept_list_end(*ket, token.token_idx(), ListEndAttr::None)
            }
            ResolvedTokenKind::Dot => self.accept_dot_opr(token.token_idx()),
            ResolvedTokenKind::Comma => self.accept_comma(token.token_idx()),
        }
    }

    fn accept_atom(&mut self, atom: Expr) {
        self.push_expr(atom)
    }

    fn accept_prefix_opr(&mut self, prefix: PrefixPunctuation, prefix_token_idx: TokenIdx) {
        self.push_opr(PartialOpn::Prefix {
            prefix,
            prefix_token_idx,
        })
    }

    fn accept_suffix_opr(&mut self, suffix: RawSuffixOpr, suffix_token_idx: TokenIdx) {
        self.synthesize_suffix(suffix, suffix_token_idx)
    }

    fn accept_dot_opr(&mut self, dot_token_idx: TokenIdx) -> ExprResult<()> {
        // let (ident, ident_token_idx) = self
        //     .try_get_identifier()
        //     .ok_or(ExprError::ExpectIdentifierAfterDot)?;
        // self.push_opr(StackOpr::Dot { dot_token_idx });
        todo!()
    }

    fn accept_comma(&mut self, token_idx: TokenIdx) -> Result<(), ExprError> {
        match self.top_opn() {
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    pub(crate) fn accept_binary_opr(
        &mut self,
        binary: BinaryPunctuation,
        binary_token_idx: TokenIdx,
    ) -> ExprResult<()> {
        let stack_opr = PartialOpn::Binary {
            binary,
            binary_token_idx,
        };
        self.synthesize_all_above(stack_opr.precedence())?;
        self.push_opr(stack_opr);
        Ok(())
    }

    pub(crate) fn accept_list_start(
        &mut self,
        bra: Bracket,
        bra_token_idx: TokenIdx,
        attr: ListStartAttr,
    ) {
        let attached = attr.attached();
        self.push_opr(PartialOpn::ListStart {
            bra,
            bra_token_idx,
            attr,
        });
        if attached {
            self.push_opr(PartialOpn::ListItem {
                separator_token_idx: None,
            })
        }
    }

    pub(crate) fn accept_list_item(
        &mut self,
        separator_token_idx: Option<TokenIdx>,
    ) -> ExprResult<()> {
        self.synthesize_all_above(Precedence::ListItem)?;
        self.push_opr(PartialOpn::ListItem {
            separator_token_idx,
        });
        Ok(())
    }

    pub(crate) fn accept_list_end(
        &mut self,
        ket: Bracket,
        ket_token: TokenIdx,
        attr: ListEndAttr,
    ) -> ExprResult<()> {
        let original_number_of_oprs = self.number_of_oprs();
        let (start_attr, bra_token) = {
            loop {
                match self.pop_opr() {
                    Some(opr) => match opr {
                        PartialOpn::ListItem { .. } => (),
                        PartialOpn::ListStart {
                            bra,
                            bra_token_idx: bra_token,
                            attr,
                        } => {
                            if ket != bra {
                                return Err(ExprError::MisMatchingBracket {
                                    bra,
                                    bra_token,
                                    ket: ket,
                                    ket_token: ket_token,
                                });
                            };
                            break (attr, bra_token);
                        }
                        _ => {
                            return Err(ExprError::NoMatchingBra {
                                ket: ket,
                                ket_token: ket_token,
                            })
                        }
                    },
                    None => {
                        return Err(ExprError::NoMatchingBra {
                            ket: ket,
                            ket_token: ket_token,
                        })
                    }
                }
            }
        };
        let list_len = original_number_of_oprs - self.number_of_oprs() - 1;
        let (opds, paths) = self.drain_exprs(list_len);
        let opds = self.sheet.alloc_expr_batch(opds, paths);
        self.push_expr(new_list_expr(ket, start_attr, attr, opds)?);
        Ok(())
    }
}

fn new_list_expr(
    bracket: Bracket,
    start_attr: ListStartAttr,
    end_attr: ListEndAttr,
    opds: ExprIdxRange,
) -> ExprResult<Expr> {
    if bracket == Bracket::Par && start_attr == ListStartAttr::None && idx_arena::len(&opds) == 1 {
        return Ok(Expr::Bracketed(opds.start()));
    }
    let opn_variant = Opn::List(match start_attr {
        ListStartAttr::None => match bracket {
            Bracket::Par => ListOpr::NewTuple,
            Bracket::Box => ListOpr::NewVec,
            Bracket::Curl => ListOpr::NewDict,
            Bracket::Angle => todo!(),
        },
        ListStartAttr::Attach => match bracket {
            Bracket::Par => ListOpr::FunctionCall,
            Bracket::Box => match end_attr {
                ListEndAttr::None => {
                    if idx_arena::len(&opds) < 2 {
                        todo!()
                        // return err!(format!("expect index expr inside `[]`"), range);
                    }
                    ListOpr::Index
                }
                ListEndAttr::Modulo => ListOpr::ModuloIndex,
                ListEndAttr::Attach => todo!(),
            },
            Bracket::Curl => ListOpr::StructInit,
            Bracket::Angle => todo!(),
        },
        ListStartAttr::MethodAttach { ranged_ident } => ListOpr::MethodCall { ranged_ident },
    });
    Ok(Expr::Opn {
        opn: opn_variant,
        opds,
    })
}
