use husky_ethereal_signature::helpers::trai_for_ty::{
    trai_path_for_ty_path_impl_block_ethereal_signature_templates,
    trai_path_for_ty_term_impl_block_ethereal_signature_builders,
};
use salsa::DisplayWithDb;
use vec_like::SmallVecPairMap;

use crate::method_fn::MethodFnFluffySignature;

use super::*;

impl HasFluffyTraitMethodDispatch for EtherealTerm {
    fn trai_method_dispatch_aux(
        self,
        engine: &mut impl FluffyTermEngine,
        expr_idx: SynExprIdx,
        ident_token: IdentRegionalToken,
        trai_item_records: TraitInUseItemsWithGivenIdent,
        mut indirections: FluffyIndirections,
    ) -> FluffyTermMaybeResult<FluffyMethodDynamicDispatch> {
        let db = engine.db();
        let mut matches: SmallVec<[(); 2]> = Default::default();
        let application_expansion = self.application_expansion(db);
        let arguments = application_expansion.arguments(db);
        let mut trai_path_selected: Option<TraitPath> = None;
        let mut matches_map: SmallVecPairMap<
            TraitPath,
            SmallVec<[TraitForTypeImplBlockEtherealSignatureBuilder; 2]>,
            2,
        > = Default::default();
        let TermFunctionReduced::TypeOntology(ty_path) = application_expansion.function() else {
            unreachable!()
        };
        for record in trai_item_records.records() {
            // todo: check scope
            let trai_path = record.trai_path();
            let mut builders =
                trai_path_for_ty_term_impl_block_ethereal_signature_builders(db, trai_path, self)?;
            if !builders.is_empty() {
                unsafe { matches_map.insert_new_unchecked((trai_path, builders)) }
            }
        }
        match matches_map.len() {
            0 => match ty_path.refine(db) {
                Left(PreludeTypePath::Indirection(prelude_indirection_ty_path)) => {
                    match prelude_indirection_ty_path {
                        PreludeIndirectionTypePath::Ref => todo!(),
                        PreludeIndirectionTypePath::RefMut => todo!(),
                        PreludeIndirectionTypePath::Leash => {
                            indirections.add(FluffyIndirection::Leash);
                            debug_assert_eq!(arguments.len(), 1);
                            let the_argument = arguments[0];
                            the_argument.trai_method_dispatch_aux(
                                engine,
                                expr_idx,
                                ident_token,
                                trai_item_records,
                                indirections,
                            )
                        }
                        PreludeIndirectionTypePath::At => todo!(),
                    }
                }
                Left(_) => Nothing,
                Right(_) => {
                    // todo: consider custom Deref Carrier etc
                    Nothing
                }
            },
            1 => {
                let (trai_path, ref matches) = matches_map.data()[0];
                match matches.len() {
                    0 => unreachable!(),
                    1 => {
                        let impl_block_signature_builder = matches[0];
                        // todo: check scope
                        let TraitForTypeItemEtherealSignatureBuilder::Method(
                            method_signature_builder,
                        ) = impl_block_signature_builder
                            .associated_item_ethereal_signature_template(db, ident_token.ident())?
                        else {
                            todo!()
                        };
                        match method_signature_builder.try_finish(db) {
                            Some(eth_sig) => JustOk(FluffyDynamicDispatch {
                                signature: MethodFnFluffySignature::from_ethereal(
                                    indirections.final_place(),
                                    eth_sig,
                                )
                                .into(),
                                indirections,
                            }),
                            None => todo!(),
                        }
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
