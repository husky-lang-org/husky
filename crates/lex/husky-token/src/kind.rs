mod attribute;
mod keyword;
mod punctuation;
mod wordopr;

pub use attribute::*;
pub use keyword::*;
pub use punctuation::*;
pub use wordopr::*;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Attr(AttributeKeyword),
    Keyword(Keyword),
    Identifier(Identifier),
    Punctuation(Punctuation),
    WordOpr(WordOpr),
    Literal(LiteralToken),
    Comment,
    Err(TokenError),
}

impl std::hash::Hash for TokenKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
