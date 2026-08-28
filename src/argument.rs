//! Input definitions accepted by a [`Command`](crate::Command).
//!
//! Import from this module when defining positional [`Arg`] values, boolean
//! [`Flag`] values, free-form [`StringOption`] values, or constrained
//! [`EnumOption`] values without using Ritty's root facade or prelude.

/// A positional argument in a Ritty command.
///
/// Arguments bind positionally in declaration order. A default satisfies a
/// required argument when no token is supplied.
///
/// # Example
///
/// ```
/// use ritty::{Arg, Command};
///
/// let command = Command::new("copy")
///     .arg(Arg::new("source").required().value_hint("file"))
///     .arg(Arg::new("destination").default("."));
/// let matches = command.parse_from(["notes.txt"])?;
///
/// assert_eq!(matches.argument("source"), Some("notes.txt"));
/// assert_eq!(matches.argument("destination"), Some("."));
/// # Ok::<(), ritty::ParseError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    name: String,
    description: Option<String>,
    value_hint: Option<String>,
    required: bool,
    default: Option<String>,
}

impl Arg {
    /// Creates a new positional argument.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            value_hint: None,
            required: false,
            default: None,
        }
    }

    /// Sets the argument description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the argument's value hint, for usage rendering.
    pub fn value_hint(mut self, value_hint: impl Into<String>) -> Self {
        self.value_hint = Some(value_hint.into());
        self
    }

    /// Marks the argument as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the default value used when the argument is not supplied.
    pub fn default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }

    /// Returns whether the argument is required.
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Returns the argument name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the argument's default value, if any.
    pub fn default_value(&self) -> Option<&str> {
        self.default.as_deref()
    }

    /// Returns the argument description.
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the argument's value hint.
    pub fn get_value_hint(&self) -> Option<&str> {
        self.value_hint.as_deref()
    }
}

/// A named string option in a Ritty command, such as `--output target`.
///
/// One-character aliases work as short options, and every alias is
/// canonicalized to the declared name in [`Matches`](crate::Matches).
///
/// # Example
///
/// ```
/// use ritty::{Command, StringOption};
///
/// let command = Command::new("build").option(
///     StringOption::new("output")
///         .alias("o")
///         .description("Output directory")
///         .value_hint("dir"),
/// );
/// let matches = command.parse_from(["-o", "dist"])?;
///
/// assert_eq!(matches.option("output"), Some("dist"));
/// # Ok::<(), ritty::ParseError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringOption {
    name: String,
    aliases: Vec<String>,
    description: Option<String>,
    value_hint: Option<String>,
    required: bool,
    default: Option<String>,
}

impl StringOption {
    /// Creates a new string option.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aliases: Vec::new(),
            description: None,
            value_hint: None,
            required: false,
            default: None,
        }
    }

    /// Adds an alias. A single-character alias can also be used as a short
    /// option (`-o`); a multi-character alias is a long-option alias (`--destination`).
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Returns the option's aliases, in insertion order.
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    /// Sets the option description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the option's value hint, for usage rendering.
    pub fn value_hint(mut self, value_hint: impl Into<String>) -> Self {
        self.value_hint = Some(value_hint.into());
        self
    }

    /// Marks the option as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the default value used when the option is not supplied.
    pub fn default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }

    /// Returns the option name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the option is required.
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Returns the option's default value, if any.
    pub fn default_value(&self) -> Option<&str> {
        self.default.as_deref()
    }

    /// Returns the option description.
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the option's value hint.
    pub fn get_value_hint(&self) -> Option<&str> {
        self.value_hint.as_deref()
    }
}

/// A named enum option whose value must belong to a declared set.
///
/// An empty allowed-value list accepts any value. Otherwise invalid explicit
/// values and invalid defaults produce a structured parse error.
///
/// # Example
///
/// ```
/// use ritty::{Command, EnumOption};
///
/// let command = Command::new("report").enum_option(
///     EnumOption::new("format", ["text", "json"])
///         .alias("f")
///         .default("text"),
/// );
/// let matches = command.parse_from(["--format=json"])?;
///
/// assert_eq!(matches.enum_option("format"), Some("json"));
/// # Ok::<(), ritty::ParseError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumOption {
    name: String,
    aliases: Vec<String>,
    values: Vec<String>,
    description: Option<String>,
    value_hint: Option<String>,
    required: bool,
    default: Option<String>,
}

