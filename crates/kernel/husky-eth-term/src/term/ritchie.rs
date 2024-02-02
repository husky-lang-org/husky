mod keyed;
mod regular;
mod variadic;

pub use self::keyed::*;
pub use self::regular::*;
pub use self::variadic::*;

use super::*;

/// representing term `x -> y`
#[salsa::interned(db = EthTermDb, jar = EthTermJar, constructor = new_inner)]
pub struct RitchieEthTerm {
    pub ritchie_kind: RitchieKind,
    #[return_ref]
    pub parameter_contracted_tys: Vec<EtherealRitchieParameter>,
    pub return_ty: EthTerm,
}

#[test]
fn term_ritchie_size_works() {
    assert_eq!(
        std::mem::size_of::<RitchieEthTerm>(),
        std::mem::size_of::<u32>()
    );
}

impl RitchieEthTerm {
    //// this constructor guarantees that the result is reduced and first-order valid
    /// returns EthTerm instead of EthTermApplication because it might reduce to a non application term
    pub fn new(
        db: &::salsa::Db,
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: impl IntoIterator<Item = EtherealRitchieParameter>,
        return_ty: EthTerm,
    ) -> EthTermResult<RitchieEthTerm> {
        // todo!("check_application_validity(db, function, argument, shift)?");
        Ok(Self::new_unchecked(
            db,
            ritchie_kind,
            parameter_contracted_tys,
            return_ty,
        ))
    }

    /// this constructor guarantees that the result is reduced, not necessarily valid;
    ///
    /// returns EthTerm instead of EthTermApplication because it might reduce to a non application term
    pub(crate) fn new_unchecked(
        db: &::salsa::Db,
        ritchie_kind: RitchieKind,
        parameter_tys: impl IntoIterator<Item = EtherealRitchieParameter>,
        return_ty: EthTerm,
    ) -> RitchieEthTerm {
        Self::new_inner(
            db,
            ritchie_kind,
            parameter_tys
                .into_iter()
                .map(|parameter_contracted_ty| parameter_contracted_ty.reduce(db))
                .collect(),
            return_ty.reduce(db),
        )
    }
    /// this constructor guarantees that the result is reduced, not necessarily valid
    ///
    /// returns EthTerm instead of EthTermApplication because it might reduce to a non application term
    fn new_unchecked2<E>(
        db: &::salsa::Db,
        ritchie_kind: RitchieKind,
        parameter_tys: impl IntoIterator<Item = Result<EtherealRitchieParameter, E>>,
        return_ty: EthTerm,
    ) -> EthTermResult<RitchieEthTerm>
    where
        EthTermError: From<E>,
    {
        Ok(Self::new_inner(
            db,
            ritchie_kind,
            parameter_tys
                .into_iter()
                .map(|parameter_contracted_ty| Ok(parameter_contracted_ty?.reduce(db)))
                .collect::<EthTermResult<Vec<_>>>()?,
            return_ty.reduce(db),
        ))
    }

    pub(super) fn reduce(self, _db: &::salsa::Db) -> RitchieEthTerm {
        // ad hoc
        self
    }

    #[inline(always)]
    pub fn from_declarative(
        db: &::salsa::Db,
        declarative_term_ritchie: RitchieDecTerm,
    ) -> EthTermResult<Self> {
        ethereal_term_ritchie_from_declarative_term_ritchie(db, declarative_term_ritchie)
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        f.write_str(self.ritchie_kind(db).code())?;
        f.write_str("(")?;
        for (i, parameter_contracted_ty) in self.parameter_contracted_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_contracted_ty.show_with_db_fmt(f, db, ctx)?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).show_with_db_fmt(f, db, ctx)
    }
}

#[salsa::tracked(jar = EthTermJar)]
pub(crate) fn ethereal_term_ritchie_from_declarative_term_ritchie(
    db: &::salsa::Db,
    declarative_term_ritchie: RitchieDecTerm,
) -> EthTermResult<RitchieEthTerm> {
    RitchieEthTerm::new_unchecked2(
        db,
        declarative_term_ritchie.ritchie_kind(db),
        declarative_term_ritchie
            .params(db)
            .iter()
            .map(|&param| -> EthTermResult<_> {
                EtherealRitchieParameter::from_declarative(param, db)
            }),
        EthTerm::ty_from_declarative(db, declarative_term_ritchie.return_ty(db))?,
    )
}

