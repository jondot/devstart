use crate::Result;
use lazy_static::lazy_static;
use std::path::Path;

use crate::tasks::ProviderConfig;

use super::{Task, TaskProvider};

lazy_static! {
    pub static ref CONFIG: ProviderConfig = serde_yaml::from_str(
        r#"
matchers: ~
defaults:
- task: build
  exec: cargo build
  provider: cargo
  emoji: 🦀
  emoji_text: "[rust]"
- task: run
  exec: cargo run
  provider: cargo
  emoji: 🦀
  emoji_text: "[rust]"
- task: test
  exec: cargo test
  provider: cargo
  emoji: 🦀
  emoji_text: "[rust]"
- task: clean
  exec: cargo clean
  provider: cargo
  emoji: 🦀
  emoji_text: "[rust]"
- task: lint
  exec: cargo clippy
  provider: cargo
  emoji: 🦀
  emoji_text: "[rust]"
"#
    )
    .unwrap();
}

#[derive(Default)]
pub struct Cargo {}

impl TaskProvider for Cargo {
    fn parse(&self, path: &Path) -> Result<Vec<Task>> {
        let file = path.join("Cargo.toml");
        if !file.exists() {
            return Ok(vec![]);
        }
        Ok(CONFIG.defaults.clone())
    }
}
