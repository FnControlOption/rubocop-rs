use crate::cop::*;

pub struct EndAlignment;

fn msg(end_line: usize, end_col: usize, align_line: usize, align_col: usize) -> String {
    let source = "if";
    let end_line = end_line + 1;
    let align_line = align_line + 1;
    format!("`end` at {end_line}, {end_col} is not aligned with `{source}` at {align_line}, {align_col}.")
}

impl Base for EndAlignment {
    fn on_if(&self, ctx: &mut Context, _corrector: &mut Corrector, node: &If) {
        // None for `elsif`
        let Some(end_l) = node.end_l else { return };

        let (if_line, if_col) = ctx.line_col_for_pos(node.keyword_l.begin);
        let (end_line, end_col) = ctx.line_col_for_pos(end_l.begin);
        if if_line == end_line || if_col == end_col {
            return;
        }

        add_offense!(self, ctx, end_l, msg(end_line, end_col, if_line, if_col));
    }
}
