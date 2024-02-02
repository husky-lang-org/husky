use super::*;

/// representing declarative_term `X -> Y` or dependent form `(a: X) -> Y(a)`
///
/// refraining from using `new_inner` except in conversion from ethereal term back to declarative term
#[salsa::interned(db = DecTermDb, jar = DecTermJar, constructor = pub new_inner)]
pub struct CurryDecTerm {
    pub toolchain: Toolchain,
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_rune: Option<RuneDecTerm>,
    /// X
    pub parameter_ty: DecTerm,
    /// Y
    pub return_ty: DecTerm,
}

impl CurryDecTerm {
    /// create a new term curry by converting a symbol to variable
    /// this is the only way to create a new dependent curry declarative term
    /// so that cache hit is maximized
    pub fn new_dependent(
        db: &::salsa::Db,
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        // to be converted to variable
        parameter_symbol: SymbolDecTerm,
        parameter_ty: DecTerm,
        return_ty: DecTerm,
    ) -> Self {
        let (return_ty, parameter_rune) = return_ty.create_rune(db, parameter_symbol);
        CurryDecTerm::new_inner(
            db,
            toolchain,
            curry_kind,
            variance,
            parameter_rune,
            parameter_ty,
            return_ty,
        )
    }

    pub fn new_nondependent(
        db: &::salsa::Db,
        toolchain: Toolchain,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_ty: DecTerm,
        return_ty: DecTerm,
    ) -> Self {
        CurryDecTerm::new_inner(
            db,
            toolchain,
            curry_kind,
            variance,
            None,
            parameter_ty,
            return_ty,
        )
    }

    pub(super) fn substitute_symbol_with_variable(
        self,
        db: &::salsa::Db,
        symbol: SymbolDecTerm,
        variable: RuneDecTerm,
    ) -> Self {
        CurryDecTerm::new_inner(
            db,
            symbol.toolchain(db),
            self.curry_kind(db),
            self.variance(db),
            self.parameter_rune(db),
            self.parameter_ty(db)
                .substitute_symbol_with_variable(db, symbol, variable),
            self.return_ty(db)
                .substitute_symbol_with_variable(db, symbol, variable),
        )
    }

    pub fn return_ty_with_variable_substituted(
        self,
        db: &::salsa::Db,
        substitute: DecTerm,
    ) -> DecTerm {
        match self.parameter_rune(db) {
            Some(parameter_rune) => self
                .return_ty(db)
                .substitute_copy(db, &DecTermSubstitution::new(parameter_rune, substitute)),
            None => self.return_ty(db),
        }
    }

    #[inline(never)]
    pub(crate) fn display_fmt_with_db_and_ctx(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &SymbolDecTermNameMap,
    ) -> std::fmt::Result {
        use salsa::DisplayWithDb;

        let parameter_rune = self.parameter_rune(db);
        if parameter_rune.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(parameter_rune) = parameter_rune {
            f.write_str("(")?;
            parameter_rune.display_fmt_with_db(f, db)?;
            f.write_str(": ")?;
            self.parameter_ty(db)
                .display_fmt_with_db_and_ctx(f, db, ctx)?;
            f.write_str(") -> ")?;
            self.return_ty(db).display_fmt_with_db_and_ctx(f, db, ctx)
        } else {
            self.parameter_ty(db)
                .display_fmt_with_db_and_ctx(f, db, ctx)?;
            f.write_str(" -> ")?;
            self.return_ty(db).display_fmt_with_db_and_ctx(f, db, ctx)
        }
    }
}

#[salsa::tracked(jar = DecTermJar)]
pub(crate) fn curry_parameter_count(db: &::salsa::Db, term: CurryDecTerm) -> u8 {
    term.return_ty(db).curry_parameter_count(db) + 1
}

impl DecTermRewriteCopy for CurryDecTerm {
    fn substitute_copy(self, db: &::salsa::Db, substitution: &DecTermSubstitution) -> Self {
        let old_parameter_variable = self.parameter_rune(db);
        let parameter_rune = old_parameter_variable.map(|v| v.substitute_copy(db, substitution));
        let old_parameter_ty = self.parameter_ty(db);
        let parameter_ty = old_parameter_ty.substitute_copy(db, substitution);
        let old_return_ty = self.return_ty(db);
        let return_ty = old_return_ty.substitute_copy(db, substitution);
        if old_parameter_variable == parameter_rune
            && old_parameter_ty == parameter_ty
            && old_return_ty == return_ty
        {
            return self;
        }
        Self::new_inner(
            db,
            self.toolchain(db),
            self.curry_kind(db),
            self.variance(db),
            parameter_rune,
            parameter_ty,
            return_ty,
        )
    }
}
