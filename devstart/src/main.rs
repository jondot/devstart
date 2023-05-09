mod argh_ext;
use argh::FromArgs;

use devstart::alias;
use devstart::clipboard;
use devstart::exec;
use devstart::prompt;
use devstart::shortcuts::ShortcutsConfig;
use devstart::table;
use devstart::tasks;
use devstart::DSFILE;
use std::path::Path;
use std::process::exit;

/// Devstart: run dev tasks without thinking
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

    /// print the command as string for customising e.g. `$ $(mm b) --release`
    #[argh(switch, short = 's')]
    string: bool,

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
        let f = devstart::tasks::local::init_local(path)?;
        println!("wrote {f}");

        exit(0);
    }

    let aliases = &alias::MAP;
    let input = args.task.as_ref().map(|a| aliases.aton(a));

    // discover tasks
    let resolved = tasks::discover(input.as_ref(), path, args.all)?;

    // discover shortcuts
    let shortcuts = ShortcutsConfig::from_path(path.join(DSFILE).as_path())?;

    if args.list {
        println!(
            "{}",
            table::render(&resolved[..], aliases, &shortcuts.shortcuts)
        );
    } else {
        let task = if resolved.len() > 1 {
            prompt::choose_task(&resolved[..])
        } else {
            resolved.first()
        };

        if let Some(cmd) = task {
            if args.copy {
                clipboard::copy(&cmd.exec)?;
            } else if args.string {
                print!("{}", cmd.exec.clone());
            } else {
                exec::run(path, cmd)?;
            }
        } else if let Some(link) = shortcuts.find_link(input.as_ref()) {
            if args.copy {
                clipboard::copy(&link.url)?;
            } else if args.string {
                print!("{}", link.url);
            } else {
                println!("opening {}...", link.url);
                open::that(link.url)?;
            }
        } else if let Some(folder) = shortcuts.find_folder(input.as_ref()) {
            if args.copy {
                clipboard::copy(&folder.path)?;
            } else if args.string {
                print!("{}", folder.path);
            } else {
                print!("cd {}", folder.path);
            }
        } else if let Some(input) = input.as_ref() {
            // only if they gave actual input and for that input we didn't find anything
            println!("error: `{input}` has no associated action");
        }
    }
    Ok(())
}
