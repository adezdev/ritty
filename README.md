# Ritty

Ritty is an idiomatic Rust CLI builder and framework inspired by UnJS Citty. It
provides a synchronous, builder-oriented API for nested commands, arguments and
options, execution, lifecycle hooks, plugins, and lazy subcommands. Ritty uses
Rust 2024 and has zero runtime dependencies.

Repository: https://github.com/adezdev/ritty

## Features

- Positional arguments, boolean flags, string options, and enum options
- Required values, defaults, descriptions, value hints, aliases, and short flags
- Nested, aliased, default, hidden, and synchronously loaded lazy subcommands
- Typed handler success values through `CommandOutput`
- Command and plugin setup/cleanup lifecycle hooks
- Generated usage plus CLI-facing `--help`, `-h`, `--version`, and `-v`
- Structured parse and execution errors

## Installation

```toml
[dependencies]
ritty = "0.1.0"
```

## Quick start

```rust
use ritty::prelude::*;

fn main() -> Result<(), RunError> {
    let command = Command::new("greet")
        .version("0.1.0")
        .arg(Arg::new("name").default("world"))
        .flag(Flag::new("excited").short('e'))
        .handler(|ctx| {
            let name = ctx.matches().argument("name").unwrap();
            let punctuation = if ctx.matches().flag("excited") {
                "!"
            } else {
                "."
            };

            println!("Hello, {name}{punctuation}");
            Ok(())
        });

    command.run()
}
```

After building the binary as `greet`:

```text
$ greet Adrian
Hello, Adrian.
$ greet -e Adrian
Hello, Adrian!
$ greet --help
$ greet --version
0.1.0
```

## Arguments and options

Ritty has four input-definition types:

```rust
use ritty::{Arg, Command, EnumOption, Flag, StringOption};

fn main() {
    let command = Command::new("convert")
        .arg(Arg::new("path").required())
        .flag(
            Flag::new("verbose")
                .short('v')
                .alias("chatty"),
        )
        .option(
            StringOption::new("output")
                .alias("o")
                .value_hint("dir"),
        )
        .enum_option(EnumOption::new("format", ["text", "json"]));

    let _ = command;
}
```

`Arg` defines a positional argument. `Flag` defines a boolean option.
`StringOption` accepts an arbitrary string value, and `EnumOption` validates its
value against the declared list. String and enum options accept either
`--name value` or `--name=value`; their one-character aliases also accept `-o
value` and `-o=value`. A missing string or enum value is an error. A separately
supplied value may begin with `-`.

All four definitions support required values and defaults. Descriptions and
value hints feed generated usage. Flags, string options, and enum options
support aliases; flags additionally support a dedicated short name with
`Flag::short` and a description for their negative form. A default satisfies a
required definition.

Boolean flags support positive and negative spellings such as `--force` and
`--no-force`. Explicit values are accepted for positive long and short
spellings:

```text
--force=true
--force=false
-v=true
-v=false
```

Only the exact lowercase values `true` and `false` are accepted. Values such as
`True`, `yes`, `1`, and an empty value produce
`ArgumentErrorKind::InvalidOptionValue`. Negative spellings do not accept an
explicit value.

Handlers and callers of `parse_from` read parsed values through `Matches`:

```rust
use ritty::{Arg, Command, EnumOption, Flag, StringOption};

fn main() {
    let command = Command::new("tool")
        .flag(Flag::new("verbose"))
        .arg(Arg::new("path"))
        .option(StringOption::new("output"))
        .enum_option(EnumOption::new("format", ["text", "json"]))
        .command(Command::new("inspect"))
        .handler(|ctx| {
            let _verbose = ctx.matches().flag("verbose");
            let _path = ctx.matches().argument("path");
            let _output = ctx.matches().option("output");
            let _format = ctx.matches().enum_option("format");
            let _child = ctx.matches().subcommand();
            let _child_matches = ctx.matches().subcommand_matches();
            Ok(())
        });

    let _ = command;
}
```

Aliases canonicalize to the declared name: for example, a value supplied with
`-o` is retrieved with `matches.option("output")`. Subcommand matches form a
nested tree instead of flattening parent and child values together.

## Subcommands

Use `Command::command` to attach eager subcommands. Commands can have aliases,
nest other commands, or be selected automatically when no explicit child was
given:

```rust
use ritty::{Arg, Command};

fn cli() -> Command {
    Command::new("repo")
        .default_subcommand("status")
        .command(
            Command::new("status")
                .alias("st")
                .handler(|_| Ok(())),
        )
        .command(
            Command::new("remote")
                .alias("r")
                .command(
                    Command::new("add")
                        .arg(Arg::new("name").required())
                        .handler(|ctx| {
                            println!("adding {}", ctx.matches().argument("name").unwrap());
                            Ok(())
                        }),
                ),
        )
        .command(Command::new("internal").hidden().handler(|_| Ok(())))
}

fn main() {
    let _ = cli();
}
```

`repo`, `repo status`, `repo st`, `repo remote add origin`, and `repo r add
origin` all select canonical commands in `Matches`. A default subcommand may be
configured by canonical name or alias. Hidden commands remain parseable by
name, alias, or as a default, but are omitted from generated usage/help
listings.

## Handlers and command output

A handler receives a `CommandContext`:

- `ctx.matches()` is the executing command's local `Matches` node.
- `ctx.root_matches()` is the complete top-level `Matches` tree.
- `ctx.command()` is the selected command whose handler is running.

Only the selected leaf handler runs. A handler may return any `'static` success
value. `Command::run_from` preserves that value in a `CommandOutput`:

