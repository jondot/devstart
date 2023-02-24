pub mod cargo;
pub mod local;
pub mod makefile;
pub mod npm;
use crate::Result;
use std::{cmp::Ordering, collections::HashMap, path::Path};

use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;

use self::local::Local;

const DSFILE: &str = ".devstart.yaml";
const CMD_NONE: &str = "unassigned";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MMFile {
    pub tasks: HashMap<String, Task>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProviderConfig {
    pub matchers: Option<Matchers>,
    pub defaults: Vec<Task>,
}

pub trait TaskProvider {
    /// Parse tasks
    ///
    /// # Errors
    ///
    /// This function will return an error
    fn parse(&self, path: &Path) -> Result<Vec<Task>>;
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub enum ProviderKind {
    #[default]
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "npm")]
    Npm,
    #[serde(rename = "make")]
    Make,
    #[serde(rename = "cargo")]
    Cargo,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Task {
    #[serde(default)]
    pub provider: ProviderKind,
    #[serde(default)]
    pub task: String,

    pub exec: String,
    #[serde(default)]
    pub emoji: String,
    #[serde(default)]
    pub emoji_text: String,

    #[serde(default)]
    pub sh: bool,

    #[serde(default)]
    pub details: Option<String>,
}

#[derive(Tabled)]
pub struct TableEntry {
    #[tabled(rename = "")]
    pub symbol: String,
    pub task: String,
    pub exec: String,
    pub details: String,
}

type Matchers = Vec<Matcher>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Matcher {
    task: String,
    #[serde(with = "serde_regex")]
    expr: Regex,
}

/// Parse tasks
///
/// # Errors
///
/// This function will return an error if IO fails
pub fn parse(path: &Path) -> Result<(Vec<Task>, Vec<Vec<Task>>)> {
    let (local_provider, other_providers): (Local, Vec<Box<dyn TaskProvider>>) = (
        Local::default(),
        vec![
            Box::<self::cargo::Cargo>::default(),
            Box::<self::npm::Npm>::default(),
            Box::<self::makefile::Makefile>::default(),
        ],
    );
    let local = local_provider.parse(path)?;
    let rest = other_providers
        .iter()
        .map(|provider| provider.parse(path))
        .collect::<Result<Vec<_>>>()?;
    Ok((local, rest))
}

#[must_use]
pub fn resolve<'a>(
    task: Option<&'a String>,
    include_unassigned: bool,
    local: &'a [Task],
    discovered: &'a [Vec<Task>],
) -> Vec<&'a Task> {
    let mut all = Vec::new();
    // merge local and discovered:
    // - local tasks are singular, there cannot be a simliar task with the same name
    // - e.g. if discovered = [ cargo:[build, dev], npm:[build] ] and local = [build]
    // result is just the 'build' task from local.
    //
    // so: (1) include all local tasks (2) remove any task from
    // discovered that's identical to a local task.
    // if local tasks are empty, this will naturally list all discovered tasks,
    // *allowing* duplicate entries for the same task name.
    //
    // finally, we sort by who's assigned a task name first, and unassigned last.

    all.extend(local);
    for group in discovered.iter() {
        all.extend(
            group
                .iter()
                // add only tasks that are NOT included in local
                .filter(|t| !local.iter().any(|loc| loc.task.eq(&t.task)))
                .filter(|t| t.task != CMD_NONE || include_unassigned),
        );
    }

    all.sort_by(|a, b| {
        if a.task.is_empty() && !b.task.is_empty() {
            Ordering::Greater
        } else if !a.task.is_empty() && b.task.is_empty() {
            Ordering::Less
        } else {
            a.task.cmp(&b.task)
        }
    });

    if let Some(task) = task {
        // slice per a given task
        all.into_iter()
            .filter(|c| c.task.eq(task))
            .collect::<Vec<_>>()
    } else {
        all
    }
}

/// Discover tasks
///
/// # Errors
///
/// This function will return an error if IO fails
pub fn discover(input: Option<&'_ String>, path: &Path, all: bool) -> Result<Vec<Task>> {
    let (local, discovered) = parse(path)?;

    let resolved = resolve(input, all, &local, &discovered[..]);
    Ok(resolved.into_iter().cloned().collect())
}
