pub mod bigint;

use self::bigint::VdBsqRnumTermBigInt;
use super::*;
use std::num::NonZeroU128;
use visored_opr::precedence::{VdPrecedence, VdPrecedenceRange};

#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqRnumTerm<'sess> {
    Int128(i128),
    BigInt(VdBsqRnumTermBigInt<'sess>),
    Rat128(i128, u128),
}

impl<'sess> From<i128> for VdBsqRnumTerm<'sess> {
    fn from(i: i128) -> Self {
        VdBsqRnumTerm::Int128(i)
    }
}

impl<'sess> From<VdBsqRnumTermBigInt<'sess>> for VdBsqRnumTerm<'sess> {
    fn from(i: VdBsqRnumTermBigInt<'sess>) -> Self {
        VdBsqRnumTerm::BigInt(i)
    }
}

impl<'sess> std::fmt::Debug for VdBsqRnumTerm<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("VdBsqRnumTerm(`")?;
        self.show_fmt(VdPrecedenceRange::Any, f)?;
        f.write_str("`)")
    }
}

impl<'sess> VdBsqRnumTerm<'sess> {
    pub fn neg(self, db: &FloaterDb) -> Self {
        match self {
            VdBsqRnumTerm::Int128(i) => VdBsqRnumTerm::Int128(-i),
            VdBsqRnumTerm::BigInt(i) => todo!(),
            VdBsqRnumTerm::Rat128(a, b) => VdBsqRnumTerm::Rat128(-a, b),
        }
    }

    pub fn add_assign(&mut self, rhs: Self, db: &FloaterDb) {
        if self.is_zero() {
            *self = rhs;
            return;
        }
        match self {
            VdBsqRnumTerm::Int128(slf) => match rhs {
                VdBsqRnumTerm::Int128(rhs) => match slf.checked_add(rhs) {
                    Some(sum) => *self = VdBsqRnumTerm::Int128(sum),
                    None => todo!(),
                },
                VdBsqRnumTerm::BigInt(i) => {
                    use husky_print_utils::p;
                    p!(self, rhs);
                    todo!()
                }
                VdBsqRnumTerm::Rat128(_, _) => todo!(),
            },
            VdBsqRnumTerm::BigInt(i) => todo!(),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }

    pub fn sub_assign(&mut self, rhs: Self, db: &'sess FloaterDb) {
        match self {
            VdBsqRnumTerm::Int128(slf) => match rhs {
                VdBsqRnumTerm::Int128(rhs) => match slf.checked_sub(rhs) {
                    Some(sum) => *self = VdBsqRnumTerm::Int128(sum),
                    None => todo!(),
                },
                VdBsqRnumTerm::BigInt(i) => todo!(),
                VdBsqRnumTerm::Rat128(_, _) => todo!(),
            },
            VdBsqRnumTerm::BigInt(i) => match rhs {
                VdBsqRnumTerm::Int128(_) => todo!(),
                VdBsqRnumTerm::BigInt(i1) => *self = i.sub(i1, db),
                VdBsqRnumTerm::Rat128(_, _) => todo!(),
            },
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }

    pub fn mul(self, rhs: Self, db: &FloaterDb) -> Self {
        match rhs {
            VdBsqRnumTerm::Int128(rhs) => self.mul128(rhs, db),
            VdBsqRnumTerm::BigInt(i) => todo!(),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }

    pub fn mul128(self, rhs: i128, db: &FloaterDb) -> Self {
        match self {
            VdBsqRnumTerm::Int128(i) => match i.checked_mul(rhs) {
                Some(product) => VdBsqRnumTerm::Int128(product),
                None => todo!(),
            },
            VdBsqRnumTerm::BigInt(i) => todo!(),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }

    pub fn mul_assign(&mut self, rhs: Self, db: &FloaterDb) {
        match self {
            VdBsqRnumTerm::Int128(slf) => match rhs {
                VdBsqRnumTerm::Int128(rhs) => match slf.checked_mul(rhs) {
                    Some(product) => *self = VdBsqRnumTerm::Int128(product),
                    None => todo!(),
                },
                VdBsqRnumTerm::BigInt(i) => todo!(),
                VdBsqRnumTerm::Rat128(_, _) => todo!(),
            },
            VdBsqRnumTerm::BigInt(i) => todo!(),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }

    pub fn pow128(self, exponent: i128, db: &FloaterDb) -> Self {
        match self {
            VdBsqRnumTerm::Int128(i) => {
                if exponent > 0 {
                    let exponent: u32 = match exponent.try_into() {
                        Ok(exponent) => exponent,
                        Err(_) => todo!(),
                    };
                    match i.checked_pow(exponent) {
                        Some(pow) => VdBsqRnumTerm::Int128(pow),
                        None => todo!(),
                    }
                } else {
                    use husky_print_utils::p;
                    p!(exponent);
                    todo!()
                }
            }
            VdBsqRnumTerm::BigInt(i) => todo!(),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }
}

impl<'sess> VdBsqRnumTerm<'sess> {
    pub const ZERO: Self = Self::Int128(0);
    pub const ONE: Self = Self::Int128(1);
    pub const NEG_ONE: Self = Self::Int128(-1);
}

impl<'sess> VdBsqRnumTerm<'sess> {
    pub fn is_zero(self) -> bool {
        self.eqs_i128(0)
    }

    pub fn is_one(self) -> bool {
        self.eqs_i128(1)
    }

    pub fn eqs_i128(self, i0: i128) -> bool {
        match self {
            VdBsqRnumTerm::Int128(i) => i == i0,
            _ => false,
        }
    }
}

impl<'sess> VdBsqRnumTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let outer_precedence = self.outer_precedence();
        if precedence_range.contains(outer_precedence) {
            self.show_fmt_inner(f)
        } else {
            f.write_str("(")?;
            self.show_fmt_inner(f)?;
            f.write_str(")")
        }
    }

    pub fn outer_precedence(self) -> VdPrecedence {
        match self {
            VdBsqRnumTerm::Int128(i) => {
                if i >= 0 {
                    VdPrecedence::ATOM
                } else {
                    VdPrecedence::ADD_SUB
                }
            }
            VdBsqRnumTerm::BigInt(i) => {
                if i.is_nonnegative() {
                    VdPrecedence::ATOM
                } else {
                    VdPrecedence::ADD_SUB
                }
            }
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }

    fn show_fmt_inner(self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdBsqRnumTerm::Int128(i) => write!(f, "{}", i),
            VdBsqRnumTerm::BigInt(i) => i.show_fmt(f),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }
}

impl<'sess> VdBsqRnumTerm<'sess> {
    pub fn compare_with_zero(self, kind: VdBsqNumRelationshipPropTermKind) -> bool {
        match self {
            VdBsqRnumTerm::Int128(i) => match kind {
                VdBsqNumRelationshipPropTermKind::Eq => i == 0,
                VdBsqNumRelationshipPropTermKind::Ne => i != 0,
                VdBsqNumRelationshipPropTermKind::Lt => i < 0,
                VdBsqNumRelationshipPropTermKind::Gt => i > 0,
                VdBsqNumRelationshipPropTermKind::Le => i <= 0,
                VdBsqNumRelationshipPropTermKind::Ge => i >= 0,
            },
            VdBsqRnumTerm::BigInt(i) => todo!(),
            VdBsqRnumTerm::Rat128(_, _) => todo!(),
        }
    }
}
