// todo: move this to husky-dec-term
mod error;
mod variable;

pub use self::error::*;
pub use self::variable::*;

pub(crate) use engine::*;

use crate::*;

use husky_entity_tree::SynNodeRegionPath;
use husky_syn_expr::{
    CurrentVariableIdx, SynExprIdx, SynExprMap, SynPatternMap, SynPatternSymbolMap,
};

/// preparation for generating signature
///
/// contains terms, symbols and liasons
///
/// should contains term information enough for generating declarations
#[derive(Debug, PartialEq, Eq)]
pub struct SynExprDecTermRegion {
    path: SynNodeRegionPath,
    symbolic_variable_region: DecSymbolicVariableRegion,
    expr_terms: SynExprMap<SynExprDecTermResult<DecTerm>>,
    pattern_expr_ty_infos: SynPatternMap<PatternExprDeclarativeTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<DecPatternVariableTypeInfo>,
}

impl SynExprDecTermRegion {
    pub(crate) fn new(
        path: SynNodeRegionPath,
        symbolic_variable_region: DecSymbolicVariableRegion,
        expr_terms: SynExprMap<SynExprDecTermResult<DecTerm>>,
        pattern_expr_ty_infos: SynPatternMap<PatternExprDeclarativeTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<DecPatternVariableTypeInfo>,
    ) -> Self {
        Self {
            path,
            symbolic_variable_region,
            expr_terms,
            pattern_expr_ty_infos,
            pattern_symbol_ty_infos,
        }
    }

    pub fn symbolic_variable_region(&self) -> &DecSymbolicVariableRegion {
        &self.symbolic_variable_region
    }

    pub fn current_variable_signature(
        &self,
        current_variable_idx: CurrentVariableIdx,
    ) -> Option<DecSymbolicVariableSignature> {
        self.symbolic_variable_region
            .current_parameter_variable_signature(current_variable_idx)
    }

    pub fn expr_term(&self, expr: SynExprIdx) -> SynExprDecTermResultRef<DecTerm> {
        self.expr_terms[expr].as_ref().copied()
    }

    pub fn path(&self) -> SynNodeRegionPath {
        self.path
    }
}
