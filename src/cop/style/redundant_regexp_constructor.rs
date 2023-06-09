use crate::cop::*;

#[derive(AutoCorrector)]
pub struct RedundantRegexpConstructor;

fn msg(method: &str) -> String {
    format!("Remove the redundant `Regexp.{method}`.")
}

impl Base for RedundantRegexpConstructor {
    fn on_send(&self, ctx: &mut Context, corrector: &mut Corrector, node: &Send) {
        let Some(regexp) = redundant_regexp_constructor(node) else { return };

        add_offense!(self, ctx, node.expression_l, msg(&node.method_name), {
            corrector.replace(node.expression_l, ctx.source(regexp.expression_l));
        });
    }
}

node_matcher!(
    /*
        def_node_matcher :redundant_regexp_constructor, <<~PATTERN
          (send
            (const {nil? cbase} :Regexp) {:new :compile}
            (regexp $... (regopt $...)))
        PATTERN
    */
    fn redundant_regexp_constructor(node: &Send) -> Option<&Regexp>,
    Send {
        recv: Some(Node::Const(Const {
            scope: None | Some(Node::Cbase(_)),
            name: "Regexp",
        })),
        method_name: "new" | "compile",
        args: [Node::Regexp(regexp)],
    }
);
