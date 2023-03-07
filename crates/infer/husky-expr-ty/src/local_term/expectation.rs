mod any_derived;
mod any_original;
mod eqs_category;
mod eqs_eqs_function_ty;
mod eqs_exactly;
mod eqs_ref_mut_application;
mod explicitly_convertible;
mod implicitly_convertible;
mod ins_sort;

pub(crate) use self::any_derived::*;
pub(crate) use self::any_original::*;
pub(crate) use self::eqs_category::*;
pub(crate) use self::eqs_eqs_function_ty::*;
pub(crate) use self::eqs_exactly::*;
pub(crate) use self::eqs_ref_mut_application::*;
pub(crate) use self::explicitly_convertible::*;
pub(crate) use self::implicitly_convertible::*;
pub(crate) use self::ins_sort::*;

use super::*;
use husky_print_utils::p;
use husky_ty_expectation::TypePathDisambiguation;
use idx_arena::Arena;
use thiserror::Error;

pub(crate) trait ExpectLocalTerm: Into<LocalTermExpectation> + Clone {
    type Outcome: ExpectLocalTermOutcome;

    fn destination(&self) -> Option<LocalTerm>;

    /// final destination of `A1 -> ... -> An` is equal to that of `An`
    ///
    /// final destination of `A1 ... An` is equal to that of `A1`
    ///
    /// final destination of `Sort` is `FinalDestination::Sort`
    ///
    /// final destination of a type path `A` is `FinalDestination::TypePath(A)`
    #[inline(always)]
    fn final_destination(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> FinalDestination;

    /// if ty_path's type is under this expectation, disambiguate whether it's an ontology or constructor
    // final
    #[inline(always)]
    fn disambiguate_ty_path(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
        ty_path: TypePath,
    ) -> TypePathDisambiguationResult {
        match self.final_destination(db, unresolved_terms) {
            FinalDestination::Sort => TypePathDisambiguation::Ontology.into(),
            FinalDestination::TypePath(final_destination_ty_path) => {
                match final_destination_ty_path == ty_path {
                    true => TypePathDisambiguation::Constructor.into(),
                    false => todo!(),
                }
            }
            FinalDestination::AnyOriginal => TypePathDisambiguationResult::ErrFromAnyOriginal,
            FinalDestination::AnyDerived => TypePathDisambiguationResult::ErrFromAnyDerived,
        }
        // LocalTerm::Resolved(term) if let Term::Category(_) = term  => todo!(),
        // LocalTerm::Resolved(term) if term == ty_path.into() => todo!(),
        // _ => Err(todo!()),
    }
}

/// final destination of `A1 -> ... -> An` is equal to that of `An`
///
/// final destination of `A1 ... An` is equal to that of `A1`
///
/// final destination of `Sort` is `FinalDestination::Sort`
///
/// final destination of a type path `A` is `FinalDestination::TypePath(A)`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum FinalDestination {
    Sort,
    TypePath(TypePath),
    AnyOriginal,
    AnyDerived,
}

/// disambiguate between type itself (or template) and its instance or constructor
#[enum_class::from_variants]
pub enum TypePathDisambiguationResult {
    Ok(TypePathDisambiguation),
    ErrDifferentTypePath {},
    ErrFromAnyOriginal,
    ErrFromAnyDerived,
}

pub(crate) trait ExpectLocalTermOutcome: Into<LocalTermExpectationOutcome> {
    fn destination(&self) -> LocalTerm;

