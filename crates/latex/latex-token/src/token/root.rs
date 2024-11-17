use super::*;
use latex_command::path::LxCommandName;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxRootTokenData {
    Command(LxCommandName),
    LeftDelimiter(LxRootDelimiter),
    RightDelimiter(LxRootDelimiter),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LxRootDelimiter {
    /// `{`,  `}`
    Curl,
    /// `[`, `]`
    Box,
}

impl<'a> LxLexer<'a> {
    pub(crate) fn next_root_token_data(&mut self) -> Option<LxRootTokenData> {
        let db = self.db;
        match self.chars.peek()? {
            '\\' => {
                self.chars.eat_char();
                match self.chars.peek() {
                    Some(c) => match c {
                        c if c.is_ascii_alphabetic() => Some(LxRootTokenData::Command(
                            LxCommandName::new(
                                self.next_coword_with(|c| c.is_ascii_alphabetic()).unwrap(),
                                db,
                            )
                            .unwrap(),
                        )),
                        c if c.is_numeric() => todo!("latex might allow single digit command"),
                        _ => todo!("latex one digit non letter command"),
                    },
                    None => todo!(),
                }
            }
            '{' => {
                self.chars.eat_char();
                Some(LxRootTokenData::LeftDelimiter(LxRootDelimiter::Curl))
            }
            '}' => {
                self.chars.eat_char();
                Some(LxRootTokenData::RightDelimiter(LxRootDelimiter::Curl))
            }
            '[' => {
                self.chars.eat_char();
                Some(LxRootTokenData::LeftDelimiter(LxRootDelimiter::Box))
            }
            ']' => {
                self.chars.eat_char();
                Some(LxRootTokenData::RightDelimiter(LxRootDelimiter::Box))
            }
            _ => todo!(),
        }
    }
}

#[test]
pub fn next_root_token_data_works() {
    fn t(input: &str, expected: &Expect) {
        let db = &DB::default();
        let mut storage = LxTokenStorage::default();
        let stream = LxLexer::new(db, input, &mut storage)
            .into_root_stream()
            .map(|(_, token_data)| token_data);
        let tokens: Vec<_> = stream.collect();
        expected.assert_debug_eq(&(tokens.debug(db)));
    }
    t(
        "\\usepackage",
        &expect![[r#"
            [
                LxRootTokenData::Command(
                    LxCommandName::LettersOnly(
                        LettersOnlyLxCommandName(
                            Coword(
                                "usepackage",
                            ),
                        ),
                    ),
                ),
            ]
        "#]],
    );
    t(
        "{",
        &expect![[r#"
            [
                LxRootTokenData::LeftDelimiter(
                    Curl,
                ),
            ]
        "#]],
    );
    t(
        "}",
        &expect![[r#"
            [
                LxRootTokenData::RightDelimiter(
                    Curl,
                ),
            ]
        "#]],
    );
}