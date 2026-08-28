use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use ritty::argument::{Arg, EnumOption, Flag, StringOption};
use ritty::command::{BoxError, Command, CommandContext, CommandOutput, HandlerResult, Plugin};
use ritty::error::{ArgumentErrorKind, ParseError, ParseErrorKind, RunError};
use ritty::matches::Matches;

#[test]
fn logical_module_paths_cover_lazy_default_execution_and_lifecycle() {
    let loads = Arc::new(AtomicUsize::new(0));
    let setups = Arc::new(AtomicUsize::new(0));
    let cleanups = Arc::new(AtomicUsize::new(0));

    let command = Command::new("ritty")
        .description("Module API")
        .default_subcommand("serve")
        .lazy_command("serve", {
            let loads = Arc::clone(&loads);
            let setups = Arc::clone(&setups);
            let cleanups = Arc::clone(&cleanups);
            move || {
                loads.fetch_add(1, Ordering::SeqCst);
                Command::new("loader-name")
                    .alias("s")
                    .description("Serve the application")
                    .plugin(
                        Plugin::new("lifecycle")
                            .setup({
                                let setups = Arc::clone(&setups);
                                move |_context: &CommandContext<'_>| -> HandlerResult {
                                    setups.fetch_add(1, Ordering::SeqCst);
                                    Ok(())
                                }
                            })
                            .cleanup({
                                let cleanups = Arc::clone(&cleanups);
                                move |_context: &CommandContext<'_>| -> HandlerResult {
                                    cleanups.fetch_add(1, Ordering::SeqCst);
                                    Ok(())
                                }
                            }),
                    )
                    .handler(|_context| -> HandlerResult<String> { Ok("served".to_owned()) })
            }
        });

    let usage = command.render_usage();
    assert!(usage.contains("Serve the application"));
    assert_eq!(loads.load(Ordering::SeqCst), 1);

    let output: CommandOutput = command.run_from([] as [&str; 0]).unwrap();
    assert_eq!(output.downcast::<String>().unwrap(), "served");
    assert_eq!(loads.load(Ordering::SeqCst), 1);
    assert_eq!(setups.load(Ordering::SeqCst), 1);
    assert_eq!(cleanups.load(Ordering::SeqCst), 1);
    assert_eq!(command.subcommands()[0].name(), "serve");
}

#[test]
fn every_public_module_type_is_downstream_usable() {
    let command = Command::new("ritty")
        .arg(Arg::new("name"))
        .flag(Flag::new("force"))
        .option(StringOption::new("config"))
        .enum_option(EnumOption::new("mode", ["fast"]));
    let matches: Matches = command.parse_from([] as [&str; 0]).unwrap();
    assert_eq!(matches.argument("name"), None);

    let error: ParseError = command.parse_from(["--missing"]).unwrap_err();
    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
    );
    let run_error: RunError = error.into();
    assert!(matches!(run_error, RunError::Parse(_)));

    let boxed: BoxError = std::io::Error::other("module error").into();
    let result: HandlerResult = Err(boxed);
    match result {
        Ok(()) => panic!("expected module error"),
        Err(error) => assert_eq!(error.to_string(), "module error"),
    }
}
