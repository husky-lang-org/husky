#[cfg(feature = "lsp_support")]
mod lsp;
mod text_bytes_len;
mod utf16;

pub use text_bytes_len::*;

use rustc_hash::FxHashMap;
use std::iter;
use utf16::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineMap {
    /// Offset the the beginning of each line, zero-based
    pub(crate) newlines: Vec<usize>,
    /// List of non-ASCII characters on each line
    pub(crate) utf16_lines: FxHashMap<usize, Vec<Utf16Char>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextPositionUtf16 {
    /// Zero-based
    pub line: usize,
    /// Zero-based
    pub col: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextPosition {
    /// Zero-based
    pub line: usize,
    /// Zero-based utf8 offset
    pub col: usize,
}

impl LineMap {
    pub fn new(text: &str) -> LineMap {
        let mut utf16_lines = FxHashMap::default();
        let mut utf16_chars = Vec::new();

        let mut newlines = vec![0];
        let mut curr_row = 0;
        let mut curr_col = 0;
        let mut line = 0;
        for c in text.chars() {
            let c_len = c.text_bytes_len();
            curr_row += c_len;
            if c == '\n' {
                newlines.push(curr_row);

                // Save any utf-16 characters seen in the previous line
                if !utf16_chars.is_empty() {
                    utf16_lines.insert(line, utf16_chars);
                    utf16_chars = Vec::new();
                }

                // Prepare for processing the next line
                curr_col = 0;
                line += 1;
                continue;
            }

            if !c.is_ascii() {
                utf16_chars.push(Utf16Char {
                    start: curr_col,
                    end: curr_col + c_len,
                });
            }

            curr_col += c_len;
        }

        // Save any utf-16 characters seen in the last line
        if !utf16_chars.is_empty() {
            utf16_lines.insert(line, utf16_chars);
        }

        LineMap {
            newlines,
            utf16_lines,
        }
    }

    pub fn line_col(&self, offset: usize) -> TextPosition {
        let line = self.newlines.partition_point(|&it| it <= offset) - 1;
        let line_start_offset = self.newlines[line];
        let col = offset - line_start_offset;
        TextPosition {
            line: line as usize,
            col: col.into(),
        }
    }

    pub fn offset(&self, line_col: TextPosition) -> usize {
        self.newlines[line_col.line as usize] + usize::from(line_col.col)
    }

    pub fn to_utf16(&self, line_col: TextPosition) -> TextPositionUtf16 {
        let col = self.utf8_to_utf16_col(line_col.line, line_col.col.into());
        TextPositionUtf16 {
            line: line_col.line,
            col: col as usize,
        }
    }

    pub fn to_utf8(&self, line_col: TextPositionUtf16) -> TextPosition {
        let col = self.utf16_to_utf8_col(line_col.line, line_col.col);
        TextPosition {
            line: line_col.line,
            col: col.into(),
        }
    }

    pub fn lines(
        &self,
        range: std::ops::Range<usize>,
    ) -> impl Iterator<Item = std::ops::Range<usize>> + '_ {
        let lo = self.newlines.partition_point(|&it| it < range.start);
        let hi = self.newlines.partition_point(|&it| it <= range.end);
        let all = iter::once(range.start)
            .chain(self.newlines[lo..hi].iter().copied())
            .chain(iter::once(range.end));

        all.clone()
            .zip(all.skip(1))
            .map(|(lo, hi)| lo..hi)
            .filter(|it| !it.is_empty())
    }

    fn utf8_to_utf16_col(&self, line: usize, col: usize) -> usize {
        let mut res: usize = col.into();
        if let Some(utf16_chars) = self.utf16_lines.get(&line) {
            for c in utf16_chars {
                if c.end <= col {
                    res -= usize::from(c.len()) - c.len_utf16();
                } else {
                    // From here on, all utf16 characters come *after* the character we are mapping,
                    // so we don't need to take them into account
                    break;
                }
            }
        }
        res
    }

    fn utf16_to_utf8_col(&self, line: usize, mut col: usize) -> usize {
        if let Some(utf16_chars) = self.utf16_lines.get(&line) {
            for c in utf16_chars {
                if col > usize::from(c.start) {
                    col += usize::from(c.len()) - c.len_utf16() as usize;
                } else {
                    // From here on, all utf16 characters come *after* the character we are mapping,
                    // so we don't need to take them into account
                    break;
                }
            }
        }

        col.into()
    }
}
