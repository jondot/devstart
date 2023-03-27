use crate::Result;
use fs_err as fs;
use fs_err::File;
use lazy_static::lazy_static;
use snafu::ResultExt;
use std::path::Path;

use crate::tasks::ProviderConfig;

use super::{MMFile, ProviderKind, Task, TaskProvider};
use crate::MMFILE;
pub const TEMPLATE: &str = r#"
tasks:
  dev: 
    exec: echo dev
    emoji: "🕹 "

  run: 
    exec: echo run
    emoji: 🟢

  build: 
    exec: echo build
    emoji: 👷‍♀️

  test: 
    exec: echo test
    emoji: 🚦

  clean: 
    exec: echo clean
    emoji: 🧹

  install: 
    exec: echo install
    emoji: 📦
"#;

lazy_static! {
    pub static ref CONFIG: ProviderConfig = serde_yaml::from_str(
        r#"
matchers: ~
defaults: ~
"#
    )
    .unwrap();
}

#[derive(Default)]
pub struct Local {}
impl TaskProvider for Local {
    fn parse(&self, path: &Path) -> Result<Vec<Task>> {
        let file = path.join(MMFILE);
        if !file.exists() {
            return Ok(vec![]);
        }

        let mmfile: MMFile = serde_yaml::from_reader(File::open(&file).context(crate::IOSnafu)?)
            .context(crate::SerializationYamlSnafu)?;
        Ok(mmfile
            .tasks
            .iter()
            .map(|(task, t)| Task {
                provider: ProviderKind::Local,
                task: task.clone(),
                exec: t.exec.clone(),
                emoji: if t.emoji.is_empty() {
                    "🕹 ".to_string()
                } else {
                    t.emoji.clone()
                },
                emoji_text: if t.emoji_text.is_empty() {
                    "[task]".to_string()
                } else {
                    t.emoji_text.clone()
                },
                sh: t.sh,
                details: t.details.clone(),
            })
            .collect::<Vec<_>>())
    }
}

/// Initialize a local task provider config
///
/// # Errors
///
/// This function will return an error if IO fails
pub fn init_local(path: &Path) -> Result<String> {
    fs::write(path.join(MMFILE), TEMPLATE).context(crate::IOSnafu)?;
    Ok(MMFILE.to_string())
}
