use crate::cop::*;

pub struct ElseAlignment;

const MSG: &str = "Align `else` with `if`.";

impl Base for ElseAlignment {
    fn on_if(&self, ctx: &mut Context, _corrector: &mut Corrector, node: &If) {
        let Some(else_l) = node.else_l else { return };

        let (if_line, if_col) = ctx.line_col_for_pos(node.keyword_l.begin);
        let (else_line, else_col) = ctx.line_col_for_pos(else_l.begin);
        if if_line == else_line || if_col == else_col {
            return;
        }

        add_offense!(self, ctx, else_l, MSG);
    }
}