    /// will panic if not right
    fn downcast_ref(resolved_ok: &LocalTermExpectationOutcome) -> &Self;
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
#[enum_class::from_variants]
pub(crate) enum LocalTermExpectationOutcome {
    ExplicitlyConvertible(ExpectExplicitlyConvertibleOutcome),
    ImplicitlyConvertible(ExpectImplicitlyConvertibleOutcome),
    InsSort(ExpectInsSortOutcome),
    EqsSort(ExpectEqsSortOutcome),
    EqsExactly(ExpectEqsExactlyOutcome),
    EqsRefMutApplication(ExpectEqsRefMutApplicationOutcome),
    EqsRitchieCallType(ExpectEqsFunctionTypeOutcome),
}

impl LocalTermExpectationOutcome {
    fn resolved(&self) -> Option<Term> {
        match self {
            LocalTermExpectationOutcome::ExplicitlyConvertible(_) => todo!(),
            LocalTermExpectationOutcome::ImplicitlyConvertible(_) => todo!(),
            LocalTermExpectationOutcome::InsSort(result) => result.resolved(),
            LocalTermExpectationOutcome::EqsSort(_) => todo!(),
            LocalTermExpectationOutcome::EqsExactly(result) => result.resolved(),
            LocalTermExpectationOutcome::EqsRefMutApplication(_) => todo!(),
            LocalTermExpectationOutcome::EqsRitchieCallType(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) enum LocalTermExpectationResolveProgress {
    Unresolved,
    Resolved(LocalTermExpectationResult<LocalTermExpectationOutcome>),
}

impl LocalTermExpectationResolveProgress {
    pub(crate) fn resolved_ok<R: ExpectLocalTermOutcome>(&self) -> Option<&R> {
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
    #[error("todo")]
    Todo,
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub enum DerivedLocalTermExpectationError {
    #[error("{term:?} {error}")]
    TermTypeError { term: Term, error: TermError },
    #[error("{0}")]
    Type(#[from] TermError),
    #[error("target substitution failure")]
    TargetSubstitutionFailure,
    #[error("duplication")]
    Duplication(LocalTermExpectationIdx),
    #[error("unresolved local term")]
    UnresolvedLocalTerm,
    #[error("type path {ty_path:?} type error {error}")]
    TypePathTypeError { ty_path: TypePath, error: TermError },
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

    pub(crate) fn reduced_term(&self) -> Option<Term> {
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
        result: LocalTermExpectationResult<LocalTermExpectationOutcome>,
    ) {
        self.resolve_progress = LocalTermExpectationResolveProgress::Resolved(result)
    }
}

pub(super) struct LocalTermExpectationEffect {
    pub(super) result: LocalTermExpectationResult<LocalTermExpectationOutcome>,
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
            LocalTermExpectation::ExplicitlyConvertible(ref expectation) => self
                .resolve_explicitly_convertible(
                    rule.expectee,
                    expectation,
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
            LocalTermExpectation::EqsSort(ref expectation) => {
                self.resolve_eqs_sort_expectation(rule.expectee, expectation, unresolved_terms)
            }
            LocalTermExpectation::FrameVariableType => todo!(),
            LocalTermExpectation::EqsRefMutApplication(ref expectation) => self
                .resolve_eqs_ref_mut_application_expectation(
                    rule.expectee,
                    expectation,
                    unresolved_terms,
                ),
            LocalTermExpectation::EqsFunctionType(ref expectation) => {
                self.resolve_eqs_function_ty(rule.src_expr_idx, rule.expectee, unresolved_terms)
            }
            LocalTermExpectation::InsSort(ref expectation) => {
                self.resolve_ins_sort_expectation(rule.expectee, expectation, unresolved_terms)
            }
            LocalTermExpectation::EqsExactly(ref expectation) => {
                self.resolve_eqs_exactly(rule.expectee, expectation, unresolved_terms)
            }
            LocalTermExpectation::AnyOriginal(_) => None,
            LocalTermExpectation::AnyDerived(_) => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
#[enum_class::from_variants]
pub(crate) enum LocalTermExpectation {
    ExplicitlyConvertible(ExpectExplicitlyConvertible),
    ImplicitlyConvertible(ExpectImplicitlyConvertible),
    /// expect term to be an instance of Type u for some universe
    InsSort(ExpectInsSort),
    EqsSort(ExpectEqsCategory),
    FrameVariableType,
    EqsRefMutApplication(ExpectEqsRefMutApplication),
    EqsExactly(ExpectEqsExactly),
    EqsFunctionType(ExpectEqsFunctionType),
    AnyOriginal(ExpectAnyOriginal),
    AnyDerived(ExpectAnyDerived),
}
