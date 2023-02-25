use crate::alias::AliasMap;
use crate::tasks::TableEntry;
use crate::Task;
use tabled::Style;
use tabled::Table;

#[must_use]
pub fn render(tasks: &[Task], aliases: &AliasMap) -> String {
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
    Table::new(&list).with(Style::rounded()).to_string()
}
