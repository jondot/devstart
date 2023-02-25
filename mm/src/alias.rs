use lazy_static::lazy_static;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AliasMap {
    aliases: HashMap<String, String>,
}

impl AliasMap {
    #[must_use]
    pub fn ntoa(&self, name: &str) -> Option<String> {
        self.aliases
            .iter()
            .find(|(_, v)| *v == name)
            .map(|(k, _)| k.clone())
    }

    #[must_use]
    pub fn aton<S: AsRef<str>>(&self, alias: S) -> String {
        let alias = alias.as_ref();
        self.aliases
            .get(alias)
            .map_or_else(|| alias, |s| s.as_str())
            .to_string()
    }
}

lazy_static! {
    pub static ref MAP: AliasMap = serde_yaml::from_str(
        r#"
aliases:
    s: start
    b: build
    c: clean
    i: install
    t: test
    l: lint
    f: format
    r: run
"#
    )
    .unwrap();
}
