use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_binary_assign_closed_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        lopd: SynExprIdx,
        opr: BinaryClosedOpr,
        ropd: SynExprIdx,
    ) -> Result<FluffyTerm, ExprTypeError> {
        // let expr_eval_lifetime = self
        //     .fluffy_term_region
        //     .new_implicit_symbol(expr_idx, ImplicitSymbolVariant::ExprEvalLifetime);
        match self.infer_new_expr_ty(lopd, ExpectAnyOriginal) {
            Some(lopd_ty) => {
                let (lopd_place, lopd_base_ty) = lopd_ty.ty_data(self);
                match lopd_place {
                    Some(lopd_place) => match lopd_place {
                        Place::Const => todo!(),
                        Place::StackPure { location } => todo!(),
                        Place::ImmutableStackOwned { location } => todo!(),
                        Place::MutableStackOwned { .. } => (),
                        Place::Transient => todo!(),
                        Place::Ref { guard } => todo!(),
                        Place::RefMut { guard } => todo!(),
                        Place::Leashed => todo!(),
                        Place::Todo => todo!(),
                    },
                    None => todo!(),
                };
                let ropd_ty_expected: FluffyTerm = match lopd_base_ty {
                    FluffyBaseTypeData::TypeOntology {
                        ty_path,
                        refined_ty_path,
                        ty_arguments,
                        ty_ethereal_term,
                    } => match ty_ethereal_term {
                        Some(ty_ethereal_term) => ty_ethereal_term.into(),
                        None => todo!(),
                    },
                    FluffyBaseTypeData::Curry {
                        curry_kind,
                        variance,
                        parameter_variable,
                        parameter_ty,
                        return_ty,
                        ty_ethereal_term,
                    } => todo!(),
                    FluffyBaseTypeData::Hole(_, _) => todo!(),
                    FluffyBaseTypeData::Category(_) => todo!(),
                    FluffyBaseTypeData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys,
                        return_ty,
                    } => todo!(),
                    FluffyBaseTypeData::Symbol { term } => todo!(),
                };
                self.infer_new_expr_ty_discarded(
                    ropd,
                    ExpectCoersion::new(Contract::Move, ropd_ty_expected),
                )
            }
            None => self.infer_new_expr_ty_discarded(ropd, ExpectAnyDerived),
        };
        Ok(self.term_menu.unit_ty_ontology().into())
    }
}
