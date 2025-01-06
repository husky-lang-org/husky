use super::*;
use crate::term::sum::VdBsqSumInumTerm;
use builder::sum::VdBsqSumBuilder;
use smallvec::*;

#[enum_class::from_variants]
#[derive(Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqNumTerm<'sess> {
    Rnum(VdBsqRnumTerm),
    Inum(VdBsqInumTerm<'sess>),
}

impl<'sess> From<VdBsqNumTerm<'sess>> for VdBsqTerm<'sess> {
    fn from(term: VdBsqNumTerm<'sess>) -> Self {
        match term {
            VdBsqNumTerm::Rnum(term) => VdBsqTerm::Rnum(term),
            VdBsqNumTerm::Inum(term) => VdBsqTerm::Inum(term),
        }
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub const ZERO: Self = VdBsqNumTerm::Rnum(VdBsqRnumTerm::ZERO);
    pub const ONE: Self = VdBsqNumTerm::Rnum(VdBsqRnumTerm::ONE);
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn is_zero_trivially(&self) -> bool {
        match self {
            VdBsqNumTerm::Rnum(term) => term.is_zero(),
            VdBsqNumTerm::Inum(term) => false,
        }
    }

    pub fn eqs_i128_trivially(&self, rhs: i128) -> bool {
        match self {
            VdBsqNumTerm::Rnum(term) => term.eqs_i128(rhs),
            VdBsqNumTerm::Inum(term) => false,
        }
    }

    pub fn add(self, rhs: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) -> VdBsqNumTerm<'sess> {
        if rhs.is_zero_trivially() {
            return self;
        }
        let mut builder = VdBsqSumBuilder::new(db);
        builder.add_num(self);
        builder.add_num(rhs);
        builder.finish()
    }

    pub fn sub(self, rhs: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) -> VdBsqNumTerm<'sess> {
        if rhs.is_zero_trivially() {
            return self;
        }
        let mut builder = VdBsqSumBuilder::new(db);
        builder.add_num(self);
        builder.sub_num(rhs);
        builder.finish()
    }

    pub fn add_assign(&mut self, rhs: VdBsqNumTerm<'sess>, db: &'sess FloaterDb) {
        *self = self.add(rhs, db);
    }
}

impl<'sess> VdBsqNumTerm<'sess> {
    pub fn show_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VdBsqNumTerm::Rnum(term) => term.show_fmt(f),
            VdBsqNumTerm::Inum(term) => term.show_fmt(f),
        }
    }
}
