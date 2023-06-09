use std::borrow::Cow;
use std::ops::Deref;

use lib_ruby_parser::Loc;

#[derive(Debug)]
pub struct DecodedInput {
    inner: lib_ruby_parser::source::DecodedInput,
}

impl From<lib_ruby_parser::source::DecodedInput> for DecodedInput {
    fn from(inner: lib_ruby_parser::source::DecodedInput) -> DecodedInput {
        DecodedInput { inner }
    }
}

impl Deref for DecodedInput {
    type Target = lib_ruby_parser::source::DecodedInput;

    fn deref(&self) -> &lib_ruby_parser::source::DecodedInput {
        &self.inner
    }
}

impl DecodedInput {
    pub fn line_col_for_pos(&self, pos: usize) -> (usize, usize) {
        self.inner.line_col_for_pos(pos).unwrap()
    }

    pub fn source(&self, loc: Loc) -> Cow<str> {
        String::from_utf8_lossy(&self.inner.bytes[loc.begin..loc.end])
    }

    pub fn intersect(&self, loc: Loc) -> Loc {
        Loc {
            begin: std::cmp::max(loc.begin, 0),
            end: std::cmp::min(loc.end, self.inner.bytes.len()),
        }
    }
}
