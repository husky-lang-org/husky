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

    pub fn data_inner<'a>(
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

// todo: replace this with macro
impl<'a, _Db: TermDb + ?Sized> ::salsa::DebugWithDb<_Db> for FluffyTermData<'a> {
    fn fmt(
        &self,
        f: &mut ::std::fmt::Formatter<'_>,
        _db: &_Db,
        _level: ::salsa::DebugFormatLevel,
    ) -> ::std::fmt::Result {
        #[allow(unused_imports)]
        use ::salsa::debug::helper::Fallback;
        match self {
            FluffyTermData::Literal(ref v0) => {
                let mut debug_tuple = &mut f.debug_tuple("FluffyTermData::Literal");
                debug_tuple = debug_tuple.field(&::salsa::debug::helper::SalsaDebug::<
                    TermLiteral,
                    _Db,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &v0,
                    _db,
                    _level.next(),
                ));
                debug_tuple.finish()
            }
            FluffyTermData::TypeOntology {
                ref path,
                ref refined_path,
                ref argument_tys,
            } => {
                let mut debug_struct = &mut f.debug_struct("FluffyTermData::TypeOntology");
                debug_struct = debug_struct.field(
                    "path",
                    &::salsa::debug::helper::SalsaDebug::<TypePath, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        path,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "refined_path",
                    &::salsa::debug::helper::SalsaDebug::<
                        Either<CustomTypePath, PreludeTypePath>,
                        _Db,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        refined_path,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "argument_tys",
                    &::salsa::debug::helper::SalsaDebug::<&'a [FluffyTerm], _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        argument_tys,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct.finish()
            }
            FluffyTermData::PlaceTypeOntology {
                ref place,
                ref path,
                ref refined_path,
                ref argument_tys,
            } => {
                let mut debug_struct = &mut f.debug_struct("FluffyTermData::PlaceTypeOntology");
                debug_struct = debug_struct.field(
                    "place",
                    &::salsa::debug::helper::SalsaDebug::<Place, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        place,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "path",
                    &::salsa::debug::helper::SalsaDebug::<TypePath, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        path,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "refined_path",
                    &::salsa::debug::helper::SalsaDebug::<
                        Either<CustomTypePath, PreludeTypePath>,
                        _Db,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        refined_path,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "argument_tys",
                    &::salsa::debug::helper::SalsaDebug::<&'a [FluffyTerm], _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        argument_tys,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct.finish()
            }
            FluffyTermData::Curry {
                ref curry_kind,
                ref variance,
                ref parameter_variable,
                ref parameter_ty,
                ref return_ty,
            } => {
                let mut debug_struct = &mut f.debug_struct("FluffyTermData::Curry");
                debug_struct = debug_struct.field(
                    "curry_kind",
                    &::salsa::debug::helper::SalsaDebug::<CurryKind, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        curry_kind,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "variance",
                    &::salsa::debug::helper::SalsaDebug::<Variance, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        variance,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "parameter_variable",
                    &::salsa::debug::helper::SalsaDebug::<Option<FluffyTerm>, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        parameter_variable,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "parameter_ty",
                    &::salsa::debug::helper::SalsaDebug::<FluffyTerm, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        parameter_ty,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "return_ty",
                    &::salsa::debug::helper::SalsaDebug::<FluffyTerm, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        return_ty,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct.finish()
            }
            FluffyTermData::Hole(ref v0, ref v1) => {
                let mut debug_tuple = &mut f.debug_tuple("FluffyTermData::Hole");
                debug_tuple = debug_tuple.field(
                    &::salsa::debug::helper::SalsaDebug::<HoleKind, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        &v0,
                        _db,
                        _level.next(),
                    ),
                );
                debug_tuple = debug_tuple.field(&::salsa::debug::helper::SalsaDebug::<
                    HollowTerm,
                    _Db,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &v1,
                    _db,
                    _level.next(),
                ));
                debug_tuple.finish()
            }
            FluffyTermData::Category(ref v0) => {
                let mut debug_tuple = &mut f.debug_tuple("FluffyTermData::Category");
                debug_tuple = debug_tuple.field(&::salsa::debug::helper::SalsaDebug::<
                    TermCategory,
                    _Db,
                >::salsa_debug(
                    #[allow(clippy::needless_borrow)]
                    &v0,
                    _db,
                    _level.next(),
                ));
                debug_tuple.finish()
            }
            FluffyTermData::Ritchie {
                ref ritchie_kind,
                ref parameter_contracted_tys,
                ref return_ty,
            } => {
                let mut debug_struct = &mut f.debug_struct("FluffyTermData::Ritchie");
                debug_struct = debug_struct.field(
                    "ritchie_kind",
                    &::salsa::debug::helper::SalsaDebug::<TermRitchieKind, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        ritchie_kind,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "parameter_contracted_tys",
                    &::salsa::debug::helper::SalsaDebug::<
                        &'a [FluffyTermRitchieParameterContractedType],
                        _Db,
                    >::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        parameter_contracted_tys,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct = debug_struct.field(
                    "return_ty",
                    &::salsa::debug::helper::SalsaDebug::<FluffyTerm, _Db>::salsa_debug(
                        #[allow(clippy::needless_borrow)]
                        return_ty,
                        _db,
                        _level.next(),
                    ),
                );
                debug_struct.finish()
            }
        }
    }
}
