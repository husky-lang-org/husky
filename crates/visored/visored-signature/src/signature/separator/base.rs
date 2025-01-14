use super::*;
use visored_entity_path::path::{trai_item::VdTraitItemPath, VdItemPath};
use visored_mir_opr::separator::VdMirBaseSeparator;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct VdBaseSeparatorSignature {
    instantiation: VdInstantiation,
    // TODO: replace this with something more ethereal
    opr: VdMirBaseSeparator,
    item_ty: VdType,
    expr_ty: VdType,
}

impl From<VdBaseSeparatorSignature> for VdSignature {
    fn from(signature: VdBaseSeparatorSignature) -> Self {
        VdSignature::Separator(signature.into())
    }
}

impl VdBaseSeparatorSignature {
    pub fn new(instantiation: VdInstantiation, item_ty: VdType, expr_ty: VdType) -> Self {
        let VdItemPath::TraitItem(path) = instantiation.path() else {
            unreachable!()
        };
        let opr = match path {
            VdTraitItemPath::GroupMul => todo!(),
            VdTraitItemPath::AbelianGroupAdd => todo!(),
            VdTraitItemPath::NatAdd => VdMirBaseSeparator::COMM_RING_ADD,
            VdTraitItemPath::NatMul => VdMirBaseSeparator::COMM_RING_MUL,
            VdTraitItemPath::CommRingAdd => VdMirBaseSeparator::COMM_RING_ADD,
            VdTraitItemPath::CommRingSub => todo!(),
            VdTraitItemPath::CommRingMul => VdMirBaseSeparator::COMM_RING_MUL,
            VdTraitItemPath::CommRingPower => todo!(),
            VdTraitItemPath::CommRingPos => todo!(),
            VdTraitItemPath::CommRingNeg => todo!(),
            VdTraitItemPath::Eq => VdMirBaseSeparator::EQ,
            VdTraitItemPath::Ne => VdMirBaseSeparator::NE,
            VdTraitItemPath::Lt => VdMirBaseSeparator::LT,
            VdTraitItemPath::Gt => VdMirBaseSeparator::GT,
            VdTraitItemPath::Le => VdMirBaseSeparator::LE,
            VdTraitItemPath::Ge => VdMirBaseSeparator::GE,
            VdTraitItemPath::FieldDiv => todo!(),
            VdTraitItemPath::RealSqrt => todo!(),
        };
        Self {
            instantiation,
            opr,
            item_ty,
            expr_ty,
        }
    }
}

impl VdBaseSeparatorSignature {
    pub fn instantiation(self) -> VdInstantiation {
        self.instantiation
    }

    pub fn opr(self) -> VdMirBaseSeparator {
        self.opr
    }

    pub fn item_ty(self) -> VdType {
        self.item_ty
    }

    pub fn expr_ty(self) -> VdType {
        self.expr_ty
    }
}
