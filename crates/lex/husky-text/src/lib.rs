mod db;
mod indent;
mod info;
mod line_map;
mod position;
mod range;
#[cfg(test)]
mod tests;

pub use indent::TextIndent;
pub use info::*;
pub use position::*;
pub use range::*;
pub type CharIter<'token_line> = std::iter::Peekable<Enumerate<Chars<'token_line>>>;
pub use db::TextDb;

use line_map::LineMap;
use std::{iter::Enumerate, ops::Deref, str::Chars, sync::Arc};

#[salsa::jar(db = TextDb)]
pub struct TextJar();

#[derive(Clone, PartialEq, Eq)]
pub struct Text {
    content: String,
    line_map: LineMap,
}

impl std::fmt::Debug for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Text...")
    }
}

impl std::ops::Index<TextRange> for Text {
    type Output = str;

    fn index(&self, index: TextRange) -> &Self::Output {
        todo!()
    }
}

impl std::ops::Index<std::ops::Range<(u32, u32)>> for Text {
    type Output = str;

    fn index(&self, index: std::ops::Range<(u32, u32)>) -> &Self::Output {
        self.text_within(index.into())
    }
}

impl Text {
    pub(crate) fn new(content: impl Into<String>) -> Self {
        let content: String = content.into();
        Self {
            line_map: LineMap::new(&content),
            content,
        }
    }

    pub fn text_within(&self, range: TextRange) -> &str {
        &self.content[self.line_map.offset_range(range)]
    }
}
