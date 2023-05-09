use regex::Regex;
use snafu::ResultExt;

use crate::Result;
use std::{fs, path::Path};

use super::{ProviderKind, Task, TaskProvider};

#[derive(Default)]
pub struct Makefile {}

impl TaskProvider for Makefile {
    /// Parse for tasks
    ///
    /// # Errors
    ///
    /// This function will return an error if IO fails
    fn parse(&self, path: &Path) -> Result<Vec<Task>> {
        let file = path.join("Makefile");
        if !file.exists() {
            return Ok(vec![]);
        }
        let text = fs::read_to_string(&file).context(crate::IOSnafu)?;
        Ok(parse_rules(&text)
            .iter()
            .map(|rule| Task {
                task: rule.clone(),
                exec: format!("make {rule}"),
                provider: ProviderKind::Make,
                emoji: "🗞".to_string(),
                emoji_text: "[make]".to_string(),
                ..Default::default()
            })
            .collect::<Vec<_>>())
    }
}

fn parse_rules(text: &str) -> Vec<String> {
    let comment = Regex::new(r"^( {4}|\t)*#").unwrap();
    let target = Regex::new(r"^(?P<target>[^\s:#=%]+):").unwrap();
    let mut rules = Vec::new();
    for line in text.lines() {
        if comment.is_match(line) {
            continue;
        }
        if target.is_match(line) {
            if let Some(matches) = target.captures(line) {
                let t = matches["target"].to_string();
                if t == ".PHONY" {
                    continue;
                }
                rules.push(t);
            }
        }
    }
    rules
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn foo_test() {
        let rules = parse_rules(
            r#"
CC=gcc
CFLAGS=-g
CLEAN_FILES=foo.o main
OBJS=foo.o math.o
BIN=main

# This is a phony rule, phony rules are always ran
.PHONY: BIN
BIN: OBJS
    # wildcards get replaced with their value
    CC CFLAGS $^ -o $@

# expand similar rules with placeholders
%.o: %.c
    CC CFLAGS -c $^ -o $@

# run tests
test:
    cargo $@

# clean up all the object and executable files
clean:
    rm OBJS BIN
"#,
        );
        assert_eq!(rules, "BIN test clean".split(' ').collect::<Vec<_>>());
    }
    #[test]
    fn none_test() {
        let rules = parse_rules(
            r#"
CC=gcc
CFLAGS=-g
CLEAN_FILES=foo.o main
OBJS=foo.o math.o
BIN=main

"#,
        );
        assert!(rules.is_empty());
    }

    #[test]
    fn empty_test() {
        let rules = parse_rules(
            r#"
"#,
        );
        assert!(rules.is_empty());
    }
}
