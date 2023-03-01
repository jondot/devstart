use crate::Result;
use fs_err as fs;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use snafu::ResultExt;
use std::{collections::HashMap, path::Path};

use super::{Matcher, ProviderConfig, ProviderKind, Task, TaskProvider, CMD_NONE};

#[derive(Deserialize, Serialize, Debug)]
pub struct PackageScripts {
    scripts: HashMap<String, String>,
}

lazy_static! {
    pub static ref CONFIG: ProviderConfig = serde_yaml::from_str(
        r#"
matchers:
- task: start
  expr: ^(dev|start|start:all)$
- task: build
  expr: ^build$
- task: test
  expr: ^(test|jest)$
defaults:
- task: install
  exec: install
  provider: npm
  emoji: " "
  emoji_text: "[npm]"
"#
    )
    .unwrap();
}

/// detect and return prefix for (script runner, runner)
fn detect_prefix(path: &Path) -> (String, String) {
    if ["pnpm-lock.yaml", "pnpm-workspace"]
        .iter()
        .any(|f| path.join(f).exists())
    {
        ("pnpm run".to_string(), "pnpm".to_string())
    } else if path.join("yarn.lock").exists() {
        ("yarn".to_string(), "yarn".to_string())
    } else {
        ("npm".to_string(), "npm".to_string())
    }
}

fn detect(
    scripts: &HashMap<String, String>,
    script_runner: &str,
    runner: &str,
    matchers: &Option<Vec<Matcher>>,
    cfg_defaults: &[Task],
) -> Vec<Task> {
    let mut from_package = matchers
        .as_ref()
        .map_or_else(std::vec::Vec::new, |matchers| {
            scripts
                .iter()
                .map(|(script_name, script_value)| {
                    matchers
                        .iter()
                        .find(|rule| rule.expr.is_match(script_name))
                        .map_or_else(
                            || Task {
                                provider: ProviderKind::Npm,
                                task: CMD_NONE.to_string(), // unassigned
                                exec: format!("{script_runner} {script_name}"),
                                emoji: " ".to_string(),
                                emoji_text: "[npm]".to_string(),
                                ..Default::default()
                            },
                            |selected| Task {
                                provider: ProviderKind::Npm,
                                task: selected.task.clone(),
                                exec: format!("{script_runner} {script_name}"),
                                details: Some(script_value.to_string()),
                                emoji: " ".to_string(),
                                emoji_text: "[npm]".to_string(),
                                ..Default::default()
                            },
                        )
                })
                .collect::<Vec<_>>()
        });

    let from_defaults = cfg_defaults
        .iter()
        .filter(|def| !from_package.iter().any(|e| def.task == e.task))
        .map(|def| Task {
            exec: format!("{runner} {}", def.exec),
            ..def.clone()
        })
        .collect::<Vec<_>>();

    from_package.extend(from_defaults);
    from_package
}

#[derive(Default)]
pub struct Npm {}

impl TaskProvider for Npm {
    /// Parse for tasks
    ///
    /// # Errors
    ///
    /// This function will return an error if IO fails
    fn parse(&self, path: &Path) -> Result<Vec<Task>> {
        let file = path.join("package.json");
        if !file.exists() {
            return Ok(vec![]);
        }

        let scripts: PackageScripts =
            serde_json::from_reader(fs::File::open(&file).context(crate::IOSnafu)?)
                .context(crate::SerializationJsonSnafu)?;
        let (script_runner, runner) = detect_prefix(path);
        Ok(detect(
            &scripts.scripts,
            &script_runner,
            &runner,
            &CONFIG.matchers,
            &CONFIG.defaults,
        ))
    }
}
