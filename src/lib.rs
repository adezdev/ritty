//! Ritty is a synchronous, builder-oriented framework for defining and
//! executing command-line interfaces. It is inspired by UnJS Citty, adapted
//! to idiomatic Rust, and has no runtime dependencies.
//!
//! A CLI starts with a [`Command`]. Add positional [`Arg`] values, boolean
//! [`Flag`] values, [`StringOption`] values, and constrained [`EnumOption`]
//! values, then attach a handler:
//!
//! ```
//! use ritty::prelude::*;
//!
//! let command = Command::new("greet")
//!     .arg(Arg::new("name").default("world"))
//!     .flag(Flag::new("excited").short('e'))
//!     .handler(|ctx| {
//!         let name = ctx.matches().argument("name").unwrap();
//!         let punctuation = if ctx.matches().flag("excited") { "!" } else { "." };
//!         Ok(format!("Hello, {name}{punctuation}"))
//!     });
//!
//! let output = command.run_from(["-e", "Ferris"])?;
//! assert_eq!(output.downcast::<String>().unwrap(), "Hello, Ferris!");
//! # Ok::<(), RunError>(())
//! ```
//!
//! [`Command::run_from`] is the programmatic argv API: it parses input,
//! executes the selected handler and lifecycle, and returns a type-erased
//! [`CommandOutput`]. [`Command::run`] consumes the current process's argv,
//! handles built-in help/version requests, and is normally used by binaries.
//!
//! Nested commands use [`Command::command`], while synchronous, cached lazy
//! children use [`Command::lazy_command`]. Parsed values are exposed through
//! [`Matches`], and parse/execution failures through [`ParseError`] and
//! [`RunError`]. Lifecycle hooks and reusable [`Plugin`] values are available
//! for setup and cleanup work.
//!
//! Import from the crate root or the logical [`argument`], [`command`],
//! [`error`], and [`mod@matches`] modules. [`prelude`] collects the common
//! public API for applications that prefer a single glob import.

#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod argument;
pub mod command;
pub mod error;
pub mod matches;

pub use argument::{Arg, EnumOption, Flag, StringOption};
pub use command::{BoxError, Command, CommandContext, CommandOutput, HandlerResult, Plugin};
pub use error::{ArgumentErrorKind, ParseError, ParseErrorKind, RunError};
pub use matches::Matches;

/// Common types for building and running a Ritty command-line interface.
///
/// This is a convenience import; the same types remain available from the
/// crate root and their logical modules.
pub mod prelude {
    pub use crate::{
        Arg, ArgumentErrorKind, BoxError, Command, CommandContext, CommandOutput, EnumOption, Flag,
        HandlerResult, Matches, ParseError, ParseErrorKind, Plugin, RunError, StringOption,
    };
}
