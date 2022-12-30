use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PunctuationToken {
    punc: Punctuation,
    token_idx: TokenIdx,
}

impl<Db> salsa::DebugWithDb<Db> for PunctuationToken
where
    Db: TokenDb + ?Sized,
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        let db = <Db as salsa::DbWithJar<TokenJar>>::as_jar_db(db);
        f.debug_struct("PunctuationToken")
            .field("punc", &self.punc)
            .field("token_idx", &self.token_idx)
            .finish()
    }
}

impl<'a, Context> parsec::ParseFrom<Context> for PunctuationToken
where
    Context: TokenParseContext<'a>,
{
    type Error = TokenError;

    fn parse_from(ctx: &mut Context) -> Result<Option<Self>, Self::Error> {
        if let Some((token_idx, token)) = ctx.token_iter_mut().next_indexed(IgnoreComment::True) {
            match token.kind {
                TokenKind::Punctuation(punc) => Ok(Some(PunctuationToken { punc, token_idx })),
                TokenKind::Comment => unreachable!(),
                TokenKind::Err(ref e) => Err(e.clone()),
                TokenKind::Identifier(_)
                | TokenKind::WordOpr(_)
                | TokenKind::Literal(_)
                | TokenKind::Attr(_)
                | TokenKind::Keyword(_) => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
