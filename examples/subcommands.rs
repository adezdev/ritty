use ritty::prelude::*;

fn main() -> Result<(), RunError> {
    let command = Command::new("repo")
        .description("Manage a repository")
        .default_subcommand("status")
        .command(
            Command::new("status")
                .alias("st")
                .description("Show repository status")
                .handler(|ctx| {
                    println!("{}: working tree clean", ctx.command().name());
                    Ok(())
                }),
        )
        .command(
            Command::new("remote")
                .alias("r")
                .description("Manage remotes")
                .command(
                    Command::new("add")
                        .description("Add a remote")
                        .arg(Arg::new("name").required())
                        .arg(Arg::new("url").required())
                        .handler(|ctx| {
                            let name = ctx.matches().argument("name").unwrap();
                            let url = ctx.matches().argument("url").unwrap();
                            let top_level = ctx.root_matches().subcommand().unwrap();
                            println!("{top_level}: added {name} at {url}");
                            Ok(())
                        }),
                ),
        );

    command.run()
}