impl EnumOption {
    /// Creates a new enum option with the given allowed values, in
    /// declaration order. An empty list means no value restriction.
    pub fn new(
        name: impl Into<String>,
        values: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            name: name.into(),
            aliases: Vec::new(),
            values: values.into_iter().map(Into::into).collect(),
            description: None,
            value_hint: None,
            required: false,
            default: None,
        }
    }

    /// Adds an alias. A single-character alias can also be used as a short
    /// option (`-l`); a multi-character alias is a long-option alias (`--log-level`).
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Returns the option's aliases, in insertion order.
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    /// Returns the option's allowed values, in declaration order.
    pub fn values(&self) -> &[String] {
        &self.values
    }

    /// Sets the option description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the option's value hint, for usage rendering.
    pub fn value_hint(mut self, value_hint: impl Into<String>) -> Self {
        self.value_hint = Some(value_hint.into());
        self
    }

    /// Marks the option as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Sets the default value used when the option is not supplied.
    pub fn default(mut self, default: impl Into<String>) -> Self {
        self.default = Some(default.into());
        self
    }

    /// Returns the option name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns whether the option is required.
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Returns the option's default value, if any.
    pub fn default_value(&self) -> Option<&str> {
        self.default.as_deref()
    }

    /// Returns the option description.
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the option's value hint.
    pub fn get_value_hint(&self) -> Option<&str> {
        self.value_hint.as_deref()
    }
}

/// A boolean flag in a Ritty command.
///
/// A flag is enabled by its positive spelling and disabled by its `--no-*`
/// spelling. Explicit `=value` forms accept only lowercase `true` or `false`.
///
/// # Example
///
/// ```
/// use ritty::{Command, Flag};
///
/// let command = Command::new("deploy").flag(
///     Flag::new("verbose")
///         .short('v')
///         .alias("chatty")
///         .default(true),
/// );
///
/// assert!(!command.parse_from(["--no-verbose"])?.flag("verbose"));
/// assert!(!command.parse_from(["-v=false"])?.flag("verbose"));
/// assert!(command.parse_from(["--chatty=true"])?.flag("verbose"));
/// # Ok::<(), ritty::ParseError>(())
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flag {
    name: String,
    short: Option<char>,
    aliases: Vec<String>,
    description: Option<String>,
    negative_description: Option<String>,
    value_hint: Option<String>,
    required: bool,
    default: Option<bool>,
}

impl Flag {
    /// Creates a new flag.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            short: None,
            aliases: Vec::new(),
            description: None,
            negative_description: None,
            value_hint: None,
            required: false,
            default: None,
        }
    }

    /// Sets the short flag name.
    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    /// Adds an alias. A single-character alias can also be used as a short
    /// flag (`-q`); a multi-character alias is a long-flag alias (`--chatty`).
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Returns the flag's aliases, in insertion order.
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    /// Sets the flag description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the flag description.
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Sets the description shown for the `--no-*` negation.
    pub fn negative_description(mut self, description: impl Into<String>) -> Self {
        self.negative_description = Some(description.into());
        self
    }

    /// Returns the negative-description metadata.
    pub fn get_negative_description(&self) -> Option<&str> {
        self.negative_description.as_deref()
    }

    /// Sets the flag's value hint, for usage rendering.
    pub fn value_hint(mut self, value_hint: impl Into<String>) -> Self {
        self.value_hint = Some(value_hint.into());
        self
    }

    /// Returns the flag's value hint.
    pub fn get_value_hint(&self) -> Option<&str> {
        self.value_hint.as_deref()
    }

    /// Marks the flag as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// Returns whether the flag is required.
    pub fn is_required(&self) -> bool {
        self.required
    }

    /// Sets the default value used when the flag is not supplied.
    pub fn default(mut self, default: bool) -> Self {
        self.default = Some(default);
        self
    }

    /// Returns the flag's default value, if any.
    pub fn default_value(&self) -> Option<bool> {
        self.default
    }

    /// Returns the flag name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the short flag name.
    pub fn short_name(&self) -> Option<char> {
        self.short
    }
}
