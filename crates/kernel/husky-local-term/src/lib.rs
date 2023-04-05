#![feature(anonymous_lifetime_in_impl_trait)]
#![feature(trait_upcasting)]
mod engine;
mod expectation;
mod pattern;
mod progress;
mod region;
mod unresolved;
mod utils;

use husky_print_utils::p;

pub use self::engine::*;
pub use self::expectation::*;
pub use self::pattern::*;
pub use self::progress::*;
pub use self::region::*;
pub use self::unresolved::*;

use either::*;
use husky_entity_path::*;
use husky_expr::*;
use husky_term::*;
use husky_term_prelude::*;
use salsa::DebugWithDb as _;
use smallvec::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[salsa::derive_debug_with_db(db = TermDb, jar = TermJar)]
#[enum_class::from_variants]
pub enum LocalTerm {
    Resolved(Term),
    Unresolved(UnresolvedTermIdx),
}

impl LocalTerm {
    pub fn new_application(
        db: &dyn TermDb,
        local_term_region: &mut LocalTermRegion,
        src_expr_idx: ExprIdx,
        function: impl Into<LocalTerm>,
        argument: impl Into<LocalTerm>,
    ) -> TermResult<Self> {
        match (function.into(), argument.into()) {
            (LocalTerm::Resolved(function), LocalTerm::Resolved(argument)) => {
                Ok(TermApplication::new(db, function, argument)?.into())
            }
            (LocalTerm::Resolved(function), argument) => {
                let expansion = function.application_expansion(db);
                match expansion.function() {
                    TermFunctionReduced::TypeOntology(path) => {
                        let mut arguments: SmallVec<[LocalTerm; 2]> = expansion
                            .arguments(db)
                            .iter()
                            .copied()
                            .map(Into::into)
                            .collect();
                        arguments.push(argument);
                        Ok(local_term_region.intern_unresolved_term(
                            src_expr_idx,
                            UnresolvedTerm::TypeOntology { path, arguments },
                        ))
                    }
                    TermFunctionReduced::Trait(_) => todo!(),
                    TermFunctionReduced::Other(_) => todo!(),
                }
            }
            (LocalTerm::Unresolved(_), LocalTerm::Resolved(_)) => todo!(),
            (LocalTerm::Unresolved(_), LocalTerm::Unresolved(_)) => todo!(),
        }
    }

    pub(crate) fn new_ty_ontology_application(
        db: &dyn TermDb,
        unresolved_terms: &mut UnresolvedTerms,
        src_expr_idx: ExprIdx,
        path: TypePath,
        arguments: SmallVec<[LocalTerm; 2]>,
    ) -> Self {
        let mut resolved_arguments: SmallVec<[Term; 2]> = smallvec![];
        for argument in &arguments {
            match argument {
                LocalTerm::Resolved(argument) => resolved_arguments.push(*argument),
                LocalTerm::Unresolved(_) => break,
            }
        }
        if resolved_arguments.len() == arguments.len() {
            todo!()
        } else {
            unresolved_terms.intern_unresolved_term(
                src_expr_idx,
                UnresolvedTerm::TypeOntology { path, arguments },
            )
        }
    }

    pub fn unravel_borrow(self, db: &dyn TermDb, unresolved_terms: &UnresolvedTerms) -> Self {
        match self.pattern_inner(db, unresolved_terms) {
            LocalTermPattern::TypeOntology {
                refined_path: Right(PreludeTypePath::Borrow(path)),
                argument_tys: arguments,
                ..
            } => match path {
                PreludeBorrowTypePath::Ref | PreludeBorrowTypePath::RefMut => {
                    assert_eq!(arguments.len(), 2);
                    todo!()
                }
                PreludeBorrowTypePath::Leash => {
                    assert_eq!(arguments.len(), 1);
                    arguments[0]
                }
            },
            _ => self,
        }
    }

    fn resolved(self) -> Option<Term> {
        match self {
            LocalTerm::Resolved(term) => Some(term),
            LocalTerm::Unresolved(_) => None,
        }
    }

    pub(crate) fn resolve_progress(
        self,
        unresolved_terms: &UnresolvedTerms,
    ) -> LocalTermResolveResultRef<Self> {
        match self {
            LocalTerm::Resolved(term) => Ok(term.into()),
            LocalTerm::Unresolved(idx) => idx.resolve_progress(unresolved_terms),
        }
    }
}

impl From<TermLiteral> for LocalTerm {
    fn from(value: TermLiteral) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermSymbol> for LocalTerm {
    fn from(value: TermSymbol) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermPlaceholder> for LocalTerm {
    fn from(value: TermPlaceholder) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermEntityPath> for LocalTerm {
    fn from(value: TermEntityPath) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermCategory> for LocalTerm {
    fn from(value: TermCategory) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermUniverse> for LocalTerm {
    fn from(value: TermUniverse) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermCurry> for LocalTerm {
    fn from(value: TermCurry) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermRitchie> for LocalTerm {
    fn from(value: TermRitchie) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermAbstraction> for LocalTerm {
    fn from(value: TermAbstraction) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermApplication> for LocalTerm {
    fn from(value: TermApplication) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermSubentity> for LocalTerm {
    fn from(value: TermSubentity) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermAsTraitSubentity> for LocalTerm {
    fn from(value: TermAsTraitSubentity) -> Self {
        LocalTerm::Resolved(value.into())
    }
}

impl From<TermTraitConstraint> for LocalTerm {
    fn from(value: TermTraitConstraint) -> Self {
        LocalTerm::Resolved(value.into())
    }
}
