mod argh_ext;
use argh::FromArgs;

use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use eyre::eyre;
use mm::alias;
use mm::exec;
use mm::prompt;
use mm::table;
use mm::tasks;
use owo_colors::OwoColorize;
use std::path::Path;
use std::process::exit;

/// Makeme: make me run dev tasks without thinking
#[derive(Debug, FromArgs)]
#[allow(clippy::struct_excessive_bools)]
struct AppArgs {
    /// task to run
    #[argh(positional)]
    task: Option<String>,

    /// list tasks
    #[argh(switch, short = 'l')]
    list: bool,

    /// show all
    #[argh(switch, short = 'a')]
    all: bool,

    /// don't execute, copy to clipboard
    #[argh(switch, short = 'c')]
    copy: bool,

    /// root path (default ".")
    #[argh(option, short = 'p')]
    path: Option<String>,

    /// init local config
    #[argh(switch, short = 'i')]
    init: bool,
}

fn main() -> eyre::Result<()> {
    let args: AppArgs = argh_ext::from_env();

    let path_s = args.path.unwrap_or_else(|| ".".to_string());

    let path = Path::new(&path_s);

    if args.init {
        let f = mm::tasks::local::init_local(path)?;
        println!("wrote {f}");

        exit(0);
    }

    let aliases = &alias::MAP;
    let task = args.task.as_ref().map(|a| aliases.aton(a));
    let resolved = tasks::discover(task.as_ref(), path, args.all)?;

    if args.list {
        println!("{}", table::render(&resolved[..], aliases));
    } else {
        let task = if resolved.len() > 1 {
            prompt::choose_task(&resolved[..])
        } else {
            resolved.first()
        };
        if let Some(cmd) = task {
            if args.copy {
                let mut ctx = ClipboardContext::new().map_err(|_| eyre!("cannot get clipboard"))?;
                ctx.set_contents(cmd.exec.clone())
                    .map_err(|_| eyre!("cannot set clipboard content"))?;
                println!("copied `{}`", cmd.exec.green());
            } else {
                exec::run(path, cmd)?;
            }
        }
    }
    Ok(())
}
