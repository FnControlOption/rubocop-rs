use std::borrow::Cow;
use std::collections::HashMap;

use lib_ruby_parser::Loc;

use crate::ast::NodeRef;
use crate::cop::mixin::range_help::*;
use crate::cop::{self, Offense};
use crate::source::DecodedInput;
use crate::Config;

pub struct Context<'cfg, 'ast> {
    config: Config<'cfg>,
    input: DecodedInput,
    parents: HashMap<NodeRef<'ast>, NodeRef<'ast>>,
    offenses: Vec<Offense>,
}

impl<'cfg, 'ast> Context<'cfg, 'ast> {
    pub fn new(
        config: Config<'cfg>,
        input: DecodedInput,
        parents: HashMap<NodeRef<'ast>, NodeRef<'ast>>,
    ) -> Self {
        Self {
            config,
            input,
            parents,
            offenses: Vec::new(),
        }
    }

    pub fn into_inner(self) -> (DecodedInput, Vec<Offense>) {
        (self.input, self.offenses)
    }

    pub fn is_cop_enabled(&self, cop: &dyn cop::Base) -> bool {
        self.config.is_cop_enabled(cop)
    }

    pub fn is_active_support_extensions_enabled(&self) -> bool {
        self.config.is_active_support_extensions_enabled()
    }

    pub fn add_offense(&mut self, offense: Offense) {
        self.offenses.push(offense)
    }

    pub fn parent<N>(&self, node: N) -> Option<&NodeRef<'ast>>
    where
        N: Into<NodeRef<'ast>>,
    {
        self.parents.get(&node.into())
    }

    pub fn line_col_for_pos(&self, pos: usize) -> (usize, usize) {
        self.input.line_col_for_pos(pos)
    }

    pub fn source(&self, loc: Loc) -> Cow<str> {
        self.input.source(loc)
    }

    pub fn intersect(&self, loc: Loc) -> Loc {
        self.input.intersect(loc)
    }
}

impl RangeHelp for Context<'_, '_> {
    fn range_with_surrounding_comma(&self, range: Loc, side: Side) -> Loc {
        self.input.range_with_surrounding_comma(range, side)
    }

    fn range_with_surrounding_space(&self, range: Loc) -> WithSurroundingSpaceBuilder {
        self.input.range_with_surrounding_space(range)
    }

    fn range_by_whole_lines(&self, range: Loc) -> ByWholeLinesBuilder {
        self.input.range_by_whole_lines(range)
    }
}
