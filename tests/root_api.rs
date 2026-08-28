use ritty::{
    Arg, ArgumentErrorKind, BoxError, Command, CommandContext, CommandOutput, EnumOption, Flag,
    HandlerResult, Matches, ParseError, ParseErrorKind, Plugin, RunError, StringOption,
};

fn accepts_context(_context: &CommandContext<'_>) {}

fn assert_public_types(
    matches: &Matches,
    output: CommandOutput,
    result: HandlerResult,
    error: ParseError,
    run_error: RunError,
    boxed: BoxError,
) {
    assert!(matches.flag("verbose"));
    assert!(output.is::<u8>());
    assert!(result.is_ok());
    assert_eq!(
        error.kind(),
        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
    );
    assert!(matches!(run_error, RunError::Parse(_)));
    assert_eq!(boxed.to_string(), "public error");
}

#[test]
fn established_root_facade_compiles_and_parses_representative_schema() {
    let plugin = Plugin::new("public-plugin");
    let command = Command::new("ritty")
        .arg(Arg::new("workspace").required())
        .flag(Flag::new("verbose").short('v'))
        .option(StringOption::new("config").alias("c"))
        .enum_option(EnumOption::new("level", ["info", "debug"]))
        .plugin(plugin)
        .command(
            Command::new("remote")
                .alias("r")
                .arg(Arg::new("name").required()),
        );

    let matches: Matches = command
        .parse_from([
            "workspace",
            "-v",
            "-c",
            "ritty.toml",
            "--level",
            "debug",
            "r",
            "origin",
        ])
        .unwrap();

    assert_eq!(matches.argument("workspace"), Some("workspace"));
    assert_eq!(matches.option("config"), Some("ritty.toml"));
    assert_eq!(matches.enum_option("level"), Some("debug"));
    assert_eq!(matches.subcommand(), Some("remote"));
    assert_eq!(
        matches.subcommand_matches().unwrap().argument("name"),
        Some("origin")
    );

    let error: ParseError = command.parse_from(["workspace", "--unknown"]).unwrap_err();
    let run_error: RunError = command.run_from(["workspace", "--unknown"]).unwrap_err();
    let boxed: BoxError = std::io::Error::other("public error").into();
    let result: HandlerResult = Ok(());
    let output = CommandOutput::new(7_u8);
    assert_public_types(&matches, output, result, error, run_error, boxed);

    let _: fn(&CommandContext<'_>) = accepts_context;
}
