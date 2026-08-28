use ritty::prelude::*;

#[test]
fn prelude_builds_and_runs_an_ordinary_cli() {
    let command = Command::new("ritty")
        .flag(Flag::new("quiet"))
        .option(StringOption::new("message"))
        .enum_option(EnumOption::new("format", ["text", "json"]))
        .arg(Arg::new("recipient"))
        .handler(
            |context: &CommandContext<'_>| -> HandlerResult<(bool, String)> {
                Ok((
                    context.matches().flag("quiet"),
                    context
                        .matches()
                        .option("message")
                        .unwrap_or_default()
                        .to_owned(),
                ))
            },
        );

    let output = command
        .run_from(["--quiet", "--message", "hello", "--format", "text", "world"])
        .unwrap();

    assert_eq!(
        output.downcast::<(bool, String)>().unwrap(),
        (true, "hello".to_owned())
    );
}
