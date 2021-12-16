use crate::*;

use file::{FileError, FileRange, FileResultArc};
use std::sync::Arc;
#[salsa::query_group(TokenQueryStorage)]
pub trait TokenSalsaQuery: file::FileQuery + word::InternWord {
    fn tokenized_text(&self, id: file::FileId) -> FileResultArc<TokenizedText>;
}

fn tokenized_text(this: &dyn TokenSalsaQuery, id: file::FileId) -> FileResultArc<TokenizedText> {
    if let Some(text) = this.text(id) {
        return Ok(Arc::new(TokenizedText::parse(
            this.word_interner(),
            text.as_str(),
        )));
    } else {
        Err(FileError::FileNotFound)
    }
}
