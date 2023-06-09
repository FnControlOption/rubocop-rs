use crate::cop::*;

#[derive(AutoCorrector)]
pub struct RedundantFilterChain;

fn msg(prefer: &str, first_method: &str, second_method: &str) -> String {
    format!("Use `{prefer}` instead of `{first_method}.{second_method}`.")
}

const RAILS_METHODS: &[&str] = &["many?"];

fn replacement_methods() -> &'static HashMap<&'static str, &'static str> {
    static REPLACEMENT_METHODS: OnceLock<HashMap<&str, &str>> = OnceLock::new();
    REPLACEMENT_METHODS.get_or_init(|| {
        HashMap::from([
            ("any?", "any?"),
            ("empty?", "none?"),
            ("none?", "none?"),
            ("one?", "one?"),
            ("many?", "many?"),
        ])
    })
}

impl Base for RedundantFilterChain {
    fn on_send(&self, ctx: &mut Context, corrector: &mut Corrector, node: &Send) {
        if !node.args.is_empty() || matches!(ctx.parent(node), Some(NodeRef::Block(_))) {
            return;
        }

        let Some((receiver, select_node, &select_selector, filter_method, &predicate_selector)) = select_predicate(node) else { return };

        if RAILS_METHODS.contains(&filter_method) && !ctx.is_active_support_extensions_enabled() {
            return;
        }

        let range = select_selector.join(&predicate_selector);

        let first_method: &str = &select_node.method_name;
        let second_method: &str = &node.method_name;
        let replacement = replacement_methods()[second_method];
        let message = msg(replacement, first_method, second_method);

        add_offense!(self, ctx, range, message, {
            corrector.remove(Loc {
                begin: receiver.expression().end,
                end: predicate_selector.end,
            });
            corrector.replace(select_selector, replacement);
        });
    }
}

node_matcher!(
    /*
        def_node_matcher :select_predicate?, <<~PATTERN
          (send
            {
              (block $(send _ {:select :filter :find_all}) ...)
              $(send _ {:select :filter :find_all} block_pass_type?)
            }
            ${:#{RESTRICT_ON_SEND.join(' :')}})
        PATTERN
    */
    fn select_predicate(node: &Send) -> Option<(&Node, &Send, &Loc, &str, &Loc)>,
    Send {
        recv: Some(receiver @ Node::Block(Block {
            call: Node::Send(select_node @ Send {
                recv: _,
                method_name: "select" | "filter" | "find_all",
                selector_l: Some(select_selector),
                args: [..],
            }),
            args: _,
            body: _,
        }) | receiver @ Node::Send(select_node @ Send {
            recv: _,
            method_name: "select" | "filter" | "find_all",
            selector_l: Some(select_selector),
            args: [Node::BlockPass(_)],
        })),
        method_name: filter_method @ ("any?" | "empty?" | "none?" | "one?" | "many?"),
        selector_l: Some(predicate_selector),
        args: [],
    }
);
