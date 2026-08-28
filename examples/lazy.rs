use ritty::prelude::*;

fn main() -> Result<(), RunError> {
    let command = Command::new("ritty")
        .description("Project tasks")
        .lazy_command("build", || {
            Command::new("build")
                .description("Build the project")
                .enum_option(
                    EnumOption::new("profile", ["debug", "release"])
                        .alias("p")
                        .default("debug"),
                )
                .handler(|ctx| {
                    println!(
                        "building with the {} profile",
                        ctx.matches().enum_option("profile").unwrap()
                    );
                    Ok(())
                })
        });

    command.run()
}
