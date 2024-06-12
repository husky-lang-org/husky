mod ethereal;
mod hollow;
mod solid;

pub(crate) use self::ethereal::*;

use super::*;
use crate::dispatch::instance::method::HasFlyMethodDispatch;
use husky_coword::Ident;
use husky_entity_path::path::major_item::{trai::TraitPath, ty::TypePath};
use husky_eth_signature::signature::package::PackageEthSignatureData;

pub type FlyFieldDyanmicDispatch = FlyInstanceDispatch<FieldFlySignature>;

use super::*;
use crate::quary::FlyQuary;
use husky_entity_path::path::assoc_item::AssocItemPath;
use husky_eth_signature::signature::{
    assoc_item::ty_item::memo_field::TypeMemoizedFieldEtherealSignature,
    major_item::ty::PropsFieldEtherealSignature,
};

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FieldFlySignature {
    PropsStruct {
        ty: FlyTerm,
    },
    Memoized {
        ty: FlyTerm,
        path: AssocItemPath,
        instantiation: FlyInstantiation,
    },
}

impl FieldFlySignature {
    pub fn return_ty(&self) -> FlyTerm {
        match *self {
            FieldFlySignature::PropsStruct { ty } => ty,
            FieldFlySignature::Memoized { ty, .. } => ty,
        }
    }
}

impl IsInstanceItemFlySignature for FieldFlySignature {
    fn expr_ty(&self, self_value_final_place: FlyQuary) -> FlyTermResult<FlyTerm> {
        todo!()
    }
}

impl From<PropsFieldEtherealSignature> for FieldFlySignature {
    fn from(signature: PropsFieldEtherealSignature) -> Self {
        match signature {
            PropsFieldEtherealSignature::PropsStruct(signature) => FieldFlySignature::PropsStruct {
                ty: signature.ty().into(),
            },
        }
    }
}

impl From<TypeMemoizedFieldEtherealSignature> for FieldFlySignature {
    fn from(signature: TypeMemoizedFieldEtherealSignature) -> Self {
        FieldFlySignature::Memoized {
            // ad hoc
            ty: signature.return_ty().into(),
            path: signature.path().into(),
            instantiation: FlyInstantiation::from_eth(
                FlyInstantiationEnvironment::MemoizedField,
                signature.instantiation(),
            ),
        }
    }
}

impl FlyTerm {
    /// returns None if no such field
    pub fn field_dispatch(
        self,
        engine: &mut impl FlyTermEngineMut,
        ident: Ident,
        available_traits: &[TraitPath],
    ) -> FlyTermMaybeResult<FlyFieldDyanmicDispatch> {
        self.field_dispatch_aux(
            engine,
            ident,
            available_traits,
            FlyIndirections::new(self.initial_place()),
        )
    }

    fn field_dispatch_aux<'db>(
        self,
        engine: &mut impl FlyTermEngineMut,
        ident: Ident,
        available_traits: &[TraitPath],
        indirections: FlyIndirections,
    ) -> FlyTermMaybeResult<FlyFieldDyanmicDispatch> {
        match self.base_resolved(engine) {
            FlyTermBase::Eth(term) => {
                ethereal_ty_field_dispatch(engine.db(), term, ident, indirections, engine.context())
            }
            FlyTermBase::Sol(term) => {
                term.field_dispatch_aux(engine, ident, available_traits, indirections)
            }
            FlyTermBase::Hol(term) => todo!(),
            FlyTermBase::Place => todo!(),
        }
    }
}
