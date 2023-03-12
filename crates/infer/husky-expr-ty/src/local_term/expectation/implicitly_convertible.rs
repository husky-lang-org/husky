use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ImplicitConversion {
    None,
    Never,
    Other,
}

/// expect a type that is implicitly convertible to dst
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectImplicitlyConvertible {
    pub(crate) destination: LocalTerm,
}

impl ExpectImplicitlyConvertible {
    pub(in super::super) fn try_substitute_unresolved_local_term<'a>(
        &self,
        unresolved_terms: &'a UnresolvedTerms,
    ) -> Result<Option<LocalTermExpectation>, &'a LocalTermResolveError> {
        match unresolved_terms.try_reduce_local_term(self.destination)? {
            Some(destination) => Ok(Some(ExpectImplicitlyConvertible { destination }.into())),
            None => Ok(None),
        }
    }
}

impl ExpectLocalTerm for ExpectImplicitlyConvertible {
    type Outcome = ImplicitConversion;

    fn retrieve_outcome(outcome: &LocalTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            LocalTermExpectationOutcome::ImplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination {
        self.destination.final_destination(db, unresolved_terms)
    }

    fn destination(&self) -> Option<LocalTerm> {
        Some(self.destination)
    }
}

impl ExpectImplicitlyConvertible {
    pub(super) fn resolve(
        &self,
        db: &dyn ExprTypeDb,
        src: LocalTerm,
        level: LocalTermResolveLevel,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        if src == self.destination {
            return Some(LocalTermExpectationEffect {
                result: Ok(ImplicitConversion::None.into()),
                actions: smallvec![],
            });
        }
        let src_patt = src.pattern(db, unresolved_terms);
        let dst_patt = self.destination.pattern(db, unresolved_terms);
        match dst_patt {
            LocalTermPattern::Literal(_) => todo!(),
            LocalTermPattern::TypeOntology {
                refined_path: dst_path,
                arguments: dst_arguments,
                ..
            } => match src_patt {
                LocalTermPattern::TypeOntology {
                    refined_path: Right(PreludeTypePath::NEVER),
                    ..
                } => Some(LocalTermExpectationEffect {
                    result: Ok(ImplicitConversion::Never.into()),
                    actions: smallvec![],
                }),
                LocalTermPattern::TypeOntology {
                    refined_path: src_path,
                    arguments: src_arguments,
                    ..
                } if dst_path == src_path => {
                    if dst_arguments.len() != src_arguments.len() {
                        p!(src.debug(db), self.destination.debug(db));
                        todo!()
                    }
                    let actions = smallvec![];
                    for (src_argument, dst_argument) in
                        std::iter::zip(src_arguments.into_iter(), dst_arguments.into_iter())
                    {
                        if src_argument != dst_argument {
                            p!(src_argument.debug(db), dst_argument.debug(db));
                            todo!()
                        }
                    }
                    Some(LocalTermExpectationEffect {
                        result: Ok(ImplicitConversion::None.into()),
                        actions,
                    })
                }
                LocalTermPattern::TypeOntology {
                    refined_path: src_path,
                    arguments: src_arguments,
                    ..
                } => {
                    p!(dst_path.debug(db), src_path.debug(db));
                    todo!()
                }
                LocalTermPattern::ImplicitSymbol(_, src_implicit_symbol) => match level {
                    LocalTermResolveLevel::Weak => None,
                    LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                        result: Ok(LocalTermExpectationOutcome::ImplicitlyConvertible(
                            ImplicitConversion::None,
                        )),
                        actions: smallvec![TermResolveAction::SubstituteImplicitSymbol {
                            implicit_symbol: src_implicit_symbol,
                            substitution: self.destination,
                        }],
                    }),
                },
                _ => {
                    p!(src.debug(db), self.destination.debug(db));
                    Some(LocalTermExpectationEffect {
                        result: Err(todo!()),
                        actions: smallvec![],
                    })
                }
            },
            LocalTermPattern::Curry { .. } => todo!(),
            LocalTermPattern::ImplicitSymbol(_, dst_implicit_symbol) => match level {
                LocalTermResolveLevel::Weak => None,
                LocalTermResolveLevel::Strong => Some(LocalTermExpectationEffect {
                    actions: smallvec![TermResolveAction::SubstituteImplicitSymbol {
                        implicit_symbol: dst_implicit_symbol,
                        substitution: src,
                    }],
                    result: Ok(ImplicitConversion::None.into()),
                }),
            },
            LocalTermPattern::Category(_) => todo!(),
            LocalTermPattern::Ritchie { .. } => todo!(),
        }
    }
}
