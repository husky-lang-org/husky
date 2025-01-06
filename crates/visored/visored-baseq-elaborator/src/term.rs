pub mod builder;
pub mod inum;
mod num;
pub mod prop;
pub mod rnum;

use self::{inum::*, num::*, prop::*, rnum::*};
use crate::elaborator::VdBsqElaboratorInner;
use builder::{product::VdBsqProductBuilder, sum::VdBsqSumBuilder};
use either::*;
use floated_sequential::db::FloaterDb;
use floated_sequential::floated;
use num_relationship::VdBsqNumRelationshipPropTermKind;
use product::VdBsqProductInumTermBase;
use vec_like::ordered_small_vec_map::OrderedSmallVecPairMap;
use visored_mir_expr::{
    expr::{application::VdMirFunc, VdMirExprData, VdMirExprEntry},
    symbol::local_defn::{
        storage::VdMirSymbolLocalDefnStorage, VdMirSymbolLocalDefnHead, VdMirSymbolLocalDefnIdx,
    },
};
use visored_mir_opr::{opr::binary::VdMirBaseBinaryOpr, separator::VdMirBaseSeparator};
use visored_term::term::{literal::VdLiteralData, VdTermData};

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    Inum(VdBsqInumTerm<'sess>),
    Prop(VdBsqPropTerm<'sess>),
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn product_or_non_product(
        self,
    ) -> Either<(VdBsqRnumTerm, VdBsqProductInumTermBase<'sess>), VdBsqNonProductNumTerm<'sess>>
    {
        match self {
            VdBsqNumTerm::Rnum(term) => todo!(),
            VdBsqNumTerm::Inum(term) => match term {
                VdBsqInumTerm::Atom(term) => Right(VdBsqNonProductNumTerm::AtomInum(term)),
                VdBsqInumTerm::Sum(term) => Right(VdBsqNonProductNumTerm::SumInum(term)),
                VdBsqInumTerm::Product(rnum, term) => Left((rnum, term)),
            },
        }
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn num(self) -> Option<VdBsqNumTerm<'sess>> {
        match self {
            VdBsqTerm::Rnum(rnum) => Some(VdBsqNumTerm::Rnum(rnum)),
            VdBsqTerm::Inum(inum) => Some(VdBsqNumTerm::Inum(inum)),
            VdBsqTerm::Prop(_) => None,
        }
    }
}

impl<'sess> std::fmt::Debug for VdBsqTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.show_fmt(f)
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn show_fmt(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdBsqTerm::Rnum(rnum) => rnum.show_fmt(f),
            VdBsqTerm::Inum(inum) => inum.show_fmt(f),
            VdBsqTerm::Prop(prop) => prop.show_fmt(f),
        }
    }
}

