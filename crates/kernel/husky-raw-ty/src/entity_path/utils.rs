use vec_like::{VecPairMap, VecSet};

use super::*;

pub(super) fn curry_from_implicit_parameters(
    db: &dyn RawTypeDb,
    term_curry_kind: CurryKind,
    variances: &[Variance],
    implicit_parameters: &[ImplicitParameterSignature],
    term: impl Into<RawTerm>,
) -> RawTerm {
    let mut term = term.into();
    debug_assert_eq!(variances.len(), implicit_parameters.len());
    for (variance, implicit_parameter) in
        std::iter::zip(variances.iter(), implicit_parameters.iter()).rev()
    {
        let symbol = implicit_parameter.symbol();
        assert_eq!(symbol.ty(db), Ok(implicit_parameter.ty()));
        term = {
            let (term, variable) = term.turn_symbol_into_variable(db, symbol);
            RawTermCurry::new(
                db,
                term_curry_kind,
                *variance,
                variable,
                implicit_parameter.ty(),
                term,
            )
        }
        .into()
    }
    term
}
