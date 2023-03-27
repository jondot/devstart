use eyre::{Context, Result};
use fs_err as fs;
use std::{collections::BTreeMap, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct ShortcutsConfig {
    pub shortcuts: Option<Shortcuts>,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Shortcuts {
    pub links: Option<BTreeMap<String, Link>>,
    pub folders: Option<BTreeMap<String, Folder>>,
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

impl ShortcutsConfig {
    /// Read shortcuts from yaml. If no shortcuts, returns a default empty holder.
    ///
    /// # Errors
    ///
    /// This function will return an error if parsing fails
    pub fn from_path(path: &Path) -> Result<Self> {
        if path.exists() {
            Self::from_text(&fs::read_to_string(path)?)
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
        serde_yaml::from_str(text).context("cannot read shortcuts")
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
