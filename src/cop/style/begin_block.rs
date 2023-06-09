use crate::cop::*;

pub struct BeginBlock;

const MSG: &str = "Avoid the use of `BEGIN` blocks.";

impl Base for BeginBlock {
    fn on_preexe(&self, ctx: &mut Context, _corrector: &mut Corrector, node: &Preexe) {
        add_offense!(self, ctx, node.keyword_l, MSG);
    }
}
