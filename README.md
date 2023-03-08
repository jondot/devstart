<h1 align="center">
   <img src="media/screen.png" width="600"/>
   <br/>
   Make me
</h1>

<h4 align="center"> Make me: make me run dev tasks without thinking</h4>
<p align="center">
<img src="https://github.com/jondot/makeme/actions/workflows/build.yml/badge.svg"/>
</p>
<p align="center">
  <a href="#how-to-use">How To Use</a> •
  <a href="#download">Download</a> •
  <a href="#contributing">Contributing</a> •
  <a href="#license">License</a>
</p>


## How to Use

`mm` will discover tasks from:

* `Cargo` (predefined)
* `node` (package.json)
* `make` (Makefile)
* Feel free to [submit a PR to add](https://github.com/jondot/makeme/pulls) your favorite one

Listing tasks from all package managers (currently showing Cargo) assigned as common tasks (build, test, clean, etc.):

```
$ mm -l
╭────┬───────────┬─────────────────┬─────────╮
│    │ task      │ exec            │ details │
├────┼───────────┼─────────────────┼─────────┤
│ 🦀 │ build (b) │ cargo build     │         │
│ 🦀 │ clean (c) │ cargo clean     │         │
│ 🦀 │ lint (l)  │ cargo clippy    │         │
│ 🟢 │ run (r)   │ cargo run -p mm │         │
│ 🚦 │ test (t)  │ cargo xtask ci  │         │
╰────┴───────────┴─────────────────┴─────────╯
```

You can use `-a` to show _all_ tasks, even those tasks which aren't common:

```
$ mm -l -a
< list of all discovered tasks >
```

Run a task using an _alias_ (`b` for `build`)

```
$ mm b
```

Or with a task name:

```
$ mm build
```

With custom build flags:

```
$ $(mm build -s) --release
```

The `-s` flag inlines the command as a simple string.

Or just `mm` to select a task to run:

```
$ mm
? Select task ›
❯ 🦀  [build] cargo build
  🦀  [clean] cargo clean
  🦀  [lint] cargo clippy
  🟢  [run] cargo run -p mm
  🚦  [test] cargo xtask ci
```

You can override and configure your own local tasks which will _overlay_ on top of the discovered tasks.


```
$ mm --init
wrote .makeme.yaml
```

Edit `.makeme.yaml`:

```yaml
tasks:
  run: 
    exec: cargo run -p mm
    emoji: 🟢

  test: 
    exec: cargo xtask ci
    emoji: 🚦
```

Use `sh: true` to invoke as a shell script. This might come handy as a one-liner. In the example below we're running a Jupyter lab instance, with all the cruft of activating environments taken care of:


```yaml
tasks:
  start: 
    exec: source ~/.zshrc && mamba activate myenv && jupyter-lab
    emoji: 🟢
    sh: true
```

Because there's only a single task, `mm` without any task will run it.

> Note that the default shell is `$SHELL`, and if you want logon/terminal facilities available for scripting you need to source your respective `rc` file (or use a specific solution provided by your shell, e.g. `.zshenv` for `zsh`).
## Download

For macOS:

```
brew tap jondot/tap && brew install mm
```

Otherwise, grab a release from [releases](https://github.com/jondot/makeme/releases) and run `bp --help`:

# Contributing

We are accepting PRs. Feel free to [submit PRs](https://github.com/jondot/makeme/pulls).

To all [Contributors](https://github.com/jondot/makeme/graphs/contributors) - you make this happen, thanks!

# License

Copyright (c) 2023 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
