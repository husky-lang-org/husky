use super::*;

#[salsa::interned(db = EthTermDb, jar = EthTermJar, constructor = new_inner)]
pub struct EthLambdaVariable {
    pub ty: EthTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub index: LambdaVariableIndex,
}

impl EthLambdaVariable {
    #[inline(always)]
    pub(crate) fn from_dec(db: &::salsa::Db, hvar: DecLambdaVariable) -> EthTermResult<Self> {
        let ty = hvar.ty(db)?;
        let ty = EthTerm::ty_from_dec(db, ty)?;
        Ok(Self::new_inner(db, ty, hvar.index(db)))
    }

    #[inline(never)]
    pub(crate) fn display_fmt_with_db(
        self,
        f: &mut std::fmt::Formatter<'_>,
        _db: &::salsa::Db,
    ) -> std::fmt::Result {
        f.write_str("variable_ad_hoc_fmt")
    }
}

impl EthTerm {
    #[track_caller]
    pub fn hvar(self) -> EthLambdaVariable {
        match self {
            EthTerm::LambdaVariable(slf) => slf,
            _ => unreachable!(),
        }
    }
}

/// # rewrite

impl EthLambdaVariable {
    pub fn substitute(self, substitution: EthTermSubstitution, db: &salsa::Db) -> EthTerm {
        if self == substitution.src() {
            return substitution.dst();
        }
        self.substitute_intact(substitution, db).into()
    }

    pub fn substitute_intact(
        self,
        substitution: EthTermSubstitution,
        db: &salsa::Db,
    ) -> EthLambdaVariable {
        Self::new_inner(db, self.ty(db).substitute(substitution, db), self.index(db))
    }
}

impl EthInstantiate for EthLambdaVariable {
    type Output = Self;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EthInstantiation) -> Self::Output {
        // it's assumed that all symbols will be replaced by its map
        // otherwise it's illegal
        Self::new_inner(
            db,
            self.ty(db).instantiate(db, instantiation),
            self.index(db),
        )
    }
}

/// back to declarative
impl EthLambdaVariable {
    pub(super) fn into_declarative(self, db: &salsa::Db) -> DecLambdaVariable {
        DecLambdaVariable::new(
            Ok(self.ty(db).into_declarative(db)),
            self.index(db).disambiguator(),
            db,
        )
    }
}
