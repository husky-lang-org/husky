use super::*;
use context::*;

/// representing precise_term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = RawTermDb, jar = RawTermJar)]
pub struct RawTermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_symbol: Option<RawTermSymbol>,
    /// X
    pub parameter_ty: RawTerm,
    /// Y
    pub return_ty: RawTerm,
}

impl RawTermCurry {
    pub fn from_raw_unchecked(
        db: &dyn RawTermDb,
        raw_term_curry: RawTermCurry,
        raw_ty_expectation: TermTypeExpectation,
    ) -> RawTermResult<Self> {
        match raw_ty_expectation {
            TermTypeExpectation::FinalDestinationEqsNonSortTypePath(_) => {
                return Err(RawTermError::ExpectationNotMatchedForCurry)
            }
            TermTypeExpectation::FinalDestinationEqsSort | TermTypeExpectation::Any => (),
        }
        precise_term_curry_from_raw(db, raw_term_curry)
    }

    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn RawTermDb,
        ctx: &mut RawTermShowContext,
    ) -> std::fmt::Result {
        let parameter_symbol = self.parameter_symbol(db);
        if parameter_symbol.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(parameter_symbol) = parameter_symbol {
            ctx.fmt_with_symbol(db, parameter_symbol, |ctx| {
                ctx.fmt_symbol(db, parameter_symbol, f);
                f.write_str(": ")?;
                self.parameter_ty(db).show_with_db_fmt(f, db, ctx)?;
                f.write_str(") -> ")?;
                self.return_ty(db).show_with_db_fmt(f, db, ctx)
            })
        } else {
            self.parameter_ty(db).show_with_db_fmt(f, db, ctx)?;
            f.write_str(" -> ")?;
            self.return_ty(db).show_with_db_fmt(f, db, ctx)
        }
    }
}

#[salsa::tracked(jar = RawTermJar)]
pub(crate) fn precise_term_curry_from_raw(
    db: &dyn RawTermDb,
    raw_term_curry: RawTermCurry,
) -> RawTermResult<RawTermCurry> {
    let t = |raw_ty| {
        RawTerm::from_raw_unchecked(db, raw_ty, TermTypeExpectation::FinalDestinationEqsSort)
    };
    Ok(RawTermCurry::new(
        db,
        raw_term_curry.curry_kind(db),
        raw_term_curry.variance(db),
        match raw_term_curry.parameter_symbol(db) {
            Some(parameter_symbol) => Some(RawTermSymbol::from_raw_unchecked(
                db,
                parameter_symbol,
                TermTypeExpectation::Any,
            )?),
            None => None,
        },
        t(raw_term_curry.parameter_ty(db))?,
        t(raw_term_curry.return_ty(db))?,
    ))
}

impl<Db: RawTermDb + ?Sized> salsa::DisplayWithDb<Db> for RawTermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<RawTermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl RawTermRewriteCopy for RawTermCurry {
    fn substitute(self, db: &dyn RawTermDb, substituation: &RawTermSubstitution) -> Self {
        todo!()
    }
}
