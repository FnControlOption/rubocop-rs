mod base;
pub use base::*;

mod context;
pub use context::*;

pub mod mixin;

mod name;
pub use name::*;

pub mod layout;
pub mod style;

use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::OnceLock;

use lib_ruby_parser::nodes::*;
use lib_ruby_parser::{Bytes, Loc};

use rubocop_macros::*;

use crate::add_offense;
use crate::ast::NodeRef;
use crate::source::Corrector;

pub trait AutoCorrector: Base {}

impl std::fmt::Display for dyn Base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

pub struct Offense {
    pub loc: Loc,
    pub correctable: bool,
    pub cop_name: &'static str,
    pub message: Cow<'static, str>,
}

#[macro_export]
macro_rules! add_offense {
    ($cop:expr, $ctx:expr, $loc:expr, $message:expr) => {
        $ctx.add_offense(Offense {
            loc: $loc,
            correctable: false,
            cop_name: $cop.name(),
            message: Cow::from($message),
        });
    };

    ($cop:expr, $ctx:expr, $loc:expr, $message:expr, $autocorrect:block) => {{
        let _: &dyn AutoCorrector = $cop;
        let offense = Offense {
            loc: $loc,
            correctable: true,
            cop_name: $cop.name(),
            message: Cow::from($message),
        };
        $autocorrect;
        $ctx.add_offense(offense);
    }};
}

// struct Foobar {}
// impl Name for Foobar {
//     fn name(&self) -> &'static str {
//         todo!()
//     }
// }
// impl Base for Foobar {
//     fn on_blockarg(
//         &self,
//         _config: &Config,
//         _input: &ProcessedInput,
//         _offenses: &mut Vec<Offense>,
//         _corrector: &mut Corrector,
//         node: &Blockarg,
//     ) {
//         foobar(node);
//     }
// }
// fn foobar(node: &Blockarg) -> Option<()> {
//     let foo;
//     node_match!(
//         node,
//         Blockarg {
//             name: Some("foo"),
//             name_l: Some(foo @ Loc { .. }),
//             ..
//         }
//     );
//     Some(())
// }
