use lib_ruby_parser::Loc;

use crate::source::DecodedInput;

pub trait RangeHelp {
    fn range_with_surrounding_comma(&self, range: Loc, side: Side) -> Loc;
    fn range_with_surrounding_space(&self, range: Loc) -> WithSurroundingSpaceBuilder;
    fn range_by_whole_lines(&self, range: Loc) -> ByWholeLinesBuilder;
}

impl RangeHelp for DecodedInput {
    fn range_with_surrounding_comma(&self, range: Loc, side: Side) -> Loc {
        range_with_surrounding_comma(self, range, side)
    }

    fn range_with_surrounding_space(&self, range: Loc) -> WithSurroundingSpaceBuilder {
        WithSurroundingSpaceBuilder::new(self, range)
    }

    fn range_by_whole_lines(&self, range: Loc) -> ByWholeLinesBuilder {
        ByWholeLinesBuilder::new(self, range)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Side {
    Both,
    Left,
    Right,
}

pub struct WithSurroundingSpaceBuilder<'src> {
    src: &'src DecodedInput,
    range: Loc,
    side: Side,
    newlines: bool,
    whitespace: bool,
    continuations: bool,
}

impl<'src> WithSurroundingSpaceBuilder<'src> {
    fn new(src: &'src DecodedInput, range: Loc) -> Self {
        Self {
            src,
            range,
            side: Side::Both,
            newlines: true,
            whitespace: false,
            continuations: false,
        }
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn newlines(mut self, newlines: bool) -> Self {
        self.newlines = newlines;
        self
    }

    pub fn whitespace(mut self, whitespace: bool) -> Self {
        self.whitespace = whitespace;
        self
    }

    pub fn continuations(mut self, continuations: bool) -> Self {
        self.continuations = continuations;
        self
    }

    pub fn build(self) -> Loc {
        let Self {
            src,
            range,
            side,
            newlines,
            whitespace,
            continuations,
        } = self;
        range_with_surrounding_space(src, range, side, newlines, whitespace, continuations)
    }
}

pub struct ByWholeLinesBuilder<'src> {
    src: &'src DecodedInput,
    range: Loc,
    include_final_newline: bool,
}

impl<'src> ByWholeLinesBuilder<'src> {
    fn new(src: &'src DecodedInput, range: Loc) -> Self {
        Self {
            src,
            range,
            include_final_newline: false,
        }
    }

    pub fn include_final_newline(mut self, include_final_newline: bool) -> Self {
        self.include_final_newline = include_final_newline;
        self
    }

    pub fn build(self) -> Loc {
        let Self {
            src,
            range,
            include_final_newline,
        } = self;
        range_by_whole_lines(src, range, include_final_newline)
    }
}

fn range_with_surrounding_comma(src: &DecodedInput, range: Loc, side: Side) -> Loc {
    let (go_left, go_right) = directions(side);

    let Loc { begin, end } = range;
    let begin = move_pos(src, begin, -1, go_left, |b| b == b',');
    let end = move_pos(src, end, 1, go_right, |b| b == b',');

    Loc { begin, end }
}

fn range_with_surrounding_space(
    src: &DecodedInput,
    range: Loc,
    side: Side,
    newlines: bool,
    whitespace: bool,
    continuations: bool,
) -> Loc {
    let (go_left, go_right) = directions(side);

    let Loc { mut begin, mut end } = range;
    if go_left {
        begin = final_pos(src, begin, -1, continuations, newlines, whitespace);
    }
    if go_right {
        end = final_pos(src, end, 1, continuations, newlines, whitespace)
    }
    Loc { begin, end }
}

fn range_by_whole_lines(src: &DecodedInput, range: Loc, include_final_newline: bool) -> Loc {
    let (begin_offset, _) = src.line_col_for_pos(range.begin);
    let (last_column, last_line) = src.line_col_for_pos(range.end);
    let mut end_offset = src.lines[last_line].len() - last_column;
    if include_final_newline {
        end_offset += 1;
    }

    src.intersect(Loc {
        begin: range.begin - begin_offset,
        end: range.end + end_offset,
    })
}

fn directions(side: Side) -> (bool, bool) {
    if side == Side::Both {
        (true, true)
    } else {
        (side == Side::Left, side == Side::Right)
    }
}

fn final_pos(
    src: &DecodedInput,
    pos: usize,
    increment: isize,
    continuations: bool,
    newlines: bool,
    whitespace: bool,
) -> usize {
    let pos = move_pos(src, pos, increment, true, |b| b == b' ' || b == b'\t');
    let pos = move_pos_str(src, pos, increment, continuations, b"\\\n");
    let pos = move_pos(src, pos, increment, newlines, |b| b == b'\n');
    move_pos(src, pos, increment, whitespace, |b| b.is_ascii_whitespace())
}

fn move_pos<F>(src: &DecodedInput, mut pos: usize, step: isize, condition: bool, f: F) -> usize
where
    F: Fn(u8) -> bool,
{
    let offset: isize = if step == -1 { -1 } else { 0 };
    while {
        let i = pos.checked_add_signed(offset).unwrap();
        condition && f(src.bytes[i])
    } {
        pos = pos.saturating_add_signed(step);
    }
    pos
}

fn move_pos_str(
    src: &DecodedInput,
    mut pos: usize,
    step: isize,
    condition: bool,
    needle: &[u8],
) -> usize {
    let size = needle.len();
    let offset: isize = if step == -1 { -1 } else { 0 };
    while {
        let i = pos.checked_add_signed(offset).unwrap();
        condition && &src.bytes[i..i + size] == needle
    } {
        for _ in 0..size {
            pos = pos.saturating_add_signed(step);
        }
    }
    pos
}
