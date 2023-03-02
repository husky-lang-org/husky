mod eqs_eqs_function_ty;
mod eqs_exactly;
mod eqs_ref_mut_application;
mod eqs_sort;
mod explicitly_convertible;
mod implicitly_convertible;
mod ins_sort;

pub(crate) use eqs_eqs_function_ty::*;
pub(crate) use eqs_exactly::*;
pub(crate) use eqs_ref_mut_application::*;
pub(crate) use eqs_sort::*;
pub(crate) use explicitly_convertible::*;
pub(crate) use implicitly_convertible::*;
pub(crate) use ins_sort::*;

use super::*;
use husky_print_utils::p;
use idx_arena::Arena;
use thiserror::Error;

pub(crate) trait ExpectLocalTerm:
    ProvideTypeContext + Clone + Into<LocalTermExpectation>
{
    type ResolvedOk: ExpectLocalTermResolvedOk;

    fn destination(&self) -> Option<LocalTerm>;
}

pub(crate) trait ExpectLocalTermResolvedOk: Into<LocalTermExpectationResolvedOk> {
    fn destination(&self) -> LocalTerm;

    /// will panic if not right
    fn downcast_ref(resolved_ok: &LocalTermExpectationResolvedOk) -> &Self;
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct LocalTermExpectationEntry {
    src_expr_idx: ExprIdx,
    expectee: LocalTerm,
    expectation: LocalTermExpectation,
    resolve_progress: LocalTermExpectationResolveProgress,
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum LocalTermExpectationResolvedOk {
    ExplicitlyConvertible(ExpectExplicitlyConvertibleResolvedOk),
    ImplicitlyConvertible(ExpectImplicitlyConvertibleResolvedOk),
    InsSort(ExpectInsSortResolvedOk),
    EqsSort(ExpectEqsSortResolvedOk),
    EqsExactly(ExpectEqsExactlyResolvedOk),
    EqsRefMutApplication(ExpectEqsRefMutApplicationResolvedOk),
    EqsRitchieCallType(ExpectEqsFunctionTypeOk),
}

impl LocalTermExpectationResolvedOk {
    fn resolved(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermExpectationResolvedOk::ExplicitlyConvertible(_) => todo!(),
            LocalTermExpectationResolvedOk::ImplicitlyConvertible(_) => todo!(),
            LocalTermExpectationResolvedOk::InsSort(result) => result.resolved(),
            LocalTermExpectationResolvedOk::EqsSort(_) => todo!(),
            LocalTermExpectationResolvedOk::EqsExactly(result) => result.resolved(),
            LocalTermExpectationResolvedOk::EqsRefMutApplication(_) => todo!(),
            LocalTermExpectationResolvedOk::EqsRitchieCallType(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum LocalTermExpectationResolveProgress {
    Unresolved,
    Resolved(LocalTermExpectationResult<LocalTermExpectationResolvedOk>),
}

impl LocalTermExpectationResolveProgress {
    pub(crate) fn resolved_ok<R: ExpectLocalTermResolvedOk>(&self) -> Option<&R> {
        match self {
            LocalTermExpectationResolveProgress::Unresolved => None,
            LocalTermExpectationResolveProgress::Resolved(Ok(resolved_ok)) => {
                Some(R::downcast_ref(resolved_ok))
            }
            LocalTermExpectationResolveProgress::Resolved(Err(_)) => None,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum LocalTermExpectationError {
    #[error("original {0}")]
    Original(#[from] OriginalLocalTermExpectationError),
    #[error("derived {0}")]
    Derived(#[from] DerivedLocalTermExpectationError),
}

pub type LocalTermExpectationResult<T> = Result<T, LocalTermExpectationError>;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum OriginalLocalTermExpectationError {
    #[error("{term:?} {error}")]
    TermTypeError { term: Term, error: TypeError },
    // #[error("todo")]
    // Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum DerivedLocalTermExpectationError {
    #[error("{term:?} {error}")]
    TermTypeError { term: Term, error: TypeError },
    #[error("{0}")]
    Type(#[from] DerivedTypeError),
    #[error("target substitution failure")]
    TargetSubstitutionFailure,
    #[error("duplication")]
    Duplication(LocalTermExpectationIdx),
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
}

impl LocalTermExpectationResolveProgress {
    // it will use derived type error
    pub(crate) fn duplicate(&self, src: LocalTermExpectationIdx) -> Self {
        match self {
            LocalTermExpectationResolveProgress::Unresolved => {
                LocalTermExpectationResolveProgress::Unresolved
            }
            LocalTermExpectationResolveProgress::Resolved(expectation_result) => {
                match expectation_result {
                    Ok(expectation_ok) => {
                        LocalTermExpectationResolveProgress::Resolved(Ok(expectation_ok.clone()))
                    }
                    Err(_) => LocalTermExpectationResolveProgress::Resolved(Err(
                        DerivedLocalTermExpectationError::Duplication(src).into(),
                    )),
                }
            }
        }
    }

    pub(crate) fn reduced_term(&self) -> Option<ReducedTerm> {
        match self {
            LocalTermExpectationResolveProgress::Unresolved
            | LocalTermExpectationResolveProgress::Resolved(Err(_)) => None,
            LocalTermExpectationResolveProgress::Resolved(Ok(result)) => result.resolved(),
        }
    }
}

impl LocalTermExpectationEntry {
    pub(crate) fn variant(&self) -> &LocalTermExpectation {
        &self.expectation
    }

    pub(crate) fn resolve_progress(&self) -> &LocalTermExpectationResolveProgress {
        &self.resolve_progress
    }

    pub fn original_error(&self) -> Option<&OriginalLocalTermExpectationError> {
        match self.resolve_progress {
            LocalTermExpectationResolveProgress::Resolved(Err(
                LocalTermExpectationError::Original(ref e),
            )) => Some(e),
            _ => None,
        }
    }

    pub(crate) fn expectee(&self) -> LocalTerm {
        self.expectee
    }

    pub fn src_expr_idx(&self) -> ExprIdx {
        self.src_expr_idx
    }

    pub(crate) fn set_resolved(
        &mut self,
        result: LocalTermExpectationResult<LocalTermExpectationResolvedOk>,
    ) {
        self.resolve_progress = LocalTermExpectationResolveProgress::Resolved(result)
    }
}

pub(super) struct LocalTermExpectationEffect {
    pub(super) result: LocalTermExpectationResult<LocalTermExpectationResolvedOk>,
    pub(super) actions: Vec<TermResolveAction>,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct LocalTermExpectations {
    arena: Arena<LocalTermExpectationEntry>,
    first_unresolved_expectation: usize,
}

impl std::ops::Index<LocalTermExpectationIdx> for LocalTermExpectations {
    type Output = LocalTermExpectationEntry;

    fn index(&self, index: LocalTermExpectationIdx) -> &Self::Output {
        &self.arena[index]
    }
}

impl LocalTermExpectations {
    pub(super) fn unresolved_rule_iter(
        &self,
    ) -> impl Iterator<Item = (LocalTermExpectationIdx, &LocalTermExpectationEntry)> {
        self.arena
            .indexed_iter_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub fn iter(&self) -> impl Iterator<Item = &LocalTermExpectationEntry> {
        self.arena.iter()
    }

    pub(super) fn unresolved_indexed_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (LocalTermExpectationIdx, &mut LocalTermExpectationEntry)> {
        self.arena
            .indexed_iter_mut_with_start(self.first_unresolved_expectation)
            .filter(|(_, rule)| match rule.resolve_progress() {
                LocalTermExpectationResolveProgress::Unresolved => true,
                LocalTermExpectationResolveProgress::Resolved(_) => false,
            })
    }

    pub(super) fn alloc_rule(
        &mut self,
        rule: LocalTermExpectationEntry,
    ) -> LocalTermExpectationIdx {
        self.arena.alloc_one(rule)
    }

    pub(super) fn take_effect(
        &mut self,
        rule_idx: LocalTermExpectationIdx,
        effect: LocalTermExpectationEffect,
    ) -> Option<Vec<TermResolveAction>> {
        self.arena
            .update(rule_idx, |rule| rule.set_resolved(effect.result));
        Some(effect.actions)
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(crate) fn add_expectation_rule(
        &mut self,
        src_expr_idx: ExprIdx,
        target: LocalTerm,
        expectation: impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
    ) -> OptionLocalTermExpectationIdx {
        local_term_region
            .expectations
            .alloc_rule(LocalTermExpectationEntry {
                src_expr_idx,
                expectee: target.into(),
                expectation: expectation.into(),
                resolve_progress: LocalTermExpectationResolveProgress::Unresolved,
            })
            .into()
    }
    pub(super) fn resolve_expectation(
        &self,
        rule: &LocalTermExpectationEntry,
        level: LocalTermResolveLevel,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match rule.expectation {
            LocalTermExpectation::ExplicitlyConvertible(ref exp) => self
                .resolve_explicitly_convertible(
                    rule.expectee,
                    exp.destination,
                    level,
                    unresolved_terms,
                ),
            LocalTermExpectation::ImplicitlyConvertible(exp) => self
                .resolve_implicitly_convertible(
                    rule.expectee,
                    exp.destination,
                    level,
                    unresolved_terms,
                ),
            LocalTermExpectation::EqsSort { smallest_universe } => {
                self.resolve_eqs_sort_expectation(rule.expectee, smallest_universe)
            }
            LocalTermExpectation::FrameVariableType => todo!(),
            LocalTermExpectation::EqsRefMutApplication { lifetime } => self
                .resolve_eqs_ref_mut_application_expectation(
                    rule.expectee,
                    lifetime,
                    unresolved_terms,
                ),
            LocalTermExpectation::EqsFunctionType(_) => {
                self.resolve_eqs_function_ty(rule.src_expr_idx, rule.expectee, unresolved_terms)
            }
            LocalTermExpectation::InsSort { smallest_universe } => self
                .resolve_ins_sort_expectation(smallest_universe, rule.expectee, unresolved_terms),
            LocalTermExpectation::EqsExactly { destination } => {
                self.resolve_eqs_exactly(rule.expectee, destination, unresolved_terms)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum LocalTermExpectation {
    ExplicitlyConvertible(ExpectExplicitlyConvertible),
    ImplicitlyConvertible(ExpectImplicitlyConvertible),
    /// expect term to be an instance of Type u for some universe
    InsSort {
        smallest_universe: TermUniverse,
    },
    EqsSort {
        smallest_universe: TermUniverse,
    },
    FrameVariableType,
    EqsRefMutApplication {
        lifetime: UnresolvedTermIdx,
    },
    EqsExactly {
        destination: LocalTerm,
    },
    EqsFunctionType(ExpectEqsFunctionType),
}
