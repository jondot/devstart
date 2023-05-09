use crate::{IOSnafu, SerializationYamlSnafu};

use super::Result;
use fs_err as fs;
use snafu::ResultExt;
use std::{
    collections::{BTreeMap, HashMap},
    env,
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct ShortcutsConfig {
    pub shortcuts: Option<Shortcuts>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Shortcuts {
    pub links: Option<BTreeMap<String, Link>>,
    pub folders: Option<BTreeMap<String, Folder>>,
    pub commands: Option<BTreeMap<String, Command>>,
}
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    pub url: String,
    pub title: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Folder {
    pub path: String,
    pub title: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Command {
    pub title: String,
    pub exec: String,
    pub platforms: Option<HashMap<String, String>>,
}
impl Command {
    #[must_use]
    pub fn get_exec(&self, path: Option<&str>) -> String {
        let exec = self
            .platforms
            .as_ref()
            .and_then(|p| p.get(env::consts::OS).cloned())
            .unwrap_or_else(|| self.exec.to_string());

        path.map(|p| exec.replace("{{path}}", p)).unwrap_or(exec)
    }

    pub fn exec(&self, path: Option<&str>) -> Result<()> {
        crate::exec::cmd(Path::new("."), &[&self.get_exec(path)])
    }
}

impl ShortcutsConfig {
    /// Read shortcuts from yaml. If no shortcuts, returns a default empty holder.
    ///
    /// # Errors
    ///
    /// This function will return an error if parsing fails
    pub fn from_path(path: &Path) -> Result<Self> {
        if path.exists() {
            Self::from_text(&fs::read_to_string(path).context(IOSnafu)?)
        } else {
            Ok(Self::default())
        }
    }
    /// Read shortcuts from yaml
    ///
    /// # Errors
    ///
    /// This function will return an error if parsing fails
    pub fn from_text(text: &str) -> Result<Self> {
        serde_yaml::from_str(text).context(SerializationYamlSnafu)
    }

    #[must_use]
    pub fn find_folder(&self, input: Option<&String>) -> Option<Folder> {
        input.and_then(|input| {
            self.shortcuts
                .as_ref()
                .and_then(|s| s.folders.as_ref())
                .and_then(|f| f.get(input).cloned())
        })
    }

    #[must_use]
    pub fn find_link(&self, input: Option<&String>) -> Option<Link> {
        input.and_then(|input| {
            self.shortcuts
                .as_ref()
                .and_then(|s| s.links.as_ref())
                .and_then(|f| f.get(input).cloned())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ShortcutsConfig;
    use pretty_assertions::assert_eq;

    fn get_exec(c: &ShortcutsConfig, command: &str, path: Option<&str>) -> String {
        c.shortcuts
            .as_ref()
            .and_then(|s| s.commands.as_ref())
            .and_then(|cs| cs.get(command).map(|cmd| cmd.get_exec(path)))
            .unwrap()
    }

    #[test]
    fn command_resolution() {
        let c = ShortcutsConfig::from_text(
            r#"
        
shortcuts: 
  commands:
    c/test-me:
      title: test command
      exec: "echo fallback {{path}}"
      platforms:
        macos: "echo macos {{path}}"
        linux: "echo linux {{path}}"
    c/test-no-path:
      title: test command
      exec: "echo fallback"
      platforms:
        macos: "echo macos"
        linux: "echo linux"


  links:
    l/actions: 
      url: https://github.com/jondot/makeme/actions
      title: Github Actions
    l/repo: 
      url: https://github.com/jondot/makeme/
      title: Repo
    l/issues: 
      url: https://github.com/jondot/makeme/issues
      title: Issues

  folders:
    f/dist: 
      path: dist
      title: Dist folder

        "#,
        )
        .unwrap();

        let exec = get_exec(&c, "c/test-me", Some("foo/bar"));
        #[cfg(target_os = "macos")]
        assert_eq!(exec, "echo macos foo/bar");

        #[cfg(target_os = "linux")]
        assert_eq!(exec, "echo linux foo/bar");

        let exec = get_exec(&c, "c/test-me", None);
        #[cfg(target_os = "macos")]
        assert_eq!(exec, "echo macos {{path}}");

        #[cfg(target_os = "linux")]
        assert_eq!(exec, "echo linux {{path}}");

        let exec = get_exec(&c, "c/test-no-path", Some("foo/bar"));
        #[cfg(target_os = "macos")]
        assert_eq!(exec, "echo macos");

        #[cfg(target_os = "linux")]
        assert_eq!(exec, "echo linux")
    }
}