```rust
use ritty::{Arg, Command, RunError};

#[derive(Debug, PartialEq)]
struct Summary {
    files: usize,
}

fn main() -> Result<(), RunError> {
    let command = Command::new("count")
        .arg(Arg::new("path").required())
        .handler(|ctx| {
            let _path = ctx.matches().argument("path").unwrap();
            Ok(Summary { files: 3 })
        });

    let output = command.run_from(["src"])?;
    let value = output.downcast::<Summary>().unwrap();
    assert_eq!(value, Summary { files: 3 });
    Ok(())
}
```

The handler's success value does not need to implement `Send`, `Sync`, `Clone`,
or `Debug`. `CommandOutput` is a type-erased in-process return value, not a
serialization format. Use `is`, `downcast_ref`, `downcast`, and `type_name` to
inspect it.

`Command::run_from(args)` is the synchronous programmatic argv API. It neither
reads process argv nor treats help/version specially, and it returns the
selected leaf's `CommandOutput`. `Command::run()` reads `std::env::args()` after
the executable name, handles the CLI-facing built-ins, runs ordinary commands,
discards their success value, and returns `Result<(), RunError>`.

## Help and usage

`Command::render_usage()` returns deterministic plain text. `Command::show_usage()`
writes that text to stdout. Usage incorporates declared command metadata,
arguments, options, aliases, required markers, defaults, enum choices, value
hints, subcommands, and hidden filtering.

`Command::run()` recognizes built-in `--help` and `-h` at the applicable command
level. Root `--version` and `-v` are built-ins only when they are the sole argv
token. Successful built-in requests do not run handlers or lifecycle hooks.
User-defined `help`/`version` option spellings take precedence: owning the long
name disables that built-in pair, while owning only a short spelling leaves the
available long form intact.

`run_from` and `parse_from` intentionally do not special-case these spellings;
they are literal programmatic parsing APIs.

## Lifecycle and plugins

Commands support `.setup(...)` and `.cleanup(...)`. A named, reusable `Plugin`
can contribute its own setup and cleanup hooks:

```rust
use ritty::{Command, Plugin};

fn main() {
    let logging = Plugin::new("logging")
        .setup(|ctx| {
            eprintln!("starting {}", ctx.command().name());
            Ok(())
        })
        .cleanup(|ctx| {
            eprintln!("finished {}", ctx.command().name());
            Ok(())
        });

    let command = Command::new("tool")
        .plugin(logging)
        .setup(|_| Ok(()))
        .cleanup(|_| Ok(()))
        .handler(|_| Ok(()));

    let _ = command;
}
```

For one entered command, ordering is:

```text
plugin setup(s), declaration order
→ command setup
→ selected command work
→ command cleanup
→ plugin cleanup(s), reverse declaration order
```

Nested execution composes that rule: setup proceeds root-to-leaf, and cleanup
returns leaf-to-root. Cleanup is attempted after setup, child, or handler
failure. The first established error remains primary; later cleanup failures do
not replace it.

## Lazy subcommands

`Command::lazy_command` defers constructing a child until it is needed:

```rust
use ritty::Command;

fn cli() -> Command {
    Command::new("ritty").lazy_command("build", || {
        Command::new("build")
            .description("Build the project")
            .handler(|_| {
                println!("building");
                Ok(())
            })
    })
}

fn main() {
    let _ = cli();
}
```

The loader is synchronous and runs only when the child is needed. Its result is
cached, and clones of the command tree share that resolution cache. The name
passed to `lazy_command` is the canonical identity even if the returned command
uses another name. Help and usage rendering may resolve lazy children to read
their descriptions, aliases, or hidden state. Ritty has no generalized
`Resolvable<T>` abstraction and no asynchronous loaders.

## Errors

Parsing uses `ArgumentErrorKind`, `ParseErrorKind`, and `ParseError` for
machine-readable classification plus a human-readable message:

```rust
use ritty::{ArgumentErrorKind, Command, ParseErrorKind};

fn main() {
    let error = Command::new("tool")
        .parse_from(["--unknown"])
        .unwrap_err();

    match error.kind() {
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption) => {
            eprintln!("{error}");
        }
        ParseErrorKind::Argument(_) => eprintln!("other argument error: {error}"),
        ParseErrorKind::UnknownCommand => eprintln!("unknown command: {error}"),
        ParseErrorKind::AmbiguousCommand => eprintln!("ambiguous command: {error}"),
        ParseErrorKind::DefaultSubcommandNotFound => {
            eprintln!("invalid default subcommand: {error}");
        }
    }
}
```

`Display` provides the parse message, while `kind()` provides structured
classification. `RunError` preserves parse errors and distinguishes missing
command/version cases, handler failures, command/plugin lifecycle failures, and
stdout I/O failures.

## API organization

The same public API is available through three supported import styles:

```rust
use ritty::{Command, Flag, RunError};

fn accepts(_: Command, _: Flag, _: Option<RunError>) {}
```

```rust
use ritty::argument::Flag;
use ritty::command::Command;
use ritty::error::RunError;

fn accepts(_: Command, _: Flag, _: Option<RunError>) {}
```

```rust
use ritty::prelude::*;

fn accepts(_: Command, _: Flag, _: Option<RunError>) {}
```

The root re-exports are intentionally preserved as the compact facade. Logical
modules are useful for explicit imports, while the prelude gathers the common
builder, execution, match, and error types.

## Citty compatibility

Ritty ports Citty's useful concepts to idiomatic synchronous Rust rather than
translating JavaScript mechanics literally. The most visible deliberate
differences include strict lowercase `true`/`false` boolean values, strict
missing string/enum values, selected-leaf `CommandOutput` propagation, and
synchronous cached lazy subcommands.

## License

Ritty is dual-licensed under MIT OR Apache-2.0. See
[LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
