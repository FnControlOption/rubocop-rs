use crate::cop::*;

pub struct DefEndAlignment;

fn msg(end_line: usize, end_col: usize, align_line: usize, align_col: usize) -> String {
    let source = "def";
    let end_line = end_line + 1;
    let align_line = align_line + 1;
    format!("`end` at {end_line}, {end_col} is not aligned with `{source}` at {align_line}, {align_col}.")
}

impl Base for DefEndAlignment {
    fn on_def(&self, ctx: &mut Context, _corrector: &mut Corrector, node: &Def) {
        let (def_line, def_col) = ctx.line_col_for_pos(node.keyword_l.begin);
        let Some(end_l) = node.end_l else { return };

        let (end_line, end_col) = ctx.line_col_for_pos(end_l.begin);
        if def_line == end_line || def_col == end_col {
            return;
        }

        add_offense!(self, ctx, end_l, msg(end_line, end_col, def_line, def_col));
    }
}
