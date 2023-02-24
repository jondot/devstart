use owo_colors::OwoColorize;
use requestty::{OnEsc, Question};

use crate::tasks::Task;

/// Select a task from a collection
///
/// # Errors
///
/// This function will return an error when input fails
#[must_use]
pub fn choose_task(tasks: &[Task]) -> Option<&Task> {
    let selections = tasks
        .iter()
        .map(|t| {
            format!(
                "{}  {} {}",
                t.emoji,
                if t.task == "unassigned" {
                    String::new()
                } else {
                    format!("[{}]", t.task).yellow().to_string()
                },
                t.exec,
            )
        })
        .collect::<Vec<_>>();

    let ans = requestty::prompt_one(
        Question::select("select")
            .message("Select task")
            .choices(selections)
            .on_esc(OnEsc::SkipQuestion)
            .build(),
    );
    let idx = ans
        .ok()
        .and_then(|a| a.as_list_item().map(|item| item.index));
    idx.and_then(|idx| tasks.get(idx))
}
