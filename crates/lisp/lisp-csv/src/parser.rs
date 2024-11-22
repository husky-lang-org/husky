use husky_text_protocol::char::TextCharIter;

pub(crate) struct LpCsvParser<'a> {
    pub(crate) input: &'a str,
    pub(crate) chars: TextCharIter<'a>,
}

impl<'a> LpCsvParser<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        Self {
            input: s,
            chars: TextCharIter::new(s),
        }
    }
}

impl<'a> LpCsvParser<'a> {
    pub(crate) fn ignore_whitespaces_and_tabs_and_blank_lines_and_comments(&mut self) {
        loop {
            self.chars.eat_chars_while(|c| c == ' ' || c == '\t');
            match self.chars.peek() {
                Some('#') => {
                    self.chars.eat_chars_while(|c| c != '\n');
                }
                Some('\n') => {
                    self.chars.eat_char();
                }
                _ => break,
            }
        }
    }

    pub(crate) fn ignore_whitespaces_and_tabs_and_comments(&mut self) {
        self.chars.eat_chars_while(|c| c == ' ' || c == '\t');
        match self.chars.peek() {
            Some('#') => {
                self.chars.eat_chars_while(|c| c != '\n');
            }
            _ => (),
        }
    }

    pub(crate) fn ignore_separators(&mut self) {
        loop {
            match self.chars.peek() {
                Some(c) if self.is_cell_separator(c) => self.chars.eat_char(),
                _ => break,
            }
        }
    }

    pub(crate) fn is_cell_separator(&self, c: char) -> bool {
        matches!(c, ',' | ';' | ':' | '=' | '|')
    }

    pub(crate) fn is_list_item_separator(&self, c: char) -> bool {
        matches!(c, ',' | ';' | ':')
    }
}
