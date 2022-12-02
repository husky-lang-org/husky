mod comment;
mod db;
mod hex;
mod iter;
mod keylike;
mod manifest;
mod string;
#[cfg(test)]
mod tests;
mod whitespace;

pub use db::TomlTokenizeDb;

use husky_text_span::TextSpan;
use husky_toml_token::*;
use husky_word::{Word, WordDb};
use iter::*;
use keylike::is_keylike;
use manifest::toml_token_text;
use std::sync::Arc;

#[salsa::jar(db = TomlTokenizeDb)]
pub struct TomlTokenizeJar(toml_token_text);
