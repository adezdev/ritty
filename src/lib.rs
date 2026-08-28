//! Ritty — an elegant CLI builder for Rust.

pub mod argument;
pub mod command;
pub mod error;
pub mod matches;

pub use argument::{Arg, EnumOption, Flag, StringOption};
pub use command::{BoxError, Command, CommandContext, CommandOutput, HandlerResult, Plugin};
pub use error::{ArgumentErrorKind, ParseError, ParseErrorKind, RunError};
pub use matches::Matches;

/// Common types for building and running a Ritty command-line interface.
pub mod prelude {
    pub use crate::{
        Arg, ArgumentErrorKind, BoxError, Command, CommandContext, CommandOutput, EnumOption, Flag,
        HandlerResult, Matches, ParseError, ParseErrorKind, Plugin, RunError, StringOption,
    };
}
