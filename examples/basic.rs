use ritty::prelude::*;

fn main() -> Result<(), RunError> {
    let command = Command::new("greet")
        .description("Print a greeting")
        .version("0.1.0")
        .arg(
            Arg::new("name")
                .description("Person to greet")
                .default("world"),
        )
        .flag(
            Flag::new("excited")
                .short('e')
                .description("Use enthusiastic punctuation"),
        )
        .enum_option(
            EnumOption::new("style", ["casual", "formal"])
                .alias("s")
                .description("Greeting style")
                .default("casual"),
        )
        .handler(|ctx| {
            let name = ctx.matches().argument("name").unwrap();
            let greeting = match ctx.matches().enum_option("style").unwrap() {
                "formal" => "Good day",
                _ => "Hello",
            };
            let punctuation = if ctx.matches().flag("excited") {
                "!"
            } else {
                "."
            };

            println!("{greeting}, {name}{punctuation}");
            Ok(())
        });

    command.run()
}
