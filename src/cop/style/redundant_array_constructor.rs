use crate::cop::*;

#[derive(AutoCorrector)]
pub struct RedundantArrayConstructor;

const MSG: &str = "Remove the redundant `Array` constructor.";

impl Base for RedundantArrayConstructor {
    fn on_send(&self, ctx: &mut Context, corrector: &mut Corrector, node: &Send) {
        let range;
        let expression;
        let replacement;
        if let Some((receiver, array_literal, &selector)) = redundant_array_new(node) {
            range = receiver.expression_l.with_end(selector.end);
            expression = node.expression_l;
            replacement = array_literal.expression_l;
        } else if let Some((array_literal, &selector)) = redundant_array_constructor(node) {
            range = selector;
            expression = node.expression_l;
            replacement = array_literal.expression_l;
        } else {
            return;
        }
        self.register_offense(ctx, corrector, range, expression, replacement);
    }

    fn on_index(&self, ctx: &mut Context, corrector: &mut Corrector, node: &Index) {
        let Some(receiver) = redundant_array_index(node) else { return };
        let range = receiver.expression_l;
        let expression = node.expression_l;
        let replacement = node.begin_l.with_end(node.end_l.end);
        self.register_offense(ctx, corrector, range, expression, replacement);
    }
}

impl RedundantArrayConstructor {
    fn register_offense(
        &self,
        ctx: &mut Context,
        corrector: &mut Corrector,
        range: Loc,
        expression: Loc,
        replacement: Loc,
    ) {
        add_offense!(self, ctx, range, MSG, {
            corrector.replace(expression, ctx.source(replacement));
        });
    }
}

node_matcher!(
    /*
            (send
              (const {nil? cbase} :Array) :new
              $(array ...))
    */
    fn redundant_array_new(node: &Send) -> Option<(&Const, &Array, &Loc)>,
    Send {
        recv: Some(Node::Const(receiver @ Const {
            scope: None | Some(Node::Cbase(_)),
            name: "Array",
        })),
        method_name: "new",
        args: [Node::Array(array_literal)],
        selector_l: Some(selector),
    }
);

node_matcher!(
    /*
            (send
              (const {nil? cbase} :Array) :[]
              $...)
    */
    fn redundant_array_index(node: &Index) -> Option<&Const>,
    Index {
        recv: Node::Const(receiver @ Const {
            scope: None | Some(Node::Cbase(_)),
            name: "Array",
        }),
        indexes: _,
    }
);

node_matcher!(
    /*
            (send
              nil? :Array
              $(array ...))
    */
    fn redundant_array_constructor(node: &Send) -> Option<(&Array, &Loc)>,
    Send {
        recv: None,
        method_name: "Array",
        args: [Node::Array(array_literal)],
        selector_l: Some(selector),
    }
);
