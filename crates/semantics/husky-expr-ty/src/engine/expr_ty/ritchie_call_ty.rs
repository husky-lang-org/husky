use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_ritchie_call_nonself_arguments_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        nonself_parameter_contracted_tys: &[FluffyTermRitchieParameter],
        nonself_arguments: impl Iterator<Item = CallListItem>,
    ) {
        // if nonself_parameter_contracted_tys.len() != nonself_arguments.len() {
        //     self.add_expr_ty_error(
        //         expr_idx,
        //         OriginalExprTypeError::RitchieCallWrongNumberOfArguments {
        //             number_of_nonself_parameters: nonself_parameter_contracted_tys.len() as u8,
        //             number_of_nonself_arguments: nonself_arguments.len() as u8,
        //         },
        //     )
        // }
        for (i, nonself_argument) in nonself_arguments.into_iter().enumerate() {
            if i < nonself_parameter_contracted_tys.len() {
                let nonself_parameter_contracted_ty = nonself_parameter_contracted_tys[i];
                match nonself_parameter_contracted_ty.kind() {
                    FluffyExplicitParameterKind::Regular => (),
                    FluffyExplicitParameterKind::Keyed { ident } => todo!(),
                }
                self.infer_new_expr_ty_discarded(
                    nonself_argument.argument_expr_idx(),
                    ExpectImplicitlyConvertible::new(nonself_parameter_contracted_ty),
                );
            } else {
                self.infer_new_expr_ty_discarded(
                    nonself_argument.argument_expr_idx(),
                    ExpectAnyDerived,
                );
            }
        }
    }
}
