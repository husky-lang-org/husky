use super::*;
use latex_math_letter::letter::LxMathLetter;
use visored_mir_expr::symbol::local_defn::VdMirSymbolLocalDefnIdx;
use visored_opr::precedence::VdPrecedenceRange;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct VdBsqAtomTerm<'sess>(VdBsqComnumTermFld<'sess>);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum VdBsqComnumAtomTermData {
    Variable(LxMathLetter, VdMirSymbolLocalDefnIdx),
}

impl<'sess> From<VdBsqAtomTerm<'sess>> for VdBsqNumTerm<'sess> {
    fn from(value: VdBsqAtomTerm<'sess>) -> Self {
        VdBsqNumTerm::Comnum(VdBsqComnumTerm::Atom(value))
    }
}

impl<'sess> VdBsqAtomTerm<'sess> {
    pub fn new(data: VdBsqComnumAtomTermData, db: &'sess FloaterDb) -> Self {
        VdBsqAtomTerm(VdBsqComnumTermFld::new(VdBsqComnumTermData::Atom(data), db))
    }
}

impl<'sess> VdBsqAtomTerm<'sess> {
    pub fn data(self) -> &'sess VdBsqComnumAtomTermData {
        match self.0.data() {
            VdBsqComnumTermData::Atom(data) => data,
            _ => unreachable!(),
        }
    }
}

impl<'sess> VdBsqComnumTerm<'sess> {
    pub fn new_atom(data: VdBsqComnumAtomTermData, db: &'sess FloaterDb) -> Self {
        VdBsqComnumTerm::Atom(VdBsqAtomTerm::new(data, db))
    }
}

impl<'sess> VdBsqTerm<'sess> {
    pub fn new_numeric_variable(
        lx_math_letter: LxMathLetter,
        local_defn_idx: VdMirSymbolLocalDefnIdx,
        db: &'sess FloaterDb,
    ) -> Self {
        VdBsqTerm::Comnum(VdBsqComnumTerm::Atom(VdBsqAtomTerm(
            VdBsqComnumTermFld::new(
                VdBsqComnumTermData::Atom(VdBsqComnumAtomTermData::Variable(
                    lx_math_letter,
                    local_defn_idx,
                )),
                db,
            ),
        )))
    }
}

impl<'sess> VdBsqAtomTerm<'sess> {
    pub fn show_fmt(
        self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        self.data().show_fmt(precedence_range, f)
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        self.data().outer_precedence()
    }
}

impl<'sess> VdBsqComnumAtomTermData {
    pub fn show_fmt(
        &self,
        precedence_range: VdPrecedenceRange,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            VdBsqComnumAtomTermData::Variable(lx_math_letter, _) => {
                write!(f, "{}", lx_math_letter.unicode())
            }
        }
    }

    pub fn outer_precedence(&self) -> VdPrecedence {
        match self {
            VdBsqComnumAtomTermData::Variable(_, _) => VdPrecedence::ATOM,
        }
    }
}

impl<'sess> VdBsqAtomTerm<'sess> {
    pub fn neg(self, db: &'sess FloaterDb) -> VdBsqProductTerm<'sess> {
        VdBsqProductTerm::new(-1, self)
    }

    pub fn mul_litnum(
        self,
        rhs: impl Into<VdBsqLitnumTerm<'sess>>,
        db: &'sess FloaterDb,
    ) -> VdBsqNumTerm<'sess> {
        let rhs = rhs.into();
        if rhs == 0.into() {
            return VdBsqNumTerm::ZERO;
        }
        if rhs == 1.into() {
            return self.into();
        }
        VdBsqProductTerm::new(rhs, self).into()
    }

    pub fn div_litnum(
        self,
        rhs: VdBsqLitnumTerm<'sess>,
        db: &'sess FloaterDb,
    ) -> Option<VdBsqComnumTerm<'sess>> {
        if rhs == 0.into() {
            return None;
        }
        if rhs == 1.into() {
            return Some(self.into());
        }
        Some(VdBsqProductTerm::new(rhs.inverse().unwrap(), self).into())
    }
}