impl EtherealRitchieParameter {
    pub fn from_declarative(
        param: DeclarativeRitchieParameter,
        db: &::salsa::Db,
    ) -> EthTermResult<Self> {
        Ok(match param {
            DeclarativeRitchieParameter::Regular(param) => {
                EthRitchieRegularParameter::from_declarative(db, param)?.into()
            }
            DeclarativeRitchieParameter::Variadic(param) => {
                EtherealRitchieVariadicParameter::from_declarative(db, param)?.into()
            }
            DeclarativeRitchieParameter::Keyed(param) => {
                EtherealRitchieKeyedParameter::from_declarative(db, param)?.into()
            }
        })
    }
}

impl EthTermInstantiate for EtherealRitchieParameter {
    type Output = Self;

    fn instantiate(
        self,
        _db: &::salsa::Db,
        _instantiation: &EtherealInstantiation,
    ) -> Self::Output {
        todo!()
    }
}

impl salsa::DisplayWithDb for RitchieEthTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str(self.ritchie_kind(db).code())?;
        f.write_str("(")?;
        for (i, parameter_ty) in self.parameter_contracted_tys(db).iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?
            }
            parameter_ty.display_with_db_fmt(f, db)?
        }
        f.write_str(") -> ")?;
        self.return_ty(db).display_with_db_fmt(f, db)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
#[enum_class::from_variants]
pub enum EtherealRitchieParameter {
    Regular(EthRitchieRegularParameter),
    Variadic(EtherealRitchieVariadicParameter),
    Keyed(EtherealRitchieKeyedParameter),
}

impl EtherealRitchieParameter {
    fn reduce(self, db: &::salsa::Db) -> Self {
        match self {
            EtherealRitchieParameter::Regular(param) => param.reduce(db).into(),
            EtherealRitchieParameter::Variadic(param) => param.reduce(db).into(),
            EtherealRitchieParameter::Keyed(param) => param.reduce(db).into(),
        }
    }

    fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        match self {
            EtherealRitchieParameter::Regular(param) => param.show_with_db_fmt(f, db, ctx),
            EtherealRitchieParameter::Variadic(param) => param.show_with_db_fmt(f, db, ctx),
            EtherealRitchieParameter::Keyed(param) => param.show_with_db_fmt(f, db, ctx),
        }
    }
}

impl salsa::DisplayWithDb for EtherealRitchieParameter {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        match self {
            EtherealRitchieParameter::Regular(param) => param.display_with_db_fmt(f, db),
            EtherealRitchieParameter::Variadic(param) => param.display_with_db_fmt(f, db),
            EtherealRitchieParameter::Keyed(param) => param.display_with_db_fmt(f, db),
        }
    }
}

impl EtherealRitchieParameter {
    pub fn ty(&self) -> EthTerm {
        match self {
            EtherealRitchieParameter::Regular(param) => param.ty(),
            EtherealRitchieParameter::Variadic(param) => param.ty(),
            EtherealRitchieParameter::Keyed(param) => param.ty(),
        }
    }
}

/// # rewrite

impl RitchieEthTerm {
    pub(super) fn substitute(
        self,
        substitution: EthTermSubstitution,
        db: &::salsa::Db,
    ) -> RitchieEthTerm {
        Self::new_inner(
            db,
            self.ritchie_kind(db),
            self.parameter_contracted_tys(db)
                .substitute(substitution, db)
                .collect(),
            self.return_ty(db).substitute(substitution, db),
        )
    }
}

impl<'a> EthTermSubstitute<'a> for EtherealRitchieParameter {
    type Output = EtherealRitchieParameter;

    fn substitute(self, _substitution: EthTermSubstitution, _db: &'a salsa::Db) -> Self::Output {
        todo!()
    }
}

impl EthTermInstantiate for RitchieEthTerm {
    type Output = Self;

    fn instantiate(self, db: &salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        Self::new_inner(
            db,
            self.ritchie_kind(db),
            self.parameter_contracted_tys(db)
                .instantiate(db, instantiation),
            self.return_ty(db).instantiate(db, instantiation),
        )
    }
}