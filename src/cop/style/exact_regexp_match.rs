use crate::cop::*;

#[derive(AutoCorrector)]
pub struct ExactRegexpMatch;

fn msg(prefer: &str) -> String {
    format!("Use `{prefer}`.")
}

impl Base for ExactRegexpMatch {
    fn on_send(&self, ctx: &mut Context, corrector: &mut Corrector, node: &Send) {
        let Some((receiver, regexp)) = exact_regexp_match(node) else { return };
        let regexp = String::from_utf8_lossy(&regexp.raw);

        let parsed_regexp = {
            let mut parser = regex_syntax::ParserBuilder::new().multi_line(true).build();
            parser.parse(&regexp).unwrap()
        };

        let literal = {
            let Some(literal) = exact_match_pattern(&parsed_regexp) else { return };
            String::from_utf8_lossy(literal)
        };

        let prefer = {
            let receiver = ctx.source(*receiver.expression());
            let new_method = if node.method_name == "!~" { "!=" } else { "==" };
            format!("{receiver} {new_method} '{literal}'")
        };

        add_offense!(self, ctx, node.expression_l, msg(&prefer), {
            corrector.replace(node.expression_l, prefer);
        });
    }
}

node_matcher!(
    /*
        def_node_matcher :exact_regexp_match, <<~PATTERN
          (send
            _ {:=~ :=== :!~ :match :match?}
            (regexp
              (str $_)
              (regopt)))
        PATTERN
    */
    fn exact_regexp_match(node: &Send) -> Option<(&Node, &Bytes)>,
    Send {
        recv: Some(receiver),
        method_name: "=~" | "===" | "!~" | "match" | "match?",
        args: [Node::Regexp(Regexp {
            parts: [Node::Str(Str { value: regexp, .. })],
            options: None,
        })],
    }
);

fn exact_match_pattern(parsed_regexp: &regex_syntax::hir::Hir) -> Option<&[u8]> {
    use regex_syntax::hir::{self, HirKind};
    match parsed_regexp.kind() {
        HirKind::Concat(concat) => match concat.as_slice() {
            [a, b, c] => match (a.kind(), b.kind(), c.kind()) {
                (
                    HirKind::Look(hir::Look::Start),
                    HirKind::Literal(hir::Literal(literal)),
                    HirKind::Look(hir::Look::End),
                ) => Some(literal),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}
