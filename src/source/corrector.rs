use lib_ruby_parser::Loc;

use crate::source::Rewriter;

pub struct Corrector {
    rewriter: Rewriter,
}

impl Corrector {
    pub fn new(code: &[u8]) -> Corrector {
        Corrector {
            rewriter: Rewriter::new(code),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.rewriter.is_empty()
    }

    pub fn replace<S: Into<String>>(&mut self, loc: Loc, content: S) {
        self.rewriter.replace(loc.begin, loc.end, content.into());
    }

    pub fn wrap<S1, S2>(&mut self, loc: Loc, insert_before: S1, insert_after: S2)
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.rewriter.wrap(
            loc.begin,
            loc.end,
            insert_before.into(),
            insert_after.into(),
        );
    }

    pub fn remove(&mut self, loc: Loc) {
        self.rewriter.remove(loc.begin, loc.end);
    }

    pub fn insert_before<S: Into<String>>(&mut self, loc: Loc, content: S) {
        self.rewriter.insert_before(loc.begin, content.into());
    }

    pub fn insert_after<S: Into<String>>(&mut self, loc: Loc, content: S) {
        self.rewriter.insert_after(loc.end, content.into());
    }

    pub fn remove_preceding(&mut self, loc: Loc, size: usize) {
        self.rewriter.remove(loc.begin - size, loc.begin);
    }

    pub fn remove_leading(&mut self, loc: Loc, size: usize) {
        self.rewriter.remove(loc.begin, loc.begin + size);
    }

    pub fn remove_trailing(&mut self, loc: Loc, size: usize) {
        self.rewriter.remove(loc.end - size, loc.end);
    }

    pub fn process(self, code: &[u8]) -> Vec<u8> {
        self.rewriter.process(code)
    }
}
