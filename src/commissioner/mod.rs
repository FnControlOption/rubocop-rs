mod visitor;

use std::collections::HashMap;

use lib_ruby_parser::nodes::*;
use lib_ruby_parser::traverse::visitor::*;
use lib_ruby_parser::{Parser, ParserResult};

use crate::ast::Processor;
use crate::cop::{Base, Context, Offense};
use crate::source::{Corrector, DecodedInput};
use crate::Config;

pub struct Commissioner<'cop, 'cfg, 'ast> {
    cop: &'cop dyn Base,
    ctx: Context<'cfg, 'ast>,
    corrector: Corrector,
}

impl Commissioner<'_, '_, '_> {
    pub fn investigate(
        cops: &[&dyn Base],
        config: Config,
        parser: Parser,
    ) -> (DecodedInput, Vec<Offense>, Corrector) {
        let ParserResult { input, ast, .. } = parser.do_parse();
        let input = DecodedInput::from(input);

        let Some(ast) = ast else {
            let corrector = Corrector::new(input.as_shared_bytes());
            return (input, Vec::new(), corrector)
        };

        let mut processor = Processor {
            parents: HashMap::new(),
        };
        processor.process(&ast);
        let Processor { parents } = processor;

        let mut corrector = Corrector::new(input.as_shared_bytes());
        let mut ctx = Context::new(config, input, parents);

        for &cop in cops.iter() {
            if ctx.is_cop_enabled(cop) {
                let mut commissioner = Commissioner {
                    cop,
                    ctx,
                    corrector,
                };
                commissioner.visit(&ast);
                Commissioner { ctx, corrector, .. } = commissioner;
            }
        }

        let (input, offenses) = ctx.into_inner();
        (input, offenses, corrector)
    }
}

macro_rules! trigger_responding_cop {
    ($method:ident, $visit:ident, $comissioner:expr, $node:ident) => {
        let Commissioner {
            cop,
            ctx,
            corrector,
        } = $comissioner;
        if cop.$visit(ctx, corrector, $node) {
            cop.$method(ctx, corrector, $node);
            $visit($comissioner, $node);
        }
    };
}

pub(crate) use trigger_responding_cop;
