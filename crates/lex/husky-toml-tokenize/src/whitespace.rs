use crate::*;

impl<'a> TokenIter<'a> {
    pub(crate) fn next_whitespace_token(&mut self) -> TomlTokenVariant {
        while self.try_eat_char(' ') || self.try_eat_char('\t') {
            // ...
        }
        TomlTokenVariant::Whitespace
    }
}
