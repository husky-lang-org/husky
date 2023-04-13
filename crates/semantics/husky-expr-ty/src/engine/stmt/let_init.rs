use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_let_stmt(
        &mut self,
        let_variable_pattern: &ExprResult<LetVariablesPattern>,
        initial_value: &ExprResult<ExprIdx>,
    ) -> Option<FluffyTerm> {
        let pattern_ty = match let_variable_pattern {
            Ok(pattern) => match pattern.ty() {
                Some(ty) => {
                    self.infer_new_expr_ty_discarded(
                        ty,
                        ExpectEqsCategory::new_expect_eqs_ty_kind(),
                    );
                    self.infer_new_expr_term(ty)
                }
                None => None,
            },
            Err(_) => todo!(),
        };
        match pattern_ty {
            Some(pattern_ty) => {
                let contract = self.infer_new_pattern_contract(
                    let_variable_pattern
                        .as_ref()
                        .expect("must be okay")
                        .pattern_expr_idx(),
                );
                initial_value.as_ref().ok().copied().map(|initial_value| {
                    self.infer_new_expr_ty_discarded(
                        initial_value,
                        // ad hoc
                        ExpectImplicitlyConvertible::new(
                            FluffyTermRitchieParameterContractedType::new(todo!(), pattern_ty),
                        ),
                    )
                });
            }
            None => {
                initial_value.as_ref().copied().map(|initial_value| {
                    self.infer_new_expr_ty_discarded(
                        initial_value,
                        // ad hoc
                        ExpectAnyOriginal,
                    )
                });
            }
        }
        match pattern_ty {
            Some(ty) if ty == self.term_menu.never().into() => Some(self.term_menu.never().into()),
            Some(ty) => {
                match let_variable_pattern {
                    Ok(let_variable_pattern) => self.infer_pattern_and_symbols_ty(
                        let_variable_pattern.pattern_expr_idx(),
                        ty,
                        let_variable_pattern.variables(),
                    ),
                    Err(_) => todo!(),
                };
                Some(self.term_menu.unit_ty_ontology().into())
            }
            None => Some(self.term_menu.unit_ty_ontology().into()),
        }
    }
}
