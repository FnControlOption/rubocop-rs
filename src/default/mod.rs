use crate::cop;
use std::sync::OnceLock;

mod cops;

pub fn config() -> &'static serde_yaml::Value {
    static CONFIG: OnceLock<serde_yaml::Value> = OnceLock::new();
    CONFIG.get_or_init(|| {
        let bytes = include_bytes!("../../config/default.yml");
        serde_yaml::from_slice(bytes).unwrap()
    })
}

pub fn cops() -> &'static [&'static dyn cop::Base] {
    cops::COPS
}
