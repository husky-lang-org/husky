use super::*;
use crate::term::{
    atom::VdBsqAtomInumTerm,
    product::{VdBsqProductInumTermBase, VdBsqProductInumTermBaseData},
    sum::VdBsqSumInumTerm,
    VdBsqInumMonomialCoefficients, VdBsqInumTerm, VdBsqLitNumTerm, VdBsqNonSumInumTerm,
    VdBsqNumTerm,
};
use floated_sequential::db::FloaterDb;

#[derive(Clone)]
pub struct VdBsqSumBuilder<'sess> {
    db: &'sess FloaterDb,
    /// Only for numbers representable efficiently by computers.
    /// For huge numbers like `2^100000`, we don't want to put it here.
    constant_rnum: VdBsqLitNumTerm<'sess>,
    unpruned_monomials: VdBsqInumMonomialCoefficients<'sess>,
}

impl<'sess> VdBsqSumBuilder<'sess> {
    pub fn new(db: &'sess FloaterDb) -> Self {
        Self {
            db,
            constant_rnum: VdBsqLitNumTerm::ZERO,
            unpruned_monomials: VdBsqInumMonomialCoefficients::default(),
        }
    }
}

impl<'sess> VdBsqSumBuilder<'sess> {
    pub fn constant_rnum(&self) -> VdBsqLitNumTerm {
        self.constant_rnum
    }

    pub fn unpruned_monomials(&self) -> &VdBsqInumMonomialCoefficients<'sess> {
        &self.unpruned_monomials
    }
}

impl<'sess> VdBsqSumBuilder<'sess> {
    pub fn add_num(&mut self, term: VdBsqNumTerm<'sess>) {
        match term {
            VdBsqNumTerm::Rnum(term) => self.add_rnum(term),
            VdBsqNumTerm::Inum(term) => match term {
                VdBsqInumTerm::Atom(term) => self.add_atom_inum(term),
                VdBsqInumTerm::Sum(term) => self.add_sum(term),
                VdBsqInumTerm::Product(rnum, term) => self.add_product(rnum, term),
            },
        }
    }

    pub fn add_rnum_times_atom(
        &mut self,
        rnum: VdBsqLitNumTerm<'sess>,
        atom: VdBsqAtomInumTerm<'sess>,
    ) {
        self.add_monomial(VdBsqNonSumInumTerm::Atom(atom), rnum);
    }

    pub fn add_rnum(&mut self, term: VdBsqLitNumTerm<'sess>) {
        self.constant_rnum.add_assign(term, self.db);
    }

    pub fn sub_rnum(&mut self, term: VdBsqLitNumTerm<'sess>) {
        self.constant_rnum.sub_assign(term, self.db);
    }

    pub fn sub_num(&mut self, term: VdBsqNumTerm<'sess>) {
        match term {
            VdBsqNumTerm::Rnum(term) => self.constant_rnum.sub_assign(term, self.db),
            VdBsqNumTerm::Inum(term) => self.sub_inum(term),
        }
    }

    pub fn sub_inum(&mut self, term: VdBsqInumTerm<'sess>) {
        match term {
            VdBsqInumTerm::Atom(term) => self.sub_atom(term),
            VdBsqInumTerm::Sum(term) => self.sub_sum(term),
            VdBsqInumTerm::Product(rnum, term) => self.sub_product(rnum, term),
        }
    }

    pub fn add_atom_inum(&mut self, term: VdBsqAtomInumTerm<'sess>) {
        self.add_monomial(VdBsqNonSumInumTerm::Atom(term), VdBsqLitNumTerm::ONE);
    }

    pub fn sub_atom(&mut self, term: VdBsqAtomInumTerm<'sess>) {
        self.add_monomial(VdBsqNonSumInumTerm::Atom(term), VdBsqLitNumTerm::NEG_ONE);
    }

    pub fn add_sum(&mut self, term: VdBsqSumInumTerm<'sess>) {
        todo!()
    }

    pub fn sub_sum(&mut self, term: VdBsqSumInumTerm<'sess>) {
        self.sub_rnum(term.constant_term());
        for &(monomial, coeff) in term.monomials() {
            self.add_monomial(monomial, coeff.neg(self.db));
        }
    }

    pub fn add_product(
        &mut self,
        rnum: VdBsqLitNumTerm<'sess>,
        term: VdBsqProductInumTermBase<'sess>,
    ) {
        self.add_monomial(VdBsqNonSumInumTerm::Product(term), rnum);
    }

    pub fn add_general_product(&mut self, rnum: VdBsqLitNumTerm<'sess>, term: VdBsqNumTerm<'sess>) {
        match term {
            VdBsqNumTerm::Rnum(term) => self.add_rnum(rnum.mul(term, self.db)),
            VdBsqNumTerm::Inum(term) => match term {
                VdBsqInumTerm::Atom(term) => {
                    self.add_monomial(VdBsqNonSumInumTerm::Atom(term), rnum);
                }
                VdBsqInumTerm::Sum(term) => {
                    self.add_monomial(
                        VdBsqNonSumInumTerm::Product(VdBsqProductInumTermBase::new(
                            [(term.into(), VdBsqNumTerm::ONE)].into_iter().collect(),
                            self.db,
                        )),
                        rnum,
                    );
                }
                VdBsqInumTerm::Product(rnum1, base) => {
                    self.add_product(rnum.mul(rnum1, self.db), base);
                }
            },
        }
    }

    pub fn sub_product(
        &mut self,
        rnum: VdBsqLitNumTerm<'sess>,
        term: VdBsqProductInumTermBase<'sess>,
    ) {
        self.add_monomial(VdBsqNonSumInumTerm::Product(term), rnum.neg(self.db));
    }

    pub fn add_monomial(
        &mut self,
        term: VdBsqNonSumInumTerm<'sess>,
        coeff: VdBsqLitNumTerm<'sess>,
    ) {
        self.unpruned_monomials
            .insert_or_update((term, coeff), |(_, old_coeff)| {
                old_coeff.add_assign(coeff, self.db);
            });
    }

    pub fn finish(self) -> VdBsqNumTerm<'sess> {
        let monomials: VdBsqInumMonomialCoefficients = self
            .unpruned_monomials
            .into_iter()
            .filter(|(_, coeff)| !coeff.is_zero())
            .collect();
        match (monomials.len(), self.constant_rnum) {
            (0, _) => self.constant_rnum.into(),
            (1, VdBsqLitNumTerm::ZERO) => {
                let (non_sum, coeff) = monomials.into_iter().next().unwrap();
                assert!(!coeff.is_zero());
                if coeff.is_one() {
                    todo!()
                } else {
                    match non_sum {
                        VdBsqNonSumInumTerm::Atom(term) => VdBsqInumTerm::Product(
                            coeff,
                            VdBsqProductInumTermBase::new(
                                [(term.into(), VdBsqNumTerm::ONE)].into_iter().collect(),
                                self.db,
                            ),
                        )
                        .into(),
                        VdBsqNonSumInumTerm::Product(base) => {
                            VdBsqInumTerm::Product(coeff, base).into()
                        }
                    }
                }
            }
            _ => VdBsqSumInumTerm::new(self.constant_rnum, monomials, self.db).into(),
        }
    }
}
