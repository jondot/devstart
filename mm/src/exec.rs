use super::Result;
use std::path::Path;

use crate::tasks::Task;
use snafu::{OptionExt, ResultExt};

/// Runs a task
///
/// # Errors
///
/// This function will return an error if task fails
pub fn run(pwd: &Path, task: &Task) -> Result<()> {
    if task.sh {
        sh(pwd, &[task.exec.as_str()])
    } else {
        cmd(pwd, &[task.exec.as_str()])
    }
}

fn cmd(pwd: &Path, args: &[&str]) -> Result<()> {
    let cmd = args
        .first()
        .with_whatever_context(|| "command is empty".to_string())?;

    let args = cmd.split(' ').collect::<Vec<_>>();
    let (first, rest) = args
        .split_first()
        .with_whatever_context(|| "command is empty".to_string())?;
    duct::cmd(*first, rest)
        .dir(pwd)
        .run()
        .context(crate::IOSnafu)?;
    Ok(())
}

fn sh(pwd: &Path, args: &[&str]) -> Result<()> {
    use duct_sh::sh_dangerous;
    let cmd = args
        .first()
        .with_whatever_context(|| "command is empty".to_string())?;
    sh_dangerous(cmd).dir(pwd).run().context(crate::IOSnafu)?;
    Ok(())
}
