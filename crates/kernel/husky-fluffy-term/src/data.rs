mod hollow;
mod solid;

pub(crate) use self::hollow::*;
pub(crate) use self::solid::*;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub enum FluffyTermData<'a> {
    Literal(TermLiteral),
    TypeOntology {
        path: TypePath,
        refined_path: Either<CustomTypePath, PreludeTypePath>,
        argument_tys: &'a [FluffyTerm],
    },
    PlaceTypeOntology {
        place: Place,
        path: TypePath,
        refined_path: Either<CustomTypePath, PreludeTypePath>,
        argument_tys: &'a [FluffyTerm],
    },
    Curry {
        curry_kind: CurryKind,
        variance: Variance,
        parameter_variable: Option<FluffyTerm>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    },
    Hole(HoleKind, HollowTerm),
    Category(TermCategory),
    Ritchie {
        ritchie_kind: TermRitchieKind,
        parameter_contracted_tys: &'a [FluffyTermRitchieParameterContractedType],
        return_ty: FluffyTerm,
    },
}

impl FluffyTerm {
    pub fn data<'a, 'b>(self, engine: &'a impl FluffyTermEngine<'b>) -> FluffyTermData<'a>
    where
        'b: 'a,
    {
        self.data_inner(engine.db(), engine.fluffy_terms())
    }

    pub(crate) fn data_inner<'a>(
        self,
        db: &'a dyn FluffyTermDb,
        fluffy_terms: &'a FluffyTerms,
    ) -> FluffyTermData<'a> {
        match self {
            FluffyTerm::Literal(_) => todo!(),
            FluffyTerm::Symbol(_) => todo!(),
            FluffyTerm::Hole(_) => todo!(),
            FluffyTerm::EntityPath(path) => match path {
                TermEntityPath::Form(_) => todo!(),
                TermEntityPath::Trait(_) => todo!(),
                TermEntityPath::TypeOntology(path) => FluffyTermData::TypeOntology {
                    path,
                    refined_path: path.refine(db),
                    argument_tys: &[],
                },
                TermEntityPath::TypeConstructor(_) => todo!(),
            },
            FluffyTerm::Category(term) => FluffyTermData::Category(term),
            FluffyTerm::Universe(_) => todo!(),
            FluffyTerm::Curry(term) => FluffyTermData::Curry {
                curry_kind: term.curry_kind(db),
                variance: term.variance(db),
                parameter_variable: term.parameter_variable(db).map(Into::into),
                parameter_ty: term.parameter_ty(db).into(),
                return_ty: term.return_ty(db).into(),
            },
            FluffyTerm::Ritchie(_) => todo!(),
            FluffyTerm::Abstraction(_) => todo!(),
            FluffyTerm::Application(_) => todo!(),
            FluffyTerm::Subentity(_) => todo!(),
            FluffyTerm::AsTraitSubentity(_) => todo!(),
            FluffyTerm::TraitConstraint(_) => todo!(),
            FluffyTerm::Solid(term) => term.data(fluffy_terms.solid_terms()).into(),
            FluffyTerm::Hollow(_) => todo!(),
        }
    }
}
