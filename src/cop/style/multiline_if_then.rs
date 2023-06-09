use crate::cop::*;

pub struct MultilineIfThen;

const MSG: &str = "Do not use `then` for multi-line `if`.";

impl Base for MultilineIfThen {
    fn on_if(&self, ctx: &mut Context, _corrector: &mut Corrector, node: &If) {
        let Loc { begin, end } = node.expression_l;
        let (begin_line, _) = ctx.line_col_for_pos(begin);
        let (end_line, _) = ctx.line_col_for_pos(end);
        if begin_line == end_line {
            return;
        }

        let then = ctx.source(node.begin_l);
        if then != "then" {
            // "\n" or ";"
            return;
        }

        add_offense!(self, ctx, node.begin_l, MSG);
    }
}
