use crate::{line_token_iter::LineTokenIter, token::TokenKind, tokenized_text::TokenGroup, *};

use text::Indent;

#[derive(Debug, PartialEq, Eq)]
pub struct TokenizedLine {
    pub(crate) indent: Indent,
    pub(crate) tokens: Range,
}

#[derive(Debug)]
pub(crate) struct TokenScanner<'lex> {
    db: &'lex dyn LexQuery,
    tokens: Vec<Token>,
    tokenized_lines: Vec<TokenizedLine>,
    errors: Vec<LexError>,
}

impl<'lex> TokenScanner<'lex> {
    pub(crate) fn new(db: &'lex dyn LexQuery) -> Self {
        Self {
            db,
            tokens: vec![],
            tokenized_lines: vec![],
            errors: vec![],
        }
    }

    pub(crate) fn scan(&mut self, line_index: usize, line: &str) {
        let start = self.tokens.len();
        let (indent, token_iter) =
            LineTokenIter::new(self.db, line_index, line.chars().enumerate().peekable());
        self.tokens.extend(token_iter);
        let end = self.tokens.len();
        self.tokenized_lines.push(TokenizedLine {
            indent,
            tokens: start..end,
        })
    }

    fn last_token(&self, line: &TokenizedLine) -> &Token {
        &self.tokens[line.tokens.end - 1]
    }

    fn first_token(&self, line: &TokenizedLine) -> &Token {
        &self.tokens[line.tokens.start]
    }

    fn produce_line_groups(&mut self) -> Vec<TokenGroup> {
        let mut line_groups = Vec::new();
        line_groups.reserve(self.tokenized_lines.len());
        let mut line_iter = self
            .tokenized_lines
            .iter()
            .filter(|line| line.tokens.len() > 0)
            .peekable();
        while let Some(first_line) = line_iter.next() {
            line_groups.push({
                let group_indent = first_line.indent;
                TokenGroup {
                    indent: group_indent,
                    tokens: first_line.tokens.start..{
                        if self.last_token(first_line).kind != TokenKind::Special(Special::Colon) {
                            loop {
                                if let Some(line) = line_iter.peek().map(|e| *e) {
                                    if line.indent.within(group_indent).expect("todo") {
                                        line_iter.next();
                                    } else {
                                        fn bind_to_last_line(kind: TokenKind) -> bool {
                                            match kind {
                                                TokenKind::Special(special) => match special {
                                                    Special::RCurl => true,
                                                    Special::RBox => true,
                                                    Special::RPar => true,
                                                    _ => false,
                                                },
                                                _ => false,
                                            }
                                        }

                                        if bind_to_last_line(self.first_token(line).kind) {
                                            line_iter.next();
                                            break line.tokens.end;
                                        } else {
                                            break line.tokens.start;
                                        }
                                    }
                                } else {
                                    break self.tokenized_lines.len();
                                }
                            }
                        } else {
                            if let Some(line) = line_iter.peek() {
                                if !line.indent.within(group_indent).expect("todo") {
                                    todo!()
                                }
                            }
                            first_line.tokens.end
                        }
                    },
                    folding_end: 0,
                }
            });
        }
        line_groups
    }
}

impl<'lex> Into<TokenizedText> for TokenScanner<'lex> {
    fn into(mut self) -> TokenizedText {
        TokenizedText::new(self.produce_line_groups(), self.tokens, self.errors)
    }
}
