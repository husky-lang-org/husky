use super::*;

/// expect a type that is explicitly convertible to dst
#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db]
pub struct ExpectCasting {
    pub(crate) destination: FlyTerm,
}

impl ExpectCasting {
    pub fn new(destination: FlyTerm) -> Self {
        Self { destination }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db]
pub struct ExpectExplicitlyConvertibleOutcome {
    destination: FlyTerm,
}

impl ExpectFlyTerm for ExpectCasting {
    type Outcome = ExpectExplicitlyConvertibleOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::ExplicitlyConvertible(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FlyTerms) -> FinalDestination {
        todo!()
    }

    #[inline(always)]
    fn destination(&self) -> FlyTermDestination {
        FlyTermDestination::Specific(self.destination)
    }

    fn resolve(
        &self,
        _db: &::salsa::Db,
        _terms: &mut FlyTerms,
        _state: &mut ExpectationState,
    ) -> AltOption<FlyTermEffect> {
        // todo
        AltOption::AltNone
    }
}
