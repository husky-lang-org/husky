use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn calc_prefix_expr_ty(
        &mut self,
        expr_idx: SynExprIdx,
        opr: PrefixOpr,
        opd: SynExprIdx,
        final_destination: FinalDestination,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<FluffyTerm>)> {
        match opr {
            PrefixOpr::Minus => {
                let opd_ty = self
                    .infer_new_expr_ty(opd, ExpectAnyOriginal)
                    .ok_or(DerivedExprTypeError::PrefixOperandTypeNotInferred)?;
                match opd_ty.data(self) {
                    FluffyTermData::Literal(_) => todo!(),
                    FluffyTermData::TypeOntology {
                        ty_path,
                        refined_ty_path,
                        arguments,
                        ty_ethereal_term,
                    } => match refined_ty_path {
                        Left(prelude_ty_path) => match prelude_ty_path {
                            PreludeTypePath::Basic(_) => todo!(),
                            PreludeTypePath::Num(num_ty_path) => match num_ty_path {
                                PreludeNumTypePath::Int(_) | PreludeNumTypePath::Float(_) => {
                                    Ok((ExprDisambiguation::Trivial, Ok(opd_ty)))
                                }
                            },
                            PreludeTypePath::Borrow(_) => todo!(),
                            PreludeTypePath::Nat => todo!(),
                            PreludeTypePath::Lifetime => todo!(),
                            PreludeTypePath::Module => todo!(),
                            PreludeTypePath::Trait => todo!(),
                            PreludeTypePath::List => todo!(),
                            PreludeTypePath::Array => todo!(),
                            PreludeTypePath::Array2d => todo!(),
                            PreludeTypePath::Array3d => todo!(),
                            PreludeTypePath::Array4d => todo!(),
                            PreludeTypePath::Array5d => todo!(),
                            PreludeTypePath::Slice => todo!(),
                            PreludeTypePath::StringLiteral => todo!(),
                            PreludeTypePath::Str => todo!(),
                            PreludeTypePath::Option => todo!(),
                            PreludeTypePath::Result => todo!(),
                        },
                        Right(_) => todo!(),
                    },
                    FluffyTermData::TypeOntologyAtPlace {
                        place,
                        ty_path: path,
                        refined_ty_path: refined_path,
                        arguments,
                        base_ty_ethereal_term,
                    } => todo!(),
                    FluffyTermData::Curry {
                        curry_kind,
                        variance,
                        parameter_variable,
                        parameter_ty,
                        return_ty,
                        ty_ethereal_term,
                    } => todo!(),
                    FluffyTermData::Hole(hole_kind, _) => match hole_kind {
                        HoleKind::UnspecifiedIntegerType | HoleKind::UnspecifiedFloatType => {
                            Ok((ExprDisambiguation::Trivial, Ok(opd_ty)))
                        }
                        HoleKind::ImplicitType => todo!(),
                        HoleKind::Any => todo!(),
                    },
                    FluffyTermData::Category(_) => todo!(),
                    FluffyTermData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys,
                        return_ty,
                        ..
                    } => todo!(),
                    FluffyTermData::HoleAtPlace {
                        place,
                        hole_kind,
                        hole,
                    } => todo!(),
                    FluffyTermData::Symbol { .. } => todo!(),
                    FluffyTermData::SymbolAtPlace { .. } => todo!(),
                    FluffyTermData::Variable { ty } => todo!(),
                    FluffyTermData::TypeVariant { path } => todo!(),
                }
            }
            PrefixOpr::Not => {
                self.infer_new_expr_ty_discarded(opd, self.expect_argument_ty_bool());
                // here we differs from Rust, but agrees with C
                Ok((
                    ExprDisambiguation::Trivial,
                    Ok(self.term_menu.bool_ty_ontology().into()),
                ))
            }
            PrefixOpr::Tilde => match final_destination {
                FinalDestination::Sort => Ok((
                    ExprDisambiguation::Tilde(TildeDisambiguation::Leash),
                    self.calc_function_application_expr_ty_aux(
                        expr_idx,
                        Variance::Covariant,
                        None,
                        self.term_menu.ty0().into(),
                        self.term_menu.ty0().into(),
                        opd,
                    ),
                )),
                FinalDestination::TypeOntology
                | FinalDestination::AnyOriginal
                | FinalDestination::AnyDerived => Ok((
                    ExprDisambiguation::Tilde(TildeDisambiguation::BitNot),
                    self.calc_bitnot_expr_ty(opd),
                )),
                FinalDestination::Ritchie(_) => todo!(),
            },
            PrefixOpr::Ref => {
                self.infer_new_expr_ty_discarded(opd, self.expect_ty0_subtype());
                // Should consider more cases, could also be taking references
                Ok((ExprDisambiguation::Trivial, Ok(self.term_menu.ty0().into())))
            }
            PrefixOpr::Vector => todo!(),
            PrefixOpr::Slice => todo!(),
            PrefixOpr::CyclicSlice => todo!(),
            PrefixOpr::Array(_) => todo!(),
            PrefixOpr::Option => {
                // todo!("consider universe");
                self.infer_new_expr_ty_discarded(opd, self.expect_ty0_subtype());
                Ok((ExprDisambiguation::Trivial, Ok(self.term_menu.ty0().into())))
            }
        }
    }

    fn calc_bitnot_expr_ty(&mut self, opd: SynExprIdx) -> ExprTypeResult<FluffyTerm> {
        let Some(ty) = self.infer_new_expr_ty(
            opd,
            self.expect_ty0_subtype(),
        ) else {
            Err(DerivedExprTypeError::BitNotOperandTypeNotInferred)?
        };
        match ty.data(self) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                arguments,
                ..
            } => todo!(),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_variable,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => todo!(),
            FluffyTermData::TypeOntologyAtPlace { .. } => todo!(),
            FluffyTermData::HoleAtPlace {
                place,
                hole_kind,
                hole,
            } => todo!(),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::SymbolAtPlace { .. } => todo!(),
            FluffyTermData::Variable { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
