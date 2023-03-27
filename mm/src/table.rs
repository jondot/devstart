use crate::alias::AliasMap;

use crate::shortcuts::Shortcuts;
use crate::tasks::TableEntry;
use crate::Task;
use owo_colors::OwoColorize;
use tabled::Style;
use tabled::Table;

#[must_use]
pub fn render(tasks: &[Task], aliases: &AliasMap, shortcuts: &Option<Shortcuts>) -> String {
    let mut out = String::new();
    let list = tasks
        .iter()
        .map(|t| TableEntry {
            task: if t.task == "unassigned" {
                "---".to_string()
            } else {
                format!(
                    "{} {}",
                    t.task.clone(),
                    aliases
                        .ntoa(&t.task)
                        .map(|a| format!("({a})"))
                        .unwrap_or_default()
                )
            },
            symbol: t.emoji.clone(),
            exec: t.exec.clone(),
            details: t.details.clone().unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    out.push_str(&Table::new(&list).with(Style::rounded()).to_string());

    if let Some(shortcuts) = shortcuts.as_ref() {
        out.push('\n');
        if let Some(links) = shortcuts.links.as_ref() {
            if !links.is_empty() {
                out.push_str(&"links:\n".underline().to_string());
                for (k, v) in links {
                    out.push_str(&format!("  {} {}\n", k.yellow(), v.title.dimmed()));
                }
            }
        }

        if let Some(folders) = shortcuts.folders.as_ref() {
            if !folders.is_empty() {
                out.push_str(&"folders:\n".underline().to_string());
                for (k, v) in folders {
                    out.push_str(&format!("  {} {}\n", k.yellow(), v.title.dimmed()));
                }
            }
        }
    }
    out
}
