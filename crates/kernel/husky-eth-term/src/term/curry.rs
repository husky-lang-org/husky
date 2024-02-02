mod utils;

pub(crate) use self::utils::*;

use super::*;

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = EthTermDb, jar = EthTermJar)]
pub struct EthCurry {
    pub toolchain: Toolchain,
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_rune: Option<EthRune>,
    /// X
    pub parameter_ty: EthTerm,
    /// Y
    pub return_ty: EthTerm,
}

#[test]
fn term_curry_size_works() {
    assert_eq!(std::mem::size_of::<EthCurry>(), std::mem::size_of::<u32>());
}

impl EthCurry {
    pub(crate) fn from_declarative(
        db: &::salsa::Db,
        declarative_term_curry: DecCurry,
    ) -> EthTermResult<Self> {
        term_curry_from_declarative(db, declarative_term_curry)
    }

    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        let parameter_rune = self.parameter_rune(db);
        if parameter_rune.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(parameter_rune) = parameter_rune {
            ctx.fmt_with_variable(db, parameter_rune, |ctx| {
                ctx.fmt_variable(db, parameter_rune, f)?;
                f.write_str(": ")?;
                self.parameter_ty(db)
                    .display_fmt_with_db_and_ctx(f, db, ctx)?;
                f.write_str(") -> ")?;
                self.return_ty(db).display_fmt_with_db_and_ctx(f, db, ctx)
            })
        } else {
            self.parameter_ty(db)
                .display_fmt_with_db_and_ctx(f, db, ctx)?;
            f.write_str(" -> ")?;
            self.return_ty(db).display_fmt_with_db_and_ctx(f, db, ctx)
        }
    }
}

/// # rewrite

impl EthCurry {
    pub fn substitute(self, substitution: EthTermSubstitution, db: &::salsa::Db) -> Self {
        let parameter_rune = self.parameter_rune(db);
        if parameter_rune == Some(substitution.src()) {
            return self;
        }
        Self::new(
            db,
            self.toolchain(db),
            self.curry_kind(db),
            self.variance(db),
            parameter_rune.map(|rune| rune.substitute_intact(substitution, db)),
            self.parameter_ty(db),
            self.return_ty(db),
        )
    }
}

impl EthTermInstantiate for EthCurry {
    type Output = Self;

    fn instantiate(self, db: &salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        Self::new(
            db,
            self.toolchain(db),
            self.curry_kind(db),
            self.variance(db),
            self.parameter_rune(db).instantiate(db, instantiation),
            self.parameter_ty(db).instantiate(db, instantiation),
            self.return_ty(db).instantiate(db, instantiation),
        )
    }
}

#[salsa::tracked(jar = EthTermJar)]
pub(crate) fn term_curry_from_declarative(
    db: &::salsa::Db,
    curry: DecCurry,
) -> EthTermResult<EthCurry> {
    let t = |declarative_ty| EthTerm::ty_from_declarative(db, declarative_ty);
    Ok(EthCurry::new(
        db,
        curry.toolchain(db),
        curry.curry_kind(db),
        curry.variance(db),
        match curry.parameter_rune(db) {
            Some(parameter_rune) => Some(EthRune::from_declarative(db, parameter_rune)?),
            None => None,
        },
        t(curry.parameter_ty(db))?,
        t(curry.return_ty(db))?,
    ))
}

impl salsa::DisplayWithDb for EthCurry {
    fn display_fmt_with_db(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.display_fmt_with_db_and_ctx(f, db, &mut Default::default())
    }
}
