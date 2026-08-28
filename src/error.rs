//! Structured parsing and execution errors.
//!
//! [`ParseError`] pairs a human-readable message with [`ParseErrorKind`] and
//! [`ArgumentErrorKind`] classifications. [`RunError`] preserves parse errors
//! and distinguishes handler, lifecycle, built-in, and output failures. Import
//! this module directly when matching errors without the root facade.

use crate::command::BoxError;

/// The specific kind of argument/option-level parse failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentErrorKind {
    /// An option token (long, short, or `=` form) matched no declared flag or option.
    UnknownOption,
    /// An option token matched more than one declared flag/option/enum option.
    AmbiguousOption,
    /// A string or enum option was given with no following value token.
    MissingOptionValue,
    /// A declared option received a value it rejects: an enum option's
    /// value matched none of its declared allowed values, or a boolean
    /// flag's explicit `=value` was neither `true` nor `false`.
    InvalidOptionValue,
    /// A required positional argument was not supplied.
    MissingRequiredArgument,
    /// A required flag, string option, or enum option was not supplied.
    MissingRequiredOption,
    /// A positional token was supplied with no remaining argument slot to receive it.
    UnexpectedArgument,
}

/// The top-level classification of a [`ParseError`].
///
/// Argument and option failures carry the more specific
/// [`ArgumentErrorKind`] classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// An argument/option-level failure; see `ArgumentErrorKind` for the subtype.
    Argument(ArgumentErrorKind),
    /// An explicit subcommand or alias token matched no declared subcommand.
    UnknownCommand,
    /// A subcommand or alias token matched more than one declared subcommand.
    AmbiguousCommand,
    /// A configured default subcommand name did not resolve to exactly one child.
    DefaultSubcommandNotFound,
}

/// An error produced while parsing command-line input.
///
/// [`Self::kind`] is intended for programmatic handling, while
/// [`std::fmt::Display`] supplies the user-facing message.
///
/// # Example
///
/// ```
/// use ritty::{ArgumentErrorKind, Command, ParseErrorKind};
///
/// let error = Command::new("tool")
///     .parse_from(["--unknown"])
///     .unwrap_err();
///
/// match error.kind() {
///     ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption) => {
///         assert_eq!(error.to_string(), "unknown flag: --unknown");
///     }
///     other => panic!("unexpected parse error: {other:?}"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    kind: ParseErrorKind,
    message: String,
}

impl ParseError {
    /// Constructs a `ParseError` with an explicit kind and message.
    pub(crate) fn new(kind: ParseErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// Returns the machine-readable classification of this error.
    pub fn kind(&self) -> ParseErrorKind {
        self.kind
    }

    /// Returns the parse error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ParseError {}
/// An error produced while executing a parsed command.
#[derive(Debug)]
pub enum RunError {
    /// Parsing the given arguments failed; the original `ParseError` is preserved.
    Parse(ParseError),
    /// No handler could be selected: a command has subcommands, none was
    /// selected (explicitly or via default), and the command itself has no handler.
    NoCommand,
    /// A command's setup hook returned an error. The command's cleanup hook
    /// still ran (if any); this is the primary failure.
    Setup(BoxError),
    /// The selected handler returned an error.
    Handler(BoxError),
    /// A command's cleanup hook returned an error, and no earlier setup,
    /// handler, or nested-execution failure took precedence over it.
    Cleanup(BoxError),
    /// A plugin's setup hook returned an error. Later plugin setups, the
    /// command's own setup, and its work are skipped, but command cleanup
    /// and every registered plugin's cleanup still run.
    PluginSetup {
        /// The failing plugin's name.
        plugin: String,
        /// The original error returned by the plugin's setup hook.
        source: BoxError,
    },
    /// A plugin's cleanup hook returned an error, and no earlier setup,
    /// handler, nested-execution, or command-cleanup failure took
    /// precedence over it.
    PluginCleanup {
        /// The failing plugin's name.
        plugin: String,
        /// The original error returned by the plugin's cleanup hook.
        source: BoxError,
    },
    /// An enabled root built-in `--version`/`-v` request was made using the
    /// exact single-token CLI form, but the root command declares no
    /// version.
    NoVersion,
    /// Writing a built-in help/version response to stdout failed.
    Io(std::io::Error),
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::Parse(err) => write!(f, "{err}"),
            RunError::NoCommand => f.write_str("no command specified"),
            RunError::Setup(err) => write!(f, "{err}"),
            RunError::Handler(err) => write!(f, "{err}"),
            RunError::Cleanup(err) => write!(f, "{err}"),
            RunError::PluginSetup { plugin, source } => {
                write!(f, "plugin {plugin} setup failed: {source}")
            }
            RunError::PluginCleanup { plugin, source } => {
                write!(f, "plugin {plugin} cleanup failed: {source}")
            }
            RunError::NoVersion => f.write_str("no version specified"),
            RunError::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for RunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RunError::Parse(err) => Some(err),
            RunError::NoCommand => None,
            RunError::Setup(err) => Some(err.as_ref()),
            RunError::Handler(err) => Some(err.as_ref()),
            RunError::Cleanup(err) => Some(err.as_ref()),
            RunError::PluginSetup { source, .. } => Some(source.as_ref()),
            RunError::PluginCleanup { source, .. } => Some(source.as_ref()),
            RunError::NoVersion => None,
            RunError::Io(err) => Some(err),
        }
    }
}

impl From<ParseError> for RunError {
    fn from(err: ParseError) -> Self {
        RunError::Parse(err)
    }
}
