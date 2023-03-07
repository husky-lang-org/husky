mod binary;
mod box_list;
mod explicit_application;
mod field;
mod literal;
mod method;
mod prefix;
mod ritchie_call_ty;
mod suffix;

use super::*;
use husky_opn_syntax::*;
use husky_ty_expectation::TypePathDisambiguation;

pub(crate) enum ExprTypeResolveProgress<E: ExpectLocalTerm> {
    Unresolved,
    Outcome(E::Outcome),
    ResolvedErr,
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_expr_ty_resolved(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
    ) -> Option<Term> {
        let ty = self.infer_new_expr_ty(expr_idx, expr_ty_expectation, local_term_region)?;
        match ty {
            LocalTerm::Resolved(ty) => Some(ty),
            LocalTerm::Unresolved(ty) => self.resolve_term(ty, local_term_region),
        }
    }

    pub(super) fn infer_new_expr_ty<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
        local_term_region: &mut LocalTermRegion,
    ) -> Option<LocalTerm> {
        let expectation_idx =
            self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation, local_term_region);

        match expectation_idx.into_option() {
            Some(expectation_idx) => local_term_region[expectation_idx]
                .resolve_progress()
                .resolved_ok::<E::Outcome>()
                .map(|ok| ok.destination()),
            None => None,
        }
    }

    pub(super) fn infer_new_expr_ty_discarded<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
        local_term_region: &mut LocalTermRegion,
    ) {
        self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation, local_term_region);
    }

    #[inline(always)]
    pub(super) fn infer_new_expr_ty_with_expectation_returned<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
        local_term_region: &mut LocalTermRegion,
    ) -> Option<E::Outcome>
    where
        E::Outcome: Clone,
    {
        let expectation_idx =
            self.infer_new_expr_ty_aux(expr_idx, expr_ty_expectation, local_term_region);
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Weak, local_term_region);
        let resolved_ok = match expectation_idx.into_option() {
            Some(expectation_idx) => local_term_region[expectation_idx]
                .resolve_progress()
                .resolved_ok()
                .cloned(),
            None => None,
        };
        resolved_ok
    }

    #[inline(always)]
    fn infer_new_expr_ty_aux<E: ExpectLocalTerm>(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: E,
        local_term_region: &mut LocalTermRegion,
    ) -> OptionLocalTermExpectationIdx {
        let ty_result = self.calc_expr_ty(expr_idx, &expr_ty_expectation, local_term_region);
        let expectation_idx = match ty_result {
            Ok((_, Ok(ty))) => {
                self.add_expectation_rule(expr_idx, ty, expr_ty_expectation, local_term_region)
            }
            _ => Default::default(),
        };
        self.save_new_expr_ty(expr_idx, ExprTypeInfo::new(ty_result, expectation_idx));
        self.resolve_as_much_as_possible(LocalTermResolveLevel::Weak, local_term_region);
        expectation_idx
    }

    fn save_new_expr_ty(&mut self, expr_idx: ExprIdx, info: ExprTypeInfo) {
        self.print_debug_expr(expr_idx);
        self.expr_ty_infos.insert_new(expr_idx, info)
    }

    fn calc_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        expr_ty_expectation: &impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        self.print_debug_expr(expr_idx);
        match self.expr_region_data[expr_idx] {
            Expr::Literal(literal_token_idx) => Ok((
                ExprDisambiguation::Trivial,
                self.calc_literal_expr_ty(
                    expr_idx,
                    literal_token_idx,
                    expr_ty_expectation,
                    local_term_region,
                ),
            )),
            Expr::EntityPath {
                entity_path_expr,
                path,
            } => self.calc_entity_path_expr_ty(path, expr_ty_expectation, local_term_region),
            Expr::InheritedSymbol {
                ident,
                inherited_symbol_idx,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                match self.inherited_symbol_tys.get(inherited_symbol_idx) {
                    Some(ty) => Ok((*ty).into()),
                    None => Err(DerivedExprTypeError::InheritedSymbolTypeError.into()),
                },
            )),
            Expr::CurrentSymbol {
                ident,
                current_symbol_idx,
                current_symbol_kind,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                self.current_symbol_tys
                    .get(current_symbol_idx)
                    .copied()
                    .ok_or(DerivedExprTypeError::CurrentSymbolTypeError.into()),
            )),
            Expr::FrameVarDecl {
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
                ..
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => Ok((
                ExprDisambiguation::Trivial,
                match self.self_ty {
                    Some(_) => todo!(),
                    None => Err(DerivedExprTypeError::SelfTypeNotInferredForSelfValue.into()),
                },
            )),
            Expr::Binary {
                lopd,
                opr,
                ropd,
                opr_token_idx,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_binary_expr_ty(expr_idx, lopd, opr, ropd, local_term_region),
            )),
            Expr::Be {
                src, ref target, ..
            } => {
                match self.infer_new_expr_ty(src, ExpectAnyOriginal, local_term_region) {
                    Some(src_ty) => match target {
                        Ok(target) => todo!(),
                        Err(_) => (),
                    },
                    None => (),
                };
                Ok((
                    ExprDisambiguation::Trivial,
                    Ok(self.term_menu.bool().into()),
                ))
            }
            Expr::Prefix { opr, opd, .. } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_prefix_expr_ty(opr, opd, local_term_region),
            )),
            Expr::Suffix { opd, opr, .. } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_suffix_expr_ty(opd, opr, local_term_region),
            )),
            Expr::ExplicitApplicationOrRitchieCall {
                function,
                ref implicit_arguments,
                ref items,
                ..
            } => self.calc_explicit_application_or_ritchie_call_expr_ty(
                function,
                expr_ty_expectation,
                local_term_region,
                implicit_arguments,
                items,
            ),
            Expr::Field {
                owner, ident_token, ..
            } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_field_expr_ty(owner, ident_token, local_term_region),
            )),
            Expr::MethodCall {
                self_argument,
                ident_token,
                ref implicit_arguments,
                nonself_arguments,
                ..
            } => Ok((
                ExprDisambiguation::Trivial,
                self.calc_method_expr_ty(
                    self_argument,
                    ident_token,
                    implicit_arguments.as_ref(),
                    nonself_arguments,
                    local_term_region,
                ),
            )),
            Expr::TemplateInstantiation {
                template,
                ref implicit_arguments,
            } => todo!(),
            Expr::ExplicitApplication { function, argument } => self.calc_explicit_application(
                function,
                argument,
                expr_ty_expectation
                    .final_destination(self.db(), local_term_region.unresolved_terms()),
                local_term_region,
            ),
            Expr::Bracketed { item, .. } => Ok((
                ExprDisambiguation::Trivial,
                self.infer_new_expr_ty(item, expr_ty_expectation.clone(), local_term_region)
                    .ok_or(DerivedExprTypeError::BracketedItemTypeError.into()),
            )),
            Expr::NewTuple { items, .. } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner,
                items: indices,
                ..
            } => self.calc_index_or_compose_with_list_expr_ty(
                expr_idx,
                owner,
                indices,
                local_term_region,
            ),
            Expr::List { items, .. } => {
                match expr_ty_expectation.disambiguate_ty_path(
                    self.db(),
                    local_term_region.unresolved_terms(),
                    self.entity_path_menu.list_ty_path(),
                ) {
                    TypePathDisambiguationResult::Ok(disambiguation) => Ok(match disambiguation {
                        TypePathDisambiguation::Ontology => {
                            // ad hoc, assume universe is 1
                            match items.len() {
                                0 => (
                                    ListExprDisambiguation::ListFunctor.into(),
                                    Ok(self.term_menu.ex_co_ty0_to_ty0().into()),
                                ),
                                _ => todo!(),
                            }
                        }
                        TypePathDisambiguation::Constructor => {
                            (ListExprDisambiguation::NewList.into(), todo!())
                        }
                    }),
                    TypePathDisambiguationResult::ErrDifferentTypePath {} => todo!(),
                    TypePathDisambiguationResult::ErrFromAnyOriginal => todo!(),
                    TypePathDisambiguationResult::ErrFromAnyDerived => {
                        Err(DerivedExprTypeError::AmbiguateListExpr.into())
                    }
                }
                // Ok(disambiguation) => match disambiguation {
                //     TypePathDisambiguation::TypeItselfOrTemplate => todo!(),
                //     TypePathDisambiguation::InstanceOrConstructor => todo!(),
                // },
                // Err(error) => {
                //     for item in items {
                //         self.infer_new_expr_ty(item, ExpectAnyDerived, local_term_region);
                //     }
                //     Err(error)
                // }
                // {
                //     Ok(disambiguation) => (
                //         match disambiguation {
                //             ListExprDisambiguation::NewList => {
                //                 self.calc_new_list_expr_ty(expr_idx, items, local_term_region)
                //             }
                //             ListExprDisambiguation::ListFunctor => todo!(),
                //         },
                //         Ok(disambiguation.into()),
                //     ),
                //     Err(error) => {
                //         for item in items {
                //             todo!()
                //         }
                //         (Err(todo!("derived")), Err(error))
                //     }
                // }
            }
            Expr::BoxColonList { .. } => todo!(),
            Expr::Block { stmts } => Ok((
                ExprDisambiguation::Trivial,
                self.infer_new_block(stmts, expr_ty_expectation.clone(), local_term_region)
                    .ok_or(DerivedExprTypeError::BlockTypeError.into()),
            )),
            Expr::Err(_) => Err(DerivedExprTypeError::ExprError.into()),
        }
    }

    fn calc_entity_path_expr_ty(
        &mut self,
        path: Option<EntityPath>,
        expr_ty_expectation: &impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        match path {
            None => Err(DerivedExprTypeError::EntityPathError.into()),
            Some(path) => match path {
                EntityPath::Module(_) => todo!(),
                EntityPath::ModuleItem(path) => match path {
                    ModuleItemPath::Type(ty_path) => {
                        self.calc_ty_path_expr_ty(expr_ty_expectation, ty_path, local_term_region)
                    }
                    ModuleItemPath::Trait(trai_path) => Ok((
                        ExprDisambiguation::Trivial,
                        self.db
                            .trai_path_ty_unchecked(trai_path)
                            .map(Into::into)
                            .map_err(|e| todo!()),
                    )),
                    ModuleItemPath::Form(form_path) => Ok((
                        ExprDisambiguation::Trivial,
                        self.db
                            .form_path_ty_unchecked(form_path)
                            .map(Into::into)
                            .map_err(Into::into),
                    )),
                },
                EntityPath::AssociatedItem(_) => todo!(),
                EntityPath::Variant(_) => todo!(),
            },
        }
    }

    fn calc_ty_path_expr_ty(
        &mut self,
        expr_ty_expectation: &impl ExpectLocalTerm,
        ty_path: TypePath,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        match expr_ty_expectation.disambiguate_ty_path(
            self.db(),
            local_term_region.unresolved_terms(),
            ty_path,
        ) {
            TypePathDisambiguationResult::Ok(disambiguation) => Ok((
                disambiguation.into(),
                self.db
                    .ty_path_ty(ty_path, disambiguation)
                    .map(Into::into)
                    .map_err(|e| e.into()),
            )),
            TypePathDisambiguationResult::ErrDifferentTypePath {} => todo!(),
            TypePathDisambiguationResult::ErrFromAnyOriginal => {
                Err(OriginalExprTypeError::AmbiguousTypePath.into())
            }
            TypePathDisambiguationResult::ErrFromAnyDerived => {
                Err(DerivedExprTypeError::AmbiguousTypePath.into())
            }
        }
    }

    fn calc_explicit_application_or_ritchie_call_expr_ty(
        &mut self,
        function: idx_arena::ArenaIdx<Expr>,
        expr_ty_expectation: &impl ExpectLocalTerm,
        local_term_region: &mut LocalTermRegion,
        implicit_arguments: &Option<ImplicitArgumentList>,
        items: &idx_arena::ArenaIdxRange<Expr>,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        let Some(expectation_ok) = self.infer_new_expr_ty_with_expectation_returned(
                function,
                ExpectEqsFunctionType::new(expr_ty_expectation.final_destination(self.db(), local_term_region.unresolved_terms())),
                local_term_region,
            ) else {
                for item in items {
                    self.infer_new_expr_ty(item, ExpectAnyDerived, local_term_region);
                }
                return
                    Err(
                        DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred
                            .into(),
                    )
            };
        if let Some(implicit_arguments) = implicit_arguments {
            todo!()
        }
        match expectation_ok.variant {
            ExpectEqsFunctionTypeOutcomeVariant::Ritchie {
                ritchie_kind,
                parameter_liasoned_tys,
            } => {
                self.calc_ritchie_call_arguments_expr_ty(
                    ritchie_kind,
                    parameter_liasoned_tys.to_vec(),
                    *items,
                    local_term_region,
                );
                Ok((
                    ExprDisambiguation::ExplicitApplicationOrRitchieCall(
                        ApplicationOrRitchieCallExprDisambiguation::RitchieCall,
                    ),
                    Ok(expectation_ok.return_ty),
                ))
            }
            ExpectEqsFunctionTypeOutcomeVariant::Curry { .. } => todo!(),
        }
    }

    fn calc_index_or_compose_with_list_expr_ty(
        &mut self,
        expr_idx: ExprIdx,
        owner: ExprIdx,
        indices: ExprIdxRange,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        let Some(owner_ty) = self.infer_new_expr_ty(owner, ExpectAnyOriginal, local_term_region)
        else  {
            for index in indices {
                self.infer_new_expr_ty(index, ExpectAnyDerived, local_term_region);
            }
            let e = DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred;
            return Err( DerivedExprTypeError::ApplicationOrRitchieCallFunctionTypeNotInferred.into()            )
        };
        todo!()
    }

    fn calc_explicit_application(
        &mut self,
        function: ExprIdx,
        argument: ExprIdx,
        final_destination: FinalDestination,
        local_term_region: &mut LocalTermRegion,
    ) -> ExprTypeResult<(ExprDisambiguation, ExprTypeResult<LocalTerm>)> {
        Ok((
            ExprDisambiguation::Trivial,
            self.calc_explicit_application_expr_ty(
                function,
                argument,
                final_destination,
                local_term_region,
            ),
        ))
    }
}
