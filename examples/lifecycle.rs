use ritty::prelude::*;

fn main() -> Result<(), RunError> {
    let logging = Plugin::new("logging")
        .setup(|ctx| {
            println!("plugin setup: {}", ctx.command().name());
            Ok(())
        })
        .cleanup(|ctx| {
            println!("plugin cleanup: {}", ctx.command().name());
            Ok(())
        });

    Command::new("lifecycle")
        .description("Demonstrate lifecycle ordering")
        .plugin(logging)
        .setup(|ctx| {
            println!("command setup: {}", ctx.command().name());
            Ok(())
        })
        .cleanup(|ctx| {
            println!("command cleanup: {}", ctx.command().name());
            Ok(())
        })
        .handler(|_| {
            println!("handler");
            Ok(())
        })
        .run()
}
