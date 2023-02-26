use super::Result;
use std::{ffi::OsString, path::Path};

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
    let cmd = args
        .first()
        .with_whatever_context(|| "command is empty".to_string())?;
    sh_dangerous(cmd).dir(pwd).run().context(crate::IOSnafu)?;
    Ok(())
}

pub fn sh_dangerous<T: Into<OsString>>(command: T) -> duct::Expression {
    let argv = shell_command_argv(command.into());
    duct::cmd(&argv[0], &argv[1..])
}

#[cfg(unix)]
fn shell_command_argv(command: OsString) -> Vec<OsString> {
    use std::env;

    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
    vec![shell.into(), "-c".into(), command]
}

#[cfg(windows)]
fn shell_command_argv(command: OsString) -> Vec<OsString> {
    let comspec = std::env::var_os("COMSPEC").unwrap_or_else(|| "cmd.exe".into());
    vec![comspec, "/C".into(), command]
}
