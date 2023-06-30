use super::*;
use husky_print_utils::p;
use salsa::DebugWithDb;
use smallvec::smallvec;

/// stack based expression parsing.
///
/// finished expression if exists, sits on top of unfinished ones.
///
/// For example,
/// ```husky
/// a + b * (c + d
/// ```
/// in the above `c + d` would be the finished expression, `a +`, `b *` and `(c + d` would be unfinished expressions.
#[derive(Default, Debug)]
pub(crate) struct ExprStack {
    incomplete_exprs: Vec<(IncompleteExpr, Precedence)>,
    complete_expr: Option<Expr>,
}

pub(super) enum TopExpr {
    Unfinished(IncompleteExpr),
    Finished(Expr),
}

pub(super) enum TopExprRef<'a> {
    Incomplete(&'a IncompleteExpr),
    Finished(&'a Expr),
    None,
}

impl From<Expr> for TopExpr {
    fn from(v: Expr) -> Self {
        Self::Finished(v)
    }
}

impl From<IncompleteExpr> for TopExpr {
    fn from(v: IncompleteExpr) -> Self {
        Self::Unfinished(v)
    }
}

impl ExprStack {
    pub(super) fn prev_unfinished_expr_precedence(&self) -> Option<Precedence> {
        self.incomplete_exprs
            .last()
            .map(|(_, precedence)| *precedence)
    }
}

impl Expr {
    pub fn base_entity_path(&self, db: &dyn WordDb, arena: &ExprArena) -> BaseEntityPath {
        match self {
            Expr::Literal(_, _) => BaseEntityPath::None,
            Expr::PrincipalEntityPath { opt_path: path, .. } => match *path {
                Some(path) => BaseEntityPath::Some(path.into()),
                None => todo!(),
            },
            Expr::ScopeResolution {
                parent_expr_idx,
                scope_resolution_token,
                ident_token,
            } => todo!(),
            Expr::InheritedSymbol { .. } | Expr::CurrentSymbol { .. } => BaseEntityPath::None,
            Expr::SelfValue(_) => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::Binary {
                lopd,
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
                ropd,
            } => todo!(),
            Expr::Prefix {
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
                opd,
            } => todo!(),
            Expr::Suffix {
                opd,
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
            } => todo!(),
            Expr::Field { .. } => BaseEntityPath::None,
            Expr::MethodApplicationOrCall { .. } => BaseEntityPath::None,
            Expr::ExplicitApplication { function, argument } => todo!(),
            Expr::FunctionApplicationOrCall { .. } => todo!(),
            // although unit is a valid entity,
            // but unit doesn't contains any subentity, so effectively none
            // ad hoc
            Expr::Unit { .. } => BaseEntityPath::None,
            Expr::NewTuple {
                lpar_token_idx,
                items,
                rpar_token_idx,
                ..
            } => todo!(),
            Expr::List { .. } => BaseEntityPath::None,
            Expr::Bracketed { item, .. } => arena[item].base_entity_path(db, arena),
            Expr::Err(e) => BaseEntityPath::Uncertain {
                inclination: match e {
                    ExprError::Original(OriginalExprError::UnrecognizedIdent { ident, .. }) => {
                        BaseEntityPathInclination::from_case(ident.case(db))
                    }
                    // ad hoc
                    _ => BaseEntityPathInclination::FunctionOrLocalValue,
                },
            },
            Expr::TemplateInstantiation { template, .. } => {
                arena[template].base_entity_path(db, arena)
            }
            Expr::Block { stmts } => BaseEntityPath::None,
            Expr::Be {
                src,
                be_token_idx,
                target,
            } => todo!(),
            Expr::BoxColonList { .. } => todo!(),
            Expr::FrameVarDecl {
                token_idx, ident, ..
            } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                items: indices,
                rbox_token_idx,
            } => arena[owner].base_entity_path(db, arena),
            Expr::EmptyHtmlTag { .. } => BaseEntityPath::Err,
            Expr::FunctionCall { .. } => todo!(),
            Expr::Ritchie { .. } => todo!(),
        }
    }
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(super) fn complete_expr(&self) -> Option<&Expr> {
        self.stack.complete_expr.as_ref()
    }

    pub(super) fn incomplete_exprs(&self) -> &[(IncompleteExpr, Precedence)] {
        &self.stack.incomplete_exprs
    }

    pub(super) fn take_last_incomplete_expr(&mut self) -> Option<IncompleteExpr> {
        self.stack.incomplete_exprs.pop().map(|(expr, _)| expr)
    }

    fn push_unfinished_expr(&mut self, incomplete_expr: IncompleteExpr) {
        assert!(self.stack.complete_expr.is_none());
        let precedence = incomplete_expr.precedence();
        self.stack
            .incomplete_exprs
            .push((incomplete_expr, precedence))
    }

    pub(super) fn last_incomplete_expr(&self) -> Option<&IncompleteExpr> {
        self.stack.incomplete_exprs.last().map(|(opr, _)| opr)
    }

    pub(super) fn last_incomplete_expr_mut(&mut self) -> Option<&mut IncompleteExpr> {
        self.stack.incomplete_exprs.last_mut().map(|(opr, _)| opr)
    }

    /// make `top_expr` the top expression.
    /// - if there is already a finished expression, interpret it as a function,
    /// and `top_expr` as an argument;
    /// - otherwise just adds it in the trivial way
    pub(super) fn push_top_expr(&mut self, top_expr: TopExpr) {
        // this is for guaranteeing that application is left associative
        if self.complete_expr().is_some() {
            self.reduce(Precedence::Application)
        };
        if let Some(function) = self.take_complete_expr() {
            self.push_unfinished_expr(IncompleteExpr::Application { function });
        }
        match top_expr {
            TopExpr::Unfinished(unfinished_expr) => self.push_unfinished_expr(unfinished_expr),
            TopExpr::Finished(finished_expr) => self.stack.complete_expr = Some(finished_expr),
        }
    }

    /// if there's no need for the information of unfinished expressions, call `finished_expr` would be faster
    pub(super) fn top_expr<'d>(&'d self) -> TopExprRef<'d> {
        if let Some(ref finished_expr) = self.stack.complete_expr {
            TopExprRef::Finished(finished_expr)
        } else if let Some((unfinished_expr, _)) = self.stack.incomplete_exprs.last() {
            TopExprRef::Incomplete(unfinished_expr)
        } else {
            TopExprRef::None
        }
    }

    pub(super) fn take_complete_expr(&mut self) -> Option<Expr> {
        std::mem::take(&mut self.stack.complete_expr)
    }

    pub(super) fn set_complete_expr(&mut self, expr: Expr) {
        debug_assert!(self.complete_expr().is_none());
        self.stack.complete_expr = Some(expr)
    }

    fn reduce_aux(&mut self, f: impl Fn(&mut Self, Option<Expr>, IncompleteExpr) -> TopExpr) {
        let complete_expr = self.take_complete_expr();
        let Some((incomplete_expr, _)) = self.stack.incomplete_exprs.pop() else {
            unreachable!()
        };
        let top_expr = f(self, complete_expr, incomplete_expr);
        self.push_top_expr(top_expr)
    }

    pub(super) fn reduce(&mut self, next_precedence: Precedence) {
        while let Some(prev_precedence) = self.stack.prev_unfinished_expr_precedence() {
            if prev_precedence < next_precedence {
                break;
            }
            // curry is right associative
            if prev_precedence == Precedence::Curry && next_precedence == Precedence::Curry {
                break;
            }
            match self.stack.incomplete_exprs.pop().unwrap().0 {
                IncompleteExpr::Binary {
                    lopd,
                    punctuation,
                    punctuation_token_idx,
                } => {
                    let lopd = self.alloc_expr(lopd);
                    let finished_expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match finished_expr {
                        Some(ropd) => Expr::Binary {
                            lopd,
                            opr: punctuation,
                            opr_token_idx: punctuation_token_idx,
                            ropd: self.alloc_expr(ropd),
                        },
                        None => Expr::Err(
                            OriginalExprError::NoRightOperandForBinaryOperator {
                                punctuation,
                                punctuation_token_idx,
                            }
                            .into(),
                        ),
                    })
                }
                IncompleteExpr::Application { function } => {
                    let argument = self.take_complete_expr().expect("");
                    let function = self.alloc_expr(function);
                    let argument = self.alloc_expr(argument);
                    self.stack.complete_expr =
                        Some(Expr::ExplicitApplication { function, argument })
                }
                IncompleteExpr::Prefix {
                    punctuation,
                    punctuation_token_idx,
                } => {
                    let finished_expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match finished_expr {
                        Some(opd) => Expr::Prefix {
                            opr: punctuation,
                            opr_token_idx: punctuation_token_idx,
                            opd: self.alloc_expr(opd),
                        },
                        None => Expr::Err(
                            OriginalExprError::NoOperandForPrefixOperator {
                                prefix: punctuation,
                                prefix_token_idx: punctuation_token_idx,
                            }
                            .into(),
                        ),
                    })
                }
                IncompleteExpr::CommaList { bra_token_idx, .. } => {
                    self.stack.complete_expr = Some(Expr::Err(
                        OriginalExprError::UnterminatedList { bra_token_idx }.into(),
                    ))
                }
                IncompleteExpr::LambdaHead { inputs, start } => todo!(),
                IncompleteExpr::CallList { lpar_token_idx, .. } => {
                    p!(prev_precedence, next_precedence);
                    todo!();
                    self.stack.complete_expr = Some(Expr::Err(
                        OriginalExprError::UnterminatedFunctionCallKeyedArgumentList {
                            bra_token_idx: lpar_token_idx,
                        }
                        .into(),
                    ))
                }
                IncompleteExpr::Ritchie {
                    ritchie_kind_token_idx,
                    ritchie_kind,
                    lpar_token,
                    argument_tys,
                    rpar_token_idx,
                    light_arrow_token,
                } => {
                    let finished_expr = self.take_complete_expr();
                    self.stack.complete_expr = Some(match finished_expr {
                        Some(return_ty) => Expr::Ritchie {
                            ritchie_kind_token_idx,
                            ritchie_kind,
                            lpar_token,
                            parameter_ty_items: argument_tys,
                            rpar_token_idx,
                            light_arrow_token: Some(light_arrow_token),
                            return_ty_expr: Some(self.alloc_expr(return_ty)),
                        },
                        None => Expr::Err(
                            OriginalExprError::ExpectedTypeAfterLightArrow { light_arrow_token }
                                .into(),
                        ),
                    })
                }
                IncompleteExpr::KeyedArgument {
                    key_token_idx,
                    key,
                    eq_token,
                } => {
                    self.reduce_aux(|this, opt_complete_expr, incomplete_expr| {
                        let Some(argument_expr) =  opt_complete_expr else {
                            todo!()
                        };
                        let argument_expr_idx = this.alloc_expr(argument_expr);
                        match incomplete_expr {
                            IncompleteExpr::CommaList {
                                opr: IncompleteCommaListOpr::FunctionApplicationOrCall { function },
                                bra,
                                bra_token_idx,
                                mut items,
                            } => {
                                let mut items: SmallVec<[CallListItem; 4]> =
                                    items.into_iter().map(Into::into).collect();
                                items.push(CallListItem {
                                    kind: CallListItemKind::KeyedArgument { key_token_idx, key },
                                    argument_expr_idx,
                                    separator: CallListSeparator::None,
                                });
                                IncompleteExpr::CallList {
                                    opr: IncompleteCallListOpr::FunctionCall {
                                        function,
                                        implicit_arguments: /* ad hoc */ None,
                                    },
                                    lpar_token_idx: bra_token_idx,
                                    items,
                                }
                                .into()
                            }
                            IncompleteExpr::CommaList {
                                opr:
                                    IncompleteCommaListOpr::MethodApplicationOrCall {
                                        self_expr,
                                        dot_token_idx,
                                        ident_token,
                                        implicit_arguments,
                                    },
                                bra,
                                bra_token_idx,
                                items,
                            } => todo!(),
                            IncompleteExpr::CallList { .. } => todo!(),
                            _ => unreachable!(),
                        }
                    })
                }
            }
        }
    }

    /// use this when the incoming token might change the nature of the top expression
    pub(super) fn take_complete_and_push_to_top(
        &mut self,
        f: impl FnOnce(&mut Self, Option<Expr>) -> TopExpr,
    ) {
        let complete_expr = self.take_complete_expr();
        let top_expr = f(self, complete_expr);
        self.push_top_expr(top_expr)
    }

    pub(super) fn finish_batch(&mut self) -> Option<ExprIdx> {
        assert!(self.stack.incomplete_exprs.len() == 0);
        std::mem::take(&mut self.stack.complete_expr).map(|expr| self.parser.alloc_expr(expr))
    }

    pub(super) fn last_bra(&self) -> Option<Bracket> {
        for (unfinished_expr, _) in self.stack.incomplete_exprs.iter().rev() {
            match unfinished_expr {
                IncompleteExpr::CommaList {
                    opr,
                    bra,
                    bra_token_idx,
                    items,
                } => return Some(*bra),
                IncompleteExpr::CallList { .. } => return Some(Bracket::Par),
                _ => (),
            }
        }
        None
    }

    pub(super) fn last_two_bras(&self) -> Vec<Bracket> {
        let mut bras = vec![];
        for (unfinished_expr, _) in self.stack.incomplete_exprs.iter().rev() {
            match unfinished_expr {
                IncompleteExpr::CommaList { bra, .. } => {
                    bras.push(*bra);
                    if bras.len() >= 2 {
                        return bras;
                    }
                }
                _ => (),
            }
        }
        bras
    }
}
