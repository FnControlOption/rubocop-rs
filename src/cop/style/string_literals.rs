use crate::cop::*;

#[derive(AutoCorrector)]
pub struct StringLiterals;

const MSG: &str =
    "Prefer single-quoted strings when you don't need string interpolation or special symbols.";

impl Base for StringLiterals {
    fn on_str(&self, ctx: &mut Context, corrector: &mut Corrector, node: &Str) {
        let (Some(begin_l), Some(end_l)) = (node.begin_l, node.end_l) else { return };

        let quote = ctx.source(begin_l);
        if quote == "'" {
            return;
        }

        add_offense!(self, ctx, node.expression_l, MSG, {
            corrector.replace(begin_l, '\'');
            corrector.replace(end_l, '\'');
        });
    }
}
