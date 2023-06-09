use crate::cop;

use serde_yaml::{Index, Value};

// TODO: support department config

pub struct Config<'a> {
    pub yaml: Option<&'a Value>,
    default: &'static Value,
}

impl<'a> Config<'a> {
    pub fn new(yaml: Option<&'a Value>) -> Self {
        Self {
            yaml,
            default: crate::default::config(),
        }
    }

    pub fn for_cop(&self, cop: &dyn cop::Base) -> Self {
        Self {
            yaml: self.yaml.map(|v| v.get(cop.name())).flatten(),
            default: &self.default[cop.name()],
        }
    }

    pub fn for_all_cops(&self) -> Self {
        Self {
            yaml: self.yaml.map(|v| v.get("AllCops")).flatten(),
            default: &self.default["AllCops"],
        }
    }

    pub fn get<I: Index>(&self, index: I) -> Option<&Value> {
        let value = self.yaml.map(|v| v.get(&index)).flatten();
        value.or_else(|| self.default.get(&index))
    }

    pub fn is_cop_enabled(&self, cop: &dyn cop::Base) -> bool {
        self.for_cop(cop)["Enabled"].as_bool().unwrap_or(false)
    }

    pub fn is_active_support_extensions_enabled(&self) -> bool {
        let value = &self.for_all_cops()["ActiveSupportExtensionsEnabled"];
        value.as_bool().unwrap_or(false)
    }
}

impl Copy for Config<'_> {}

impl Clone for Config<'_> {
    fn clone(&self) -> Self {
        Self {
            yaml: self.yaml,
            default: self.default,
        }
    }
}

impl<I: Index> std::ops::Index<I> for Config<'_> {
    type Output = Value;

    fn index(&self, index: I) -> &Value {
        let value = self.yaml.map(|v| v.get(&index)).flatten();
        value.unwrap_or_else(|| &self.default[&index])
    }
}
