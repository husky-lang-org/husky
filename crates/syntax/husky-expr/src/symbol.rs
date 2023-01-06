use crate::*;
use husky_entity_tree::CratePrelude;

pub struct SymbolSheet<'a> {
    crate_prelude: CratePrelude<'a>,
    variable_sheet: VariableSheet,
}

impl<'a> SymbolSheet<'a> {
    pub fn new(crate_prelude: CratePrelude<'a>) -> Self {
        Self {
            crate_prelude,
            variable_sheet: Default::default(),
        }
    }

    pub(crate) fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol> {
        // ad hoc
        if let Some(variable) = self.variable_sheet.resolve_ident(token_idx, ident) {
            Some(Symbol::Variable(variable))
        } else if let Some(entity_symbol) = self.crate_prelude.resolve_ident(ident) {
            Some(Symbol::Entity(entity_symbol.entity_path()))
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) fn define_variables(&mut self, variables: Vec<Variable>) -> ArenaIdxRange<Variable> {
        self.variable_sheet.define_variables(variables)
    }

    pub fn variable_sheet(self) -> VariableSheet {
        self.variable_sheet
    }
}

pub enum Symbol {
    Variable(VariableIdx),
    Entity(EntityPath),
}
