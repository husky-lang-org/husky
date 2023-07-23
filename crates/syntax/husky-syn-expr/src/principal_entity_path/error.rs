use super::*;
use original_error::IntoError;

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynExprDb)]
pub enum PrincipalEntityPathExprError {
    #[error("original `{0}`")]
    Original(OriginalPrincipalEntityPathExprError),
    #[error("derived `{0}`")]
    Derived(DerivedPrincipalEntityPathExprError),
}

impl From<TokenError> for PrincipalEntityPathExprError {
    fn from(value: TokenError) -> Self {
        PrincipalEntityPathExprError::Derived(value.into())
    }
}

impl From<OriginalPrincipalEntityPathExprError> for PrincipalEntityPathExprError {
    fn from(v: OriginalPrincipalEntityPathExprError) -> Self {
        Self::Original(v)
    }
}

impl From<DerivedPrincipalEntityPathExprError> for PrincipalEntityPathExprError {
    fn from(v: DerivedPrincipalEntityPathExprError) -> Self {
        Self::Derived(v)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynExprDb)]
pub enum OriginalPrincipalEntityPathExprError {
    #[error("entity tree")]
    EntityTree {
        token_idx: TokenIdx,
        error: EntityTreeError,
    },
    #[error("expect identifier after `::`")]
    ExpectIdentAfterScopeResolution(TokenStreamState),
}

impl IntoError for OriginalPrincipalEntityPathExprError {
    type Error = PrincipalEntityPathExprError;
}

impl From<OriginalExprError> for OriginalPrincipalEntityPathExprError {
    fn from(value: OriginalExprError) -> Self {
        todo!()
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = SynExprDb)]
pub enum DerivedPrincipalEntityPathExprError {
    #[error("derived from expr error {0}")]
    AbortFromExprError(#[from] OriginalExprError),
    #[error("token error {0}")]
    TokenError(#[from] TokenError),
}

pub type PrincipalEntityPathExprResult<T> = Result<T, PrincipalEntityPathExprError>;
