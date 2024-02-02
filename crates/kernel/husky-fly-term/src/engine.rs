use super::*;
use husky_entity_tree::helpers::TraitInUseItemsTable;
use husky_stack_location::{StackLocationIdx, StackLocationRegistry};

pub trait FlyTermEngine<'a>: Sized {
    fn db(&self) -> &'a ::salsa::Db;
    fn trai_in_use_items_table(&self) -> TraitInUseItemsTable<'a>;
    fn stack_location_registry_mut(&mut self) -> &mut StackLocationRegistry;
    fn issue_new_stack_location_idx(&mut self) -> StackLocationIdx {
        self.stack_location_registry_mut().issue_new()
    }
    fn fluffy_term_region(&self) -> &FlyTermRegion;
    fn fluffy_term_region_mut(&mut self) -> &mut FlyTermRegion;
    fn fluffy_terms(&self) -> &FlyTerms {
        self.fluffy_term_region().terms()
    }
    fn fluffy_terms_mut(&mut self) -> &mut FlyTerms {
        self.fluffy_term_region_mut().terms_mut()
    }
    fn item_path_menu(&self) -> &'a ItemPathMenu;
    fn term_menu(&self) -> &'a EthTermMenu;
    fn expr_region_data(&self) -> &'a SynExprRegionData;

    fn new_ty_ontology_application(
        &mut self,
        src: HoleSource,
        path: TypePath,
        arguments: SmallVec<[FlyTerm; 2]>,
    ) -> FlyTerm {
        todo!()
    }

    fn new_hole(&mut self, src: impl Into<HoleSource>, hole_kind: HoleKind) -> FlyTerm {
        HolTerm::new_hole(self, src, hole_kind).into()
    }

    fn synthesize_function_application_expr_ty(
        &mut self,
        variance: Variance,
        parameter_rune: Option<RuneFlyTerm>,
        parameter_ty: FlyTerm,
        return_ty: FlyTerm,
        argument_ty: FlyTerm,
        shift: i8,
    ) -> FlyTermResult<FlyTerm> {
        debug_assert!(shift >= 0);
        if shift == 0 {
            return Ok(return_ty);
        }
        debug_assert_eq!(parameter_ty.place(), None);
        debug_assert_eq!(return_ty.place(), None);
        match (
            parameter_rune.map(|rune| rune.base_resolved(self)),
            parameter_ty.base_resolved(self),
            return_ty.base_resolved(self),
            argument_ty.base_resolved(self),
        ) {
            (
                None,
                FlyTermBase::Eth(parameter_ty),
                FlyTermBase::Eth(return_ty),
                FlyTermBase::Eth(argument_ty),
            ) => {
                return Ok(EthTerm::synthesize_function_application_expr_ty(
                    self.db(),
                    variance,
                    None,
                    parameter_ty,
                    return_ty,
                    argument_ty,
                    shift,
                )?
                .into())
            }
            _ => (),
        }
        if parameter_rune.is_some() {
            todo!()
        }
        match argument_ty.data(self) {
            FlyTermData::Curry {
                curry_kind: argument_curry_kind,
                variance: argument_variance,
                parameter_rune: argument_parameter_variable,
                parameter_ty: argument_parameter_ty,
                return_ty: argument_return_ty,
                ..
            } => {
                if argument_parameter_variable.is_some() {
                    todo!()
                }
                let expr_ty = self.synthesize_function_application_expr_ty(
                    variance,
                    parameter_rune,
                    parameter_ty,
                    return_ty,
                    argument_return_ty,
                    shift - 1,
                );
                todo!()
                // FlyTerm::new_curry()
            }
            _ => unreachable!(),
        }
    }

    fn path(&self) -> String {
        format!("{:?}", self.expr_region_data().path().debug(self.db()))
    }

    fn add_expectation(
        &mut self,
        src: ExpectationSource,
        expectee: FlyTerm,
        expectation: impl Into<Expectation>,
    ) -> (FlyTermExpectationIdx, FlyTerm) {
        let db = self.db();
        self.fluffy_term_region_mut()
            .add_expectation(src, expectee, expectation, db)
    }
}

pub trait FlyTermEngineMut<'a>: FlyTermEngine<'a> {}

struct A {
    x: i32,
    y: i32,
}