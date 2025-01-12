use super::*;
use crate::term::{
    atom::VdBsqAtomComnumTerm,
    product::{VdBsqProductBase, VdBsqProductComnumTermBaseData},
    sum::VdBsqSumComnumTerm,
    VdBsqComnumTerm, VdBsqLitnumTerm, VdBsqMonomialCoefficients, VdBsqNonSumComnumTerm,
    VdBsqNumTerm,
};
use floated_sequential::db::FloaterDb;

#[derive(Clone)]
pub struct VdBsqSumBuilder<'sess> {
    db: &'sess FloaterDb,
    /// Only for numbers representable efficiently by computers.
    /// For huge numbers like `2^100000`, we don't want to put it here.
    constant_litnum: VdBsqLitnumTerm<'sess>,
    unpruned_monomials: VdBsqMonomialCoefficients<'sess>,
}

impl<'sess> std::fmt::Debug for VdBsqSumBuilder<'sess> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VdBsqSumBuilder")
            .field("constant_litnum", &self.constant_litnum)
            .field("unpruned_monomials", &self.unpruned_monomials)
            .finish()
    }
}

impl<'sess> VdBsqSumBuilder<'sess> {
    pub fn new(db: &'sess FloaterDb) -> Self {
        Self {
            db,
            constant_litnum: VdBsqLitnumTerm::ZERO,
            unpruned_monomials: VdBsqMonomialCoefficients::default(),
        }
    }
}

impl<'sess> VdBsqSumBuilder<'sess> {
    pub fn constant_litnum(&self) -> VdBsqLitnumTerm {
        self.constant_litnum
    }

    pub fn unpruned_monomials(&self) -> &VdBsqMonomialCoefficients<'sess> {
        &self.unpruned_monomials
    }
}

impl<'sess> VdBsqSumBuilder<'sess> {
    pub fn add_num(&mut self, term: VdBsqNumTerm<'sess>) {
        match term {
            VdBsqNumTerm::Litnum(term) => self.add_litnum(term),
            VdBsqNumTerm::Comnum(term) => match term {
                VdBsqComnumTerm::Atom(term) => self.add_atom(term),
                VdBsqComnumTerm::Sum(term) => self.add_sum(term),
                VdBsqComnumTerm::Product(litnum, term) => self.add_product(litnum, term),
            },
        }
    }

    pub fn add_litnum_times_atom(
        &mut self,
        litnum: VdBsqLitnumTerm<'sess>,
        atom: VdBsqAtomComnumTerm<'sess>,
    ) {
        self.add_monomial(VdBsqNonSumComnumTerm::Atom(atom), litnum);
    }

    pub fn add_litnum(&mut self, term: VdBsqLitnumTerm<'sess>) {
        self.constant_litnum.add_assign(term, self.db);
    }

    pub fn sub_litnum(&mut self, term: VdBsqLitnumTerm<'sess>) {
        self.constant_litnum.sub_assign(term, self.db);
    }

    pub fn sub_num(&mut self, term: VdBsqNumTerm<'sess>) {
        match term {
            VdBsqNumTerm::Litnum(term) => self.constant_litnum.sub_assign(term, self.db),
            VdBsqNumTerm::Comnum(term) => self.sub_comnum(term),
        }
    }

    pub fn sub_comnum(&mut self, term: VdBsqComnumTerm<'sess>) {
        match term {
            VdBsqComnumTerm::Atom(term) => self.sub_atom(term),
            VdBsqComnumTerm::Sum(term) => self.sub_sum(term),
            VdBsqComnumTerm::Product(litnum, term) => self.sub_product(litnum, term),
        }
    }

    pub fn add_atom(&mut self, term: VdBsqAtomComnumTerm<'sess>) {
        self.add_monomial(VdBsqNonSumComnumTerm::Atom(term), VdBsqLitnumTerm::ONE);
    }

    pub fn sub_atom(&mut self, term: VdBsqAtomComnumTerm<'sess>) {
        self.add_monomial(VdBsqNonSumComnumTerm::Atom(term), VdBsqLitnumTerm::NEG_ONE);
    }

    pub fn add_sum(&mut self, term: VdBsqSumComnumTerm<'sess>) {
        self.add_litnum(term.constant_term());
        for &(monomial, coeff) in term.monomials() {
            self.add_monomial(monomial, coeff);
        }
    }

    pub fn sub_sum(&mut self, term: VdBsqSumComnumTerm<'sess>) {
        self.sub_litnum(term.constant_term());
        for &(monomial, coeff) in term.monomials() {
            self.add_monomial(monomial, coeff.neg(self.db));
        }
    }

    pub fn add_product(&mut self, litnum: VdBsqLitnumTerm<'sess>, term: VdBsqProductBase<'sess>) {
        self.add_monomial(VdBsqNonSumComnumTerm::Product(term), litnum);
    }

    pub fn add_general_product(
        &mut self,
        litnum: VdBsqLitnumTerm<'sess>,
        term: VdBsqNumTerm<'sess>,
    ) {
        match term {
            VdBsqNumTerm::Litnum(term) => self.add_litnum(litnum.mul(term, self.db)),
            VdBsqNumTerm::Comnum(term) => match term {
                VdBsqComnumTerm::Atom(term) => {
                    self.add_monomial(VdBsqNonSumComnumTerm::Atom(term), litnum);
                }
                VdBsqComnumTerm::Sum(term) => {
                    self.add_monomial(
                        VdBsqNonSumComnumTerm::Product(VdBsqProductBase::new(
                            [(term.into(), VdBsqNumTerm::ONE)].into_iter().collect(),
                            self.db,
                        )),
                        litnum,
                    );
                }
                VdBsqComnumTerm::Product(litnum1, base) => {
                    self.add_product(litnum.mul(litnum1, self.db), base);
                }
            },
        }
    }

    pub fn sub_product(&mut self, litnum: VdBsqLitnumTerm<'sess>, term: VdBsqProductBase<'sess>) {
        self.add_monomial(VdBsqNonSumComnumTerm::Product(term), litnum.neg(self.db));
    }

    pub fn add_monomial(
        &mut self,
        term: VdBsqNonSumComnumTerm<'sess>,
        coeff: VdBsqLitnumTerm<'sess>,
    ) {
        self.unpruned_monomials
            .insert_or_update((term, coeff), |(_, old_coeff)| {
                old_coeff.add_assign(coeff, self.db);
            });
    }

    pub fn finish(self) -> VdBsqNumTerm<'sess> {
        let monomials: VdBsqMonomialCoefficients = self
            .unpruned_monomials
            .into_iter()
            .filter(|(_, coeff)| !coeff.is_zero())
            .collect();
        match (monomials.len(), self.constant_litnum) {
            (0, _) => self.constant_litnum.into(),
            (1, VdBsqLitnumTerm::ZERO) => {
                let (non_sum, coeff) = monomials.into_iter().next().unwrap();
                assert!(!coeff.is_zero());
                if coeff.is_one() {
                    todo!()
                } else {
                    match non_sum {
                        VdBsqNonSumComnumTerm::Atom(term) => VdBsqComnumTerm::Product(
                            coeff,
                            VdBsqProductBase::new(
                                [(term.into(), VdBsqNumTerm::ONE)].into_iter().collect(),
                                self.db,
                            ),
                        )
                        .into(),
                        VdBsqNonSumComnumTerm::Product(base) => {
                            VdBsqComnumTerm::Product(coeff, base).into()
                        }
                    }
                }
            }
            _ => VdBsqSumComnumTerm::new(self.constant_litnum, monomials, self.db).into(),
        }
    }
}
