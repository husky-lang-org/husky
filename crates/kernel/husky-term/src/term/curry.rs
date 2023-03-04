pub use context::*;

use crate::*;

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = TermDb, jar = TermJar)]
pub struct TermCurry {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub input_symbol: Option<TermSymbol>,
    /// X
    pub input_ty: Term,
    /// Y
    pub return_ty: Term,
}

impl TermCurry {
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn TermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        let input_symbol = self.input_symbol(db);
        if input_symbol.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(input_symbol) = input_symbol {
            ctx.fmt_with_symbol(db, input_symbol, |ctx| {
                ctx.fmt_symbol(db, input_symbol, f);
                f.write_str(": ")?;
                self.input_ty(db).show_with_db_fmt(f, db, ctx)?;
                f.write_str(") -> ")?;
                self.return_ty(db).show_with_db_fmt(f, db, ctx)
            })
        } else {
            self.input_ty(db).show_with_db_fmt(f, db, ctx)?;
            f.write_str(" -> ")?;
            self.return_ty(db).show_with_db_fmt(f, db, ctx)
        }
    }
}

impl<Db: TermDb + ?Sized> salsa::DisplayWithDb<Db> for TermCurry {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        level: salsa::DisplayFormatLevel,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TermJar>>::as_jar_db(db);
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}

impl TermRewriteCopy for TermCurry {
    fn substitute_copy(self, db: &dyn TermDb, substituation: &TermSubstitution) -> Self {
        todo!()
    }
}
