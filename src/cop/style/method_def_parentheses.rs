use crate::cop::*;

#[derive(AutoCorrector)]
pub struct MethodDefParentheses;

const MSG: &str = "Use def with parentheses when there are parameters.";

impl Base for MethodDefParentheses {
    fn on_def(&self, ctx: &mut Context, corrector: &mut Corrector, node: &Def) {
        let Some(Node::Args(args)) = node.args.as_deref() else { return };

        if args.end_l.is_some() {
            // Found a parenthesis
            return;
        }

        add_offense!(self, ctx, args.expression_l, MSG, {
            let begin_l = Loc {
                begin: node.name_l.end,
                end: args.expression_l.begin,
            };
            corrector.replace(begin_l, '(');
            corrector.insert_after(args.expression_l, ')');
        });
    }
}