impl<'sess> std::fmt::Debug for VdBsqNumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'db, 'sess> VdBsqElaboratorInner<'db, 'sess> {
    pub fn calc_expr_term(
        &self,
        expr_entry: &VdMirExprEntry,
        symbol_local_defn_storage: &VdMirSymbolLocalDefnStorage,
    ) -> VdBsqTerm<'sess> {
        match *expr_entry.data() {
            VdMirExprData::Literal(vd_literal) => match *vd_literal.data() {
                VdLiteralData::Int128(i) => VdBsqTerm::Rnum(VdBsqRnumTerm::Int128(i)),
                VdLiteralData::BigInt(n) => todo!(),
                VdLiteralData::Float(_) => todo!(),
                VdLiteralData::SpecialConstant(vd_special_constant) => todo!(),
            },
            VdMirExprData::Variable(local_defn_idx) => {
                let lx_math_letter =
                    match *symbol_local_defn_storage.defn_arena()[local_defn_idx].head() {
                        VdMirSymbolLocalDefnHead::Letter(lx_math_letter) => lx_math_letter,
                    };
                if expr_entry.ty().is_numeric(self.eterner_db()) {
                    if let Some(_) = self.eval_variable() {
                        todo!()
                    } else {
                        VdBsqTerm::new_numeric_variable(
                            lx_math_letter,
                            local_defn_idx,
                            self.floater_db(),
                        )
                    }
                } else {
                    todo!()
                }
            }
            VdMirExprData::Application {
                function,
                arguments,
            } => match function {
                VdMirFunc::NormalBasePrefixOpr(signature) => todo!(),
                VdMirFunc::NormalBaseSeparator(signature) => todo!(),
                VdMirFunc::NormalBaseBinaryOpr(signature) => match signature.opr {
                    VdMirBaseBinaryOpr::CommRingSub => {
                        let lopd = self
                            .expr_fld(arguments.first().unwrap())
                            .term()
                            .num()
                            .unwrap();
                        let ropd = self
                            .expr_fld(arguments.last().unwrap())
                            .term()
                            .num()
                            .unwrap();
                        lopd.sub(ropd, self.floater_db()).into()
                    }
                    VdMirBaseBinaryOpr::CommFieldDiv => todo!(),
                },
                VdMirFunc::Power(signature) => {
                    assert_eq!(arguments.len(), 2);
                    let Some(base) = self.expr_fld(arguments.first().unwrap()).term().num() else {
                        todo!()
                    };
                    let Some(exponent) = self.expr_fld(arguments.last().unwrap()).term().num()
                    else {
                        todo!()
                    };
                    match base.product_or_non_product() {
                        Either::Left(base) => todo!(),
                        Either::Right(base) => {
                            VdBsqTerm::new_power(base, exponent, self.floater_db())
                        }
                    }
                }
                VdMirFunc::InSet => todo!(),
                VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
                VdMirFunc::NormalBaseFrac(vd_base_binary_opr_signature) => todo!(),
            },
            VdMirExprData::FoldingSeparatedList {
                leader,
                ref followers,
            } => {
                let (func, follower) = *followers.first().unwrap();
                let num_relationship = |slf: &Self, kind| {
                    VdBsqTerm::new_num_relationship(
                        slf.expr_fld(leader).term().num().unwrap(),
                        kind,
                        slf.expr_fld(follower).term().num().unwrap(),
                        slf.floater_db(),
                    )
                };
                match func {
                    VdMirFunc::NormalBasePrefixOpr(signature) => todo!(),
                    VdMirFunc::NormalBaseSeparator(signature) => match signature.opr() {
                        VdMirBaseSeparator::CommRingAdd => {
                            let mut builder = VdBsqSumBuilder::new(self.floater_db());
                            builder.add_num(self.expr_fld(leader).term().num().unwrap());
                            for &(_, follower) in followers.iter() {
                                builder.add_num(self.expr_fld(follower).term().num().unwrap());
                            }
                            builder.finish().into()
                        }
                        VdMirBaseSeparator::CommRingMul => {
                            let mut builder = VdBsqProductBuilder::new(self.floater_db());
                            builder.mul_num(self.expr_fld(leader).term().num().unwrap());
                            for &(_, follower) in followers.iter() {
                                builder.mul_num(self.expr_fld(follower).term().num().unwrap());
                            }
                            builder.finish().into()
                        }
                        VdMirBaseSeparator::Eq => todo!(),
                        VdMirBaseSeparator::Ne => todo!(),
                        VdMirBaseSeparator::Lt => todo!(),
                        VdMirBaseSeparator::Gt => todo!(),
                        VdMirBaseSeparator::Le => todo!(),
                        VdMirBaseSeparator::Ge => todo!(),
                        VdMirBaseSeparator::Subset => todo!(),
                        VdMirBaseSeparator::Supset => todo!(),
                        VdMirBaseSeparator::Subseteq => todo!(),
                        VdMirBaseSeparator::Supseteq => todo!(),
                        VdMirBaseSeparator::Subseteqq => todo!(),
                        VdMirBaseSeparator::Supseteqq => todo!(),
                        VdMirBaseSeparator::Subsetneq => todo!(),
                        VdMirBaseSeparator::Supsetneq => todo!(),
                        VdMirBaseSeparator::In => todo!(),
                        VdMirBaseSeparator::Notin => todo!(),
                        VdMirBaseSeparator::SetTimes => todo!(),
                        VdMirBaseSeparator::TensorOtimes => todo!(),
                    },
                    VdMirFunc::NormalBaseBinaryOpr(signature) => todo!(),
                    VdMirFunc::Power(signature) => todo!(),
                    VdMirFunc::InSet => todo!(),
                    VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
                    VdMirFunc::NormalBaseFrac(vd_base_binary_opr_signature) => todo!(),
                }
            }
            VdMirExprData::ChainingSeparatedList {
                leader,
                ref followers,
                joined_signature: joined_separator_and_signature,
            } => match joined_separator_and_signature {
                Some(joined_separator_and_signature) => todo!(),
                None => {
                    use VdBsqNumRelationshipPropTermKind::*;

                    let (func, follower) = *followers.first().unwrap();
                    let num_relationship = |slf: &Self, kind| {
                        VdBsqTerm::new_num_relationship(
                            slf.expr_fld(leader).term().num().unwrap(),
                            kind,
                            slf.expr_fld(follower).term().num().unwrap(),
                            slf.floater_db(),
                        )
                    };
                    match func {
                        VdMirFunc::NormalBasePrefixOpr(signature) => todo!(),
                        VdMirFunc::NormalBaseSeparator(signature) => match signature.opr() {
                            VdMirBaseSeparator::CommRingAdd => todo!(),
                            VdMirBaseSeparator::CommRingMul => todo!(),
                            VdMirBaseSeparator::Eq => num_relationship(self, Eq),
                            VdMirBaseSeparator::Ne => num_relationship(self, Ne),
                            VdMirBaseSeparator::Lt => num_relationship(self, Lt),
                            VdMirBaseSeparator::Gt => num_relationship(self, Gt),
                            VdMirBaseSeparator::Le => num_relationship(self, Le),
                            VdMirBaseSeparator::Ge => num_relationship(self, Ge),
                            VdMirBaseSeparator::Subset => todo!(),
                            VdMirBaseSeparator::Supset => todo!(),
                            VdMirBaseSeparator::Subseteq => todo!(),
                            VdMirBaseSeparator::Supseteq => todo!(),
                            VdMirBaseSeparator::Subseteqq => todo!(),
                            VdMirBaseSeparator::Supseteqq => todo!(),
                            VdMirBaseSeparator::Subsetneq => todo!(),
                            VdMirBaseSeparator::Supsetneq => todo!(),
                            VdMirBaseSeparator::In => todo!(),
                            VdMirBaseSeparator::Notin => todo!(),
                            VdMirBaseSeparator::SetTimes => todo!(),
                            VdMirBaseSeparator::TensorOtimes => todo!(),
                        },
                        VdMirFunc::NormalBaseBinaryOpr(signature) => todo!(),
                        VdMirFunc::Power(signature) => todo!(),
                        VdMirFunc::InSet => todo!(),
                        VdMirFunc::NormalBaseSqrt(vd_base_sqrt_signature) => todo!(),
                        VdMirFunc::NormalBaseFrac(vd_base_binary_opr_signature) => todo!(),
                    }
                }
            },
            VdMirExprData::ItemPath(vd_item_path) => todo!(),
        }
    }
}
