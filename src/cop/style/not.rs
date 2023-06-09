use crate::cop::mixin::range_help::*;
use crate::cop::*;

#[derive(AutoCorrector)]
pub struct Not;

const MSG: &str = "Use `!` instead of `not`.";

fn opposite_methods() -> &'static HashMap<&'static str, &'static str> {
    static OPPOSITE_METHODS: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    OPPOSITE_METHODS.get_or_init(|| {
        HashMap::from([
            ("any?", "any?"),
            ("empty?", "none?"),
            ("none?", "none?"),
            ("one?", "one?"),
            ("many?", "many?"),
        ])
    })
}

impl Base for Not {
    fn on_send(&self, ctx: &mut Context, corrector: &mut Corrector, node: &Send) {
        let Some((receiver, selector)) = prefix_not(ctx, node) else { return };

        add_offense!(self, ctx, selector, MSG, {
            let range = ctx
                .range_with_surrounding_space(selector)
                .side(Side::Right)
                .build();

            if let Some(receiver) = is_opposite_method(receiver) {
                correct_opposite_method(corrector, range, receiver);
            } else if requires_parens(receiver) {
                correct_with_parens(corrector, range, node);
            } else {
                correct_without_parens(corrector, range);
            }
        });
    }
}

fn is_opposite_method(child: &Node) -> Option<&Send> {
    let Node::Send(s) = child else { return None };
    if opposite_methods().contains_key(s.method_name.as_str()) {
        Some(s)
    } else {
        None
    }
}

fn requires_parens(child: &Node) -> bool {
    match child {
        Node::And(_) | Node::Or(_) => true,
        Node::Send(s) => s.is_binary_operation(),
        Node::If(_) => todo!("If"),
        _ => false,
    }
}

fn correct_opposite_method(corrector: &mut Corrector, range: Loc, child: &Send) {
    corrector.remove(range);
    corrector.replace(
        child.selector_l.unwrap(),
        opposite_methods()[child.method_name.as_str()],
    );
}

fn correct_with_parens(corrector: &mut Corrector, range: Loc, node: &Send) {
    corrector.replace(range, "!(");
    corrector.insert_after(node.expression_l, ')');
}

fn correct_without_parens(corrector: &mut Corrector, range: Loc) {
    corrector.replace(range, '!');
}

// TODO: rubocop-ast

node_matcher!(
    fn negation_method(node: &Send) -> Option<(&Node, &Loc)>,
    Send {
        recv: Some(receiver),
        method_name: "!",
        selector_l: Some(selector),
    }
);

fn prefix_not<'ast>(ctx: &Context, node: &'ast Send) -> Option<(&'ast Node, Loc)> {
    let Some((receiver, selector)) = negation_method(node) else { return None };
    let selector = *selector;
    if ctx.source(selector) == "not" {
        Some((receiver, selector))
    } else {
        None
    }
}

trait MethodDispatchNode {
    fn is_binary_operation(&self) -> bool;
}

impl MethodDispatchNode for Send {
    fn is_binary_operation(&self) -> bool {
        let Some(selector) = self.selector_l else { return false };
        // TODO: is_operator_method
        self.expression_l.begin != selector.begin
    }
}
