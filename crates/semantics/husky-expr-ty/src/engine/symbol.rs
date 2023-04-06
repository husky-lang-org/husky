use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_all_parameter_symbols(&mut self) {
        self.infer_inherited_parameter_symbols();
        self.infer_current_parameter_symbols()
    }

    fn infer_inherited_parameter_symbols(&mut self) {
        let Some(parent) = self.expr_region_data.parent()
            else{
                return;
            };
        let parent_symbol_region = parent.data(self.db).symbol_region();
        for (inherited_symbol_idx, inherited_symbol) in self
            .expr_region_data
            .symbol_region()
            .indexed_inherited_symbol_iter()
        {
            if let Ok(qualified_ty) = self
                .signature_term_region
                .term_symbol_region()
                .inherited_symbol_term(inherited_symbol_idx)
                .qualified_ty(self.db)
            {
                todo!()
                // if let Ok(ty) = Term::ty_from_raw(self.db, ty) {
                //     self.inherited_symbol_tys
                //         .insert_new(inherited_symbol_idx, ty)
                // }
            }
        }
    }

    fn infer_current_parameter_symbols(&mut self) {
        for current_symbol_idx in self
            .expr_region_data
            .symbol_region()
            .current_symbol_index_iter()
        {
            let Some(current_symbol_term) = self
                .signature_term_region
                .term_symbol_region()
                .current_symbol_term(current_symbol_idx)
                else {
                    return
                };
            if let Ok(qualified_ty) = current_symbol_term.qualified_ty(self.db) {
                if let Ok(qualified_ty) = Term::ty_from_raw(self.db, qualified_ty.into()) {
                    self.current_symbol_qualified_tys
                        .insert_new(current_symbol_idx, qualified_ty.into())
                }
            }
        }
    }

    fn parameter_pattern_ty(&self, pattern_expr_idx: PatternExprIdx) -> Term {
        match self
            .expr_region_data
            .symbol_region()
            .regular_parameter_pattern_ty_constraint(pattern_expr_idx)
        {
            Some(_) => todo!(),
            None => todo!(),
        }
        todo!()
    }
}
