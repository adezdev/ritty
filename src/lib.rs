//! Ritty — an elegant CLI builder for Rust.

use std::collections::HashSet;
use std::iter::once;
use std::sync::Arc;

/// A positional argument in a Ritty command.
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

/// A named string option in a Ritty command, e.g. `--name value`.
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

/// A named enum option in a Ritty command, e.g. `--level info`, whose value
/// must belong to a declared set of allowed values.
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

/// Parsed command-line matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matches {
    flags: Vec<(String, bool)>,
    arguments: Vec<(String, String)>,
    options: Vec<(String, String)>,
    enum_options: Vec<(String, String)>,
    subcommand: Option<String>,
    subcommand_matches: Option<Box<Matches>>,
}

impl Matches {
    /// Returns the flag's effective boolean value (`false` if absent).
    pub fn flag(&self, name: &str) -> bool {
        self.flags
            .iter()
            .find(|(flag, _)| flag == name)
            .map(|(_, value)| *value)
            .unwrap_or(false)
    }

    /// Records the effective boolean state for a canonical flag name,
    /// overwriting any prior state so only one value survives per flag.
    fn set_flag(&mut self, name: &str, value: bool) {
        if let Some(entry) = self.flags.iter_mut().find(|(flag, _)| flag == name) {
            entry.1 = value;
        } else {
            self.flags.push((name.to_owned(), value));
        }
    }

    /// Returns the value of a positional argument.
    pub fn argument(&self, name: &str) -> Option<&str> {
        self.arguments
            .iter()
            .find(|(argument, _)| argument == name)
            .map(|(_, value)| value.as_str())
    }

    /// Returns the value of a string option.
    pub fn option(&self, name: &str) -> Option<&str> {
        self.options
            .iter()
            .find(|(option, _)| option == name)
            .map(|(_, value)| value.as_str())
    }

    /// Returns the value of an enum option.
    pub fn enum_option(&self, name: &str) -> Option<&str> {
        self.enum_options
            .iter()
            .find(|(option, _)| option == name)
            .map(|(_, value)| value.as_str())
    }

    /// Returns the selected subcommand's canonical name.
    pub fn subcommand(&self) -> Option<&str> {
        self.subcommand.as_deref()
    }

    /// Returns the selected subcommand's own parsed matches.
    pub fn subcommand_matches(&self) -> Option<&Matches> {
        self.subcommand_matches.as_deref()
    }
}

/// The specific kind of argument/option-level parse failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentErrorKind {
    /// An option token (long, short, or `=` form) matched no declared flag or option.
    UnknownOption,
    /// An option token matched more than one declared flag/option/enum option.
    AmbiguousOption,
    /// A string or enum option was given with no following value token.
    MissingOptionValue,
    /// An enum option's value did not match any of its declared allowed values.
    InvalidOptionValue,
    /// A required positional argument was not supplied.
    MissingRequiredArgument,
    /// A required flag, string option, or enum option was not supplied.
    MissingRequiredOption,
    /// A positional token was supplied with no remaining argument slot to receive it.
    UnexpectedArgument,
}

/// The top-level classification of a `ParseError`.
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    kind: ParseErrorKind,
    message: String,
}

impl ParseError {
    /// Constructs a `ParseError` with an explicit kind and message.
    fn new(kind: ParseErrorKind, message: impl Into<String>) -> Self {
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

/// Resolution of a bare long-option token to either a flag (with its
/// effective positive/negative value) or a string option.
enum LongMatch<'a> {
    Flag(&'a Flag, bool),
    Option(&'a StringOption),
    EnumOption(&'a EnumOption),
}

/// Resolution of a bare short-option token to a flag, string option, or enum option.
enum ShortMatch<'a> {
    Flag(&'a Flag),
    Option(&'a StringOption),
    EnumOption(&'a EnumOption),
}

/// Whether a recognized option consumes a following token as its value.
enum OptionArity {
    Flag,
    Value,
}

/// Determines whether `name` is recognized as a bare long option by
/// `command`, or transitively by its default-subcommand chain, and if so
/// with what arity. Used to decide, at the level currently being parsed, how
/// many raw tokens to hold back for a default child without fully resolving
/// the option there — the child (or its own default chain) re-resolves and
/// consumes the held-back tokens itself.
fn probe_long(command: &Command, name: &str) -> Result<Option<OptionArity>, ParseError> {
    if let Some(m) = command.resolve_long(name)? {
        return Ok(Some(match m {
            LongMatch::Flag(_, _) => OptionArity::Flag,
            LongMatch::Option(_) | LongMatch::EnumOption(_) => OptionArity::Value,
        }));
    }

    match command.resolve_default_child()? {
        Some(next) => probe_long(next, name),
        None => Ok(None),
    }
}

/// Short-option counterpart to `probe_long`.
fn probe_short(command: &Command, short: char) -> Result<Option<OptionArity>, ParseError> {
    if let Some(m) = command.resolve_short(short)? {
        return Ok(Some(match m {
            ShortMatch::Flag(_) => OptionArity::Flag,
            ShortMatch::Option(_) | ShortMatch::EnumOption(_) => OptionArity::Value,
        }));
    }

    match command.resolve_default_child()? {
        Some(next) => probe_short(next, short),
        None => Ok(None),
    }
}

/// Determines whether `--name=value` is recognized by `command`, or
/// transitively by its default-subcommand chain.
fn probe_long_equals(command: &Command, name: &str) -> Result<bool, ParseError> {
    let (string_match, enum_match) = command.long_equals_candidates(name)?;
    if string_match.is_some() || enum_match.is_some() {
        return Ok(true);
    }

    match command.resolve_default_child()? {
        Some(next) => probe_long_equals(next, name),
        None => Ok(false),
    }
}

/// Short-option counterpart to `probe_long_equals`.
fn probe_short_equals(command: &Command, short: char) -> Result<bool, ParseError> {
    let (string_match, enum_match) = command.short_equals_candidates(short)?;
    if string_match.is_some() || enum_match.is_some() {
        return Ok(true);
    }

    match command.resolve_default_child()? {
        Some(next) => probe_short_equals(next, short),
        None => Ok(false),
    }
}

/// Validates an enum option's effective value against its allowed values.
/// An empty allowed-value list means there is no restriction.
fn validate_enum_value(option: &EnumOption, value: &str) -> Result<(), ParseError> {
    if option.values().is_empty() || option.values().iter().any(|allowed| allowed == value) {
        return Ok(());
    }

    Err(ParseError::new(
        ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue),
        format!(
            "invalid value for option: --{}: {} (expected one of: {})",
            option.name(),
            value,
            option.values().join(", ")
        ),
    ))
}

/// A two-column usage row: left-hand label and right-hand detail. An empty
/// detail means the row has no second column.
type UsageRow = (String, String);

/// Renders rows as a deterministic, left-aligned two-column block, indented
/// two spaces, with no trailing whitespace on rows that lack a detail.
fn render_rows(rows: &[UsageRow]) -> String {
    let width = rows
        .iter()
        .map(|(label, _)| label.chars().count())
        .max()
        .unwrap_or(0);
    rows.iter()
        .map(|(label, detail)| {
            if detail.is_empty() {
                format!("  {label}")
            } else {
                format!("  {label:width$}  {detail}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn dedup_strings(items: Vec<String>) -> Vec<String> {
    let mut seen = HashSet::new();
    items
        .into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}

fn dedup_chars(items: Vec<char>) -> Vec<char> {
    let mut seen = HashSet::new();
    items
        .into_iter()
        .filter(|item| seen.insert(*item))
        .collect()
}

/// Extracts `value` as a single Unicode scalar, the definition of "one
/// short-option character" shared by the parser and usage renderer.
/// `None` for an empty string or one with more than one scalar — in
/// particular, a multi-byte UTF-8 encoding of a single `char` still counts
/// as one.
fn single_char(value: &str) -> Option<char> {
    let mut chars = value.chars();
    let first = chars.next()?;
    if chars.next().is_some() {
        return None;
    }
    Some(first)
}

/// Single-character aliases, which the parser also accepts as short options.
fn short_aliases(aliases: &[String]) -> Vec<char> {
    aliases
        .iter()
        .filter_map(|a| single_char(a.as_str()))
        .collect()
}

/// Multi-character aliases, which the parser treats as long-option spellings.
fn long_aliases(aliases: &[String]) -> Vec<&str> {
    aliases
        .iter()
        .filter(|a| a.chars().count() > 1)
        .map(String::as_str)
        .collect()
}

/// Long spellings in display order: aliases first (insertion order), then
/// the canonical name, deduplicated.
fn long_spellings(canonical: &str, aliases: &[String]) -> Vec<String> {
    let names = long_aliases(aliases)
        .into_iter()
        .map(String::from)
        .chain(once(canonical.to_string()))
        .collect();
    dedup_strings(names)
}

/// Joins short and long spellings into one display string, e.g.
/// `-o, --output=<dir>`. Only the canonical long spelling carries the value
/// marker; aliases are shown bare.
fn format_spellings(
    shorts: &[char],
    longs: &[String],
    value_marker: Option<&str>,
    canonical: &str,
) -> String {
    let mut parts: Vec<String> = shorts.iter().map(|c| format!("-{c}")).collect();
    parts.extend(longs.iter().map(|name| match value_marker {
        Some(marker) if name.as_str() == canonical => format!("--{name}={marker}"),
        _ => format!("--{name}"),
    }));
    parts.join(", ")
}

/// `(Required)` when required and unsatisfied by a default; `(Default: x)`
/// when a default exists. A default always satisfies the requirement, so the
/// two markers are mutually exclusive.
fn required_or_default_annotation(required: bool, default: Option<&str>) -> Option<String> {
    match default {
        Some(default) => Some(format!("(Default: {default})")),
        None if required => Some("(Required)".to_string()),
        None => None,
    }
}

fn combine_description(description: Option<&str>, annotation: Option<String>) -> String {
    match (description, annotation) {
        (Some(d), Some(a)) => format!("{d} {a}"),
        (Some(d), None) => d.to_string(),
        (None, Some(a)) => a,
        (None, None) => String::new(),
    }
}

fn render_argument_row(arg: &Arg) -> UsageRow {
    let mut label = arg.name().to_uppercase();
    if let Some(hint) = arg.get_value_hint() {
        label.push_str(&format!(" <{hint}>"));
    }
    let annotation = required_or_default_annotation(arg.is_required(), arg.default_value());
    (
        label,
        combine_description(arg.get_description(), annotation),
    )
}

fn render_string_option_row(option: &StringOption) -> UsageRow {
    let shorts = dedup_chars(short_aliases(option.aliases()));
    let longs = long_spellings(option.name(), option.aliases());
    let marker = format!("<{}>", option.get_value_hint().unwrap_or(option.name()));
    let label = format_spellings(&shorts, &longs, Some(&marker), option.name());
    let annotation = required_or_default_annotation(option.is_required(), option.default_value());
    (
        label,
        combine_description(option.get_description(), annotation),
    )
}

fn render_enum_option_row(option: &EnumOption) -> UsageRow {
    let shorts = dedup_chars(short_aliases(option.aliases()));
    let longs = long_spellings(option.name(), option.aliases());
    let marker = format!("<{}>", option.values().join("|"));
    let label = format_spellings(&shorts, &longs, Some(&marker), option.name());
    let annotation = required_or_default_annotation(option.is_required(), option.default_value());
    (
        label,
        combine_description(option.get_description(), annotation),
    )
}

/// Whether a `--no-*` row should be rendered for this flag: it must be
/// parseable (the canonical name doesn't already read as a negation) and
/// either default to `true` or carry an explicit negative description.
fn negative_row_eligible(flag: &Flag) -> bool {
    (flag.default_value() == Some(true) || flag.get_negative_description().is_some())
        && !flag.name().starts_with("no-")
}

/// `--no-*` spellings the parser actually accepts: `--no-` plus every alias
/// (the parser's long-option matching does not distinguish alias length for
/// negation, so even a one-character alias yields a valid `--no-x`) and the
/// canonical name. A dedicated `.short()` is not an alias and so does not by
/// itself produce a `--no-*` spelling.
fn negative_spellings(flag: &Flag) -> Vec<String> {
    let names = flag
        .aliases()
        .iter()
        .cloned()
        .chain(once(flag.name().to_string()))
        .collect();
    dedup_strings(names)
        .into_iter()
        .map(|n| format!("--no-{n}"))
        .collect()
}

fn render_flag_rows(flag: &Flag) -> Vec<UsageRow> {
    let shorts = dedup_chars(
        flag.short_name()
            .into_iter()
            .chain(short_aliases(flag.aliases()))
            .collect(),
    );
    let longs = long_spellings(flag.name(), flag.aliases());
    let label = format_spellings(&shorts, &longs, None, flag.name());
    let default_str = flag.default_value().map(|b| b.to_string());
    let annotation = required_or_default_annotation(flag.is_required(), default_str.as_deref());

    let mut rows = vec![(
        label,
        combine_description(flag.get_description(), annotation),
    )];

    if negative_row_eligible(flag) {
        let neg_label = negative_spellings(flag).join(", ");
        let neg_detail = flag.get_negative_description().unwrap_or("").to_string();
        rows.push((neg_label, neg_detail));
    }

    rows
}

fn render_command_rows(command: &Command) -> Vec<UsageRow> {
    command
        .visible_subcommands()
        .map(|c| {
            let mut label = c.name().to_string();
            for alias in c.aliases() {
                label.push_str(", ");
                label.push_str(alias);
            }
            (label, c.get_description().unwrap_or("").to_string())
        })
        .collect()
}

/// `{description} ({name} v{version})`, degrading cleanly when either piece
/// is absent; `None` when neither is present.
fn render_header(description: Option<&str>, name: &str, version: Option<&str>) -> Option<String> {
    match (description, version) {
        (Some(d), Some(v)) => Some(format!("{d} ({name} v{v})")),
        (Some(d), None) => Some(d.to_string()),
        (None, Some(v)) => Some(format!("{name} v{v}")),
        (None, None) => None,
    }
}

fn synopsis_required_options(command: &Command) -> Vec<String> {
    let mut items = Vec::new();

    for flag in command.flags() {
        if flag.is_required() && flag.default_value().is_none() {
            items.push(format!("--{}", flag.name()));
        }
    }

    for option in command.options() {
        if option.is_required() && option.default_value().is_none() {
            let marker = option.get_value_hint().unwrap_or(option.name());
            items.push(format!("--{}=<{}>", option.name(), marker));
        }
    }

    for option in command.enum_options() {
        if option.is_required() && option.default_value().is_none() {
            items.push(format!(
                "--{}=<{}>",
                option.name(),
                option.values().join("|")
            ));
        }
    }

    items
}

fn synopsis_positionals(command: &Command) -> Vec<String> {
    command
        .arguments()
        .iter()
        .map(|arg| {
            let name = arg.name().to_uppercase();
            if arg.is_required() && arg.default_value().is_none() {
                format!("<{name}>")
            } else {
                format!("[{name}]")
            }
        })
        .collect()
}

/// Flattens every visible subcommand's canonical name and aliases into one
/// pipe-joined alternative expression, e.g. `build|b|test|t`.
fn synopsis_command_alternatives(command: &Command) -> Option<String> {
    let tokens: Vec<String> = command
        .visible_subcommands()
        .flat_map(|c| once(c.name().to_string()).chain(c.aliases().iter().cloned()))
        .collect();

    if tokens.is_empty() {
        None
    } else {
        Some(tokens.join("|"))
    }
}

fn render_usage_line(command: &Command, display_name: &str) -> String {
    let mut parts = vec!["USAGE".to_string(), display_name.to_string()];

    let has_options = !command.flags().is_empty()
        || !command.options().is_empty()
        || !command.enum_options().is_empty();
    if has_options {
        parts.push("[OPTIONS]".to_string());
    }

    parts.extend(synopsis_required_options(command));
    parts.extend(synopsis_positionals(command));

    if let Some(alternatives) = synopsis_command_alternatives(command) {
        parts.push(alternatives);
    }

    parts.join(" ")
}

/// A command in a Ritty CLI application.
/// The context handed to a command's handler when it runs.
///
/// `matches()` is the selected command's own parsed matches; `root_matches()`
/// is the complete top-level parse result, so a nested handler can still
/// inspect parent/global options without Ritty flattening match ownership.
#[derive(Debug)]
pub struct CommandContext<'a> {
    command: &'a Command,
    matches: &'a Matches,
    root_matches: &'a Matches,
}

impl<'a> CommandContext<'a> {
    /// Returns the command whose handler is executing.
    pub fn command(&self) -> &Command {
        self.command
    }

    /// Returns the executing command's own parsed matches.
    pub fn matches(&self) -> &Matches {
        self.matches
    }

    /// Returns the complete top-level parsed result.
    pub fn root_matches(&self) -> &Matches {
        self.root_matches
    }
}

/// A boxed error, as returned by a failing handler.
pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// The result returned by a command handler.
pub type HandlerResult = Result<(), BoxError>;

/// A shared, cloneable handler callable. Wrapped so `Command` can derive a
/// meaningful `Debug` without trying to print closure internals.
#[derive(Clone)]
struct Handler(Arc<dyn for<'a> Fn(&CommandContext<'a>) -> HandlerResult + Send + Sync>);

impl std::fmt::Debug for Handler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Handler(..)")
    }
}

/// An error produced while executing a parsed command.
#[derive(Debug)]
pub enum RunError {
    /// Parsing the given arguments failed; the original `ParseError` is preserved.
    Parse(ParseError),
    /// No handler could be selected: a command has subcommands, none was
    /// selected (explicitly or via default), and the command itself has no handler.
    NoCommand,
    /// The selected handler returned an error.
    Handler(BoxError),
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::Parse(err) => write!(f, "{err}"),
            RunError::NoCommand => f.write_str("no command specified"),
            RunError::Handler(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for RunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RunError::Parse(err) => Some(err),
            RunError::NoCommand => None,
            RunError::Handler(err) => Some(err.as_ref()),
        }
    }
}

impl From<ParseError> for RunError {
    fn from(err: ParseError) -> Self {
        RunError::Parse(err)
    }
}

#[derive(Debug, Clone)]
pub struct Command {
    name: String,
    aliases: Vec<String>,
    description: Option<String>,
    version: Option<String>,
    subcommands: Vec<Command>,
    arguments: Vec<Arg>,
    flags: Vec<Flag>,
    options: Vec<StringOption>,
    enum_options: Vec<EnumOption>,
    default_subcommand: Option<String>,
    hidden: bool,
    handler: Option<Handler>,
}

impl Command {
    /// Creates a new command.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aliases: Vec::new(),
            description: None,
            version: None,
            subcommands: Vec::new(),
            arguments: Vec::new(),
            flags: Vec::new(),
            options: Vec::new(),
            enum_options: Vec::new(),
            default_subcommand: None,
            hidden: false,
            handler: None,
        }
    }

    /// Sets the command's handler, invoked when this command is selected for
    /// execution by `run_from`. Ordinary captured closures are supported.
    pub fn handler<F>(mut self, handler: F) -> Self
    where
        F: for<'a> Fn(&CommandContext<'a>) -> HandlerResult + Send + Sync + 'static,
    {
        self.handler = Some(Handler(Arc::new(handler)));
        self
    }

    /// Returns whether the command has a handler set.
    pub fn has_handler(&self) -> bool {
        self.handler.is_some()
    }

    /// Marks the command as hidden from generated usage/help listings.
    /// Hidden is presentation-only: it has no effect on parsing — a hidden
    /// subcommand remains selectable by name, alias, or as a default subcommand.
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Returns whether the command is hidden from usage/help listings.
    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    /// Adds a subcommand alias. Aliases are exact and case-sensitive.
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }

    /// Returns the command's aliases, in insertion order.
    pub fn aliases(&self) -> &[String] {
        &self.aliases
    }

    /// Sets the command description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the command version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Adds a subcommand.
    pub fn command(mut self, command: Command) -> Self {
        self.subcommands.push(command);
        self
    }

    /// Returns the command's subcommands.
    pub fn subcommands(&self) -> &[Command] {
        &self.subcommands
    }

    /// Sets the subcommand to select automatically when no explicit child
    /// is chosen. May name a child's canonical name or one of its aliases;
    /// resolved at parse time.
    pub fn default_subcommand(mut self, name: impl Into<String>) -> Self {
        self.default_subcommand = Some(name.into());
        self
    }

    /// Returns the configured default-subcommand spelling, if any.
    pub fn get_default_subcommand(&self) -> Option<&str> {
        self.default_subcommand.as_deref()
    }

    /// Returns subcommands eligible for usage/help listings, in declaration order.
    fn visible_subcommands(&self) -> impl Iterator<Item = &Command> {
        self.subcommands.iter().filter(|c| !c.is_hidden())
    }

    /// Returns every subcommand whose canonical name or an alias matches `name`.
    fn subcommands_matching(&self, name: &str) -> Vec<&Command> {
        self.subcommands
            .iter()
            .filter(|command| command.name() == name || command.aliases().iter().any(|a| a == name))
            .collect()
    }

    /// Adds a positional argument.
    pub fn arg(mut self, arg: Arg) -> Self {
        self.arguments.push(arg);
        self
    }

    /// Returns the command's positional arguments.
    pub fn arguments(&self) -> &[Arg] {
        &self.arguments
    }

    /// Adds a flag.
    pub fn flag(mut self, flag: Flag) -> Self {
        self.flags.push(flag);
        self
    }

    /// Returns the command's flags.
    pub fn flags(&self) -> &[Flag] {
        &self.flags
    }

    /// Adds a string option.
    pub fn option(mut self, option: StringOption) -> Self {
        self.options.push(option);
        self
    }

    /// Returns the command's string options.
    pub fn options(&self) -> &[StringOption] {
        &self.options
    }

    /// Adds an enum option.
    pub fn enum_option(mut self, option: EnumOption) -> Self {
        self.enum_options.push(option);
        self
    }

    /// Returns the command's enum options.
    pub fn enum_options(&self) -> &[EnumOption] {
        &self.enum_options
    }

    /// Returns every string option whose canonical name or an alias matches `name`.
    fn options_matching_long(&self, name: &str) -> Vec<&StringOption> {
        self.options
            .iter()
            .filter(|option| option.name() == name || option.aliases().iter().any(|a| a == name))
            .collect()
    }

    /// Returns every string option that declares `short` as a single-character alias.
    fn options_matching_short(&self, short: char) -> Vec<&StringOption> {
        self.options
            .iter()
            .filter(|option| {
                option
                    .aliases()
                    .iter()
                    .any(|alias| single_char(alias) == Some(short))
            })
            .collect()
    }

    /// Returns every enum option whose canonical name or an alias matches `name`.
    fn enum_options_matching_long(&self, name: &str) -> Vec<&EnumOption> {
        self.enum_options
            .iter()
            .filter(|option| option.name() == name || option.aliases().iter().any(|a| a == name))
            .collect()
    }

    /// Returns every enum option that declares `short` as a single-character alias.
    fn enum_options_matching_short(&self, short: char) -> Vec<&EnumOption> {
        self.enum_options
            .iter()
            .filter(|option| {
                option
                    .aliases()
                    .iter()
                    .any(|alias| single_char(alias) == Some(short))
            })
            .collect()
    }

    /// Returns every flag whose canonical name or an alias matches `name`.
    fn flags_matching_long(&self, name: &str) -> Vec<&Flag> {
        self.flags
            .iter()
            .filter(|flag| flag.name() == name || flag.aliases().iter().any(|a| a == name))
            .collect()
    }

    /// Returns every flag that declares `short` as its dedicated short name
    /// or as a single-character alias.
    fn flags_matching_short(&self, short: char) -> Vec<&Flag> {
        self.flags
            .iter()
            .filter(|flag| {
                flag.short_name() == Some(short)
                    || flag
                        .aliases()
                        .iter()
                        .any(|alias| single_char(alias) == Some(short))
            })
            .collect()
    }

    /// Resolves a bare long-option token (no `--` prefix, no `=`) against
    /// declared flags (positive and `no-*` negation) and string options,
    /// erroring on ambiguity rather than silently preferring one schema.
    fn resolve_long(&self, name: &str) -> Result<Option<LongMatch<'_>>, ParseError> {
        let positive_flags = self.flags_matching_long(name);
        let string_options = self.options_matching_long(name);
        let enum_options = self.enum_options_matching_long(name);
        let negative_flags = name
            .strip_prefix("no-")
            .map(|base| self.flags_matching_long(base))
            .unwrap_or_default();

        if positive_flags.len() + string_options.len() + enum_options.len() + negative_flags.len()
            > 1
        {
            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption),
                format!("ambiguous option: --{name}"),
            ));
        }

        if let Some(flag) = positive_flags.first() {
            return Ok(Some(LongMatch::Flag(flag, true)));
        }

        if let Some(option) = string_options.first() {
            return Ok(Some(LongMatch::Option(option)));
        }

        if let Some(option) = enum_options.first() {
            return Ok(Some(LongMatch::EnumOption(option)));
        }

        if let Some(flag) = negative_flags.first() {
            return Ok(Some(LongMatch::Flag(flag, false)));
        }

        Ok(None)
    }

    /// Resolves a bare short-option token (no `=`) against declared flags,
    /// string options, and enum options, erroring on ambiguity.
    fn resolve_short(&self, short: char) -> Result<Option<ShortMatch<'_>>, ParseError> {
        let flag_candidates = self.flags_matching_short(short);
        let option_candidates = self.options_matching_short(short);
        let enum_candidates = self.enum_options_matching_short(short);

        if flag_candidates.len() + option_candidates.len() + enum_candidates.len() > 1 {
            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption),
                format!("ambiguous option: -{short}"),
            ));
        }

        if let Some(flag) = flag_candidates.first() {
            return Ok(Some(ShortMatch::Flag(flag)));
        }

        if let Some(option) = option_candidates.first() {
            return Ok(Some(ShortMatch::Option(option)));
        }

        if let Some(option) = enum_candidates.first() {
            return Ok(Some(ShortMatch::EnumOption(option)));
        }

        Ok(None)
    }

    /// Resolves a `--name=value` long-option token to whichever value-bearing
    /// schema entry owns `name`. Flags participate only in the ambiguity
    /// count: `--flag=value` is never a valid spelling for a boolean flag.
    fn long_equals_candidates(
        &self,
        name: &str,
    ) -> Result<(Option<&StringOption>, Option<&EnumOption>), ParseError> {
        let positive_flags = self.flags_matching_long(name);
        let negative_flags = name
            .strip_prefix("no-")
            .map(|base| self.flags_matching_long(base))
            .unwrap_or_default();
        let string_candidates = self.options_matching_long(name);
        let enum_candidates = self.enum_options_matching_long(name);

        if positive_flags.len()
            + negative_flags.len()
            + string_candidates.len()
            + enum_candidates.len()
            > 1
        {
            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption),
                format!("ambiguous option: --{name}"),
            ));
        }

        Ok((
            string_candidates.first().copied(),
            enum_candidates.first().copied(),
        ))
    }

    /// Resolves a `-x=value` short-option token, mirroring `long_equals_candidates`.
    fn short_equals_candidates(
        &self,
        short: char,
    ) -> Result<(Option<&StringOption>, Option<&EnumOption>), ParseError> {
        let flag_candidates = self.flags_matching_short(short);
        let string_candidates = self.options_matching_short(short);
        let enum_candidates = self.enum_options_matching_short(short);

        if flag_candidates.len() + string_candidates.len() + enum_candidates.len() > 1 {
            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption),
                format!("ambiguous option: -{short}"),
            ));
        }

        Ok((
            string_candidates.first().copied(),
            enum_candidates.first().copied(),
        ))
    }

    /// Resolves the configured default subcommand to its concrete child,
    /// re-checking alias/name collisions at each call site that needs it.
    /// Returns `Ok(None)` when no default subcommand is configured.
    fn resolve_default_child(&self) -> Result<Option<&Command>, ParseError> {
        let Some(default_name) = &self.default_subcommand else {
            return Ok(None);
        };

        let candidates = self.subcommands_matching(default_name);

        if candidates.len() > 1 {
            return Err(ParseError::new(
                ParseErrorKind::AmbiguousCommand,
                format!("ambiguous command: {default_name}"),
            ));
        }

        let child = candidates.first().copied().ok_or_else(|| {
            ParseError::new(
                ParseErrorKind::DefaultSubcommandNotFound,
                format!("default subcommand not found: {default_name}"),
            )
        })?;

        Ok(Some(child))
    }

    /// Parses command-line arguments.
    pub fn parse_from<I, S>(&self, args: I) -> Result<Matches, ParseError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args: Vec<String> = args
            .into_iter()
            .map(|arg| arg.as_ref().to_owned())
            .collect();
        self.parse_tokens(&args)
    }

    /// Parses `args` and executes the selected command's handler.
    ///
    /// Parsing runs exactly once; execution then traverses `Command` and the
    /// resulting `Matches` tree together, following the canonical subcommand
    /// selection parsing already made (explicit, aliased, or default) rather
    /// than re-examining argv. Only the selected leaf's handler runs — a
    /// parent's handler is not invoked when a child is selected.
    ///
    /// This is a synchronous, programmatic API: it does not read
    /// `std::env::args`, print, exit, render usage, or special-case
    /// help/version.
    pub fn run_from<I, S>(&self, args: I) -> Result<(), RunError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let matches = self.parse_from(args)?;
        self.execute(&matches, &matches)
    }

    /// Executes the handler selected by `matches`, recursing into a selected
    /// child by its canonical name rather than re-resolving aliases.
    fn execute<'a>(
        &'a self,
        matches: &'a Matches,
        root_matches: &'a Matches,
    ) -> Result<(), RunError> {
        if let Some(name) = matches.subcommand() {
            let child = self
                .subcommands
                .iter()
                .find(|child| child.name() == name)
                .expect("a parser-selected canonical subcommand always exists in the command tree");
            let child_matches = matches
                .subcommand_matches()
                .expect("a selected subcommand always carries its own matches");
            return child.execute(child_matches, root_matches);
        }

        if let Some(handler) = &self.handler {
            let context = CommandContext {
                command: self,
                matches,
                root_matches,
            };
            return (handler.0)(&context).map_err(RunError::Handler);
        }

        if self.subcommands.is_empty() {
            return Ok(());
        }

        Err(RunError::NoCommand)
    }

    /// Parses a slice of already-collected argv tokens against this command,
    /// recursing into a selected subcommand's own tokens once one is found.
    ///
    /// When no explicit child is selected and a default subcommand is
    /// configured, tokens this command's own schema does not recognize are
    /// held back into `child_tokens` and handed to the default child's own
    /// `parse_tokens` afterward, rather than being replayed verbatim the way
    /// Citty's `strict: false` parser does. This keeps unknown-input errors
    /// meaningful (a token accepted by neither command still errors) while
    /// letting the default child receive real argv instead of only defaults.
    fn parse_tokens(&self, args: &[String]) -> Result<Matches, ParseError> {
        let mut matches = Matches {
            flags: Vec::new(),
            arguments: Vec::new(),
            options: Vec::new(),
            enum_options: Vec::new(),
            subcommand: None,
            subcommand_matches: None,
        };
        let mut positional = 0;
        let mut terminated = false;
        let mut child_tokens: Vec<String> = Vec::new();
        let mut child_terminator_sent = false;

        let mut index = 0;

        while index < args.len() {
            let arg = args[index].as_str();

            if !terminated && arg == "--" {
                terminated = true;
                index += 1;
                continue;
            }

            if !terminated {
                if let Some(rest) = arg.strip_prefix("--") {
                    if let Some((name, value)) = rest.split_once('=') {
                        let (string_match, enum_match) = self.long_equals_candidates(name)?;

                        if let Some(option) = string_match {
                            matches
                                .options
                                .push((option.name().to_owned(), value.to_owned()));
                            index += 1;
                            continue;
                        }

                        if let Some(option) = enum_match {
                            matches
                                .enum_options
                                .push((option.name().to_owned(), value.to_owned()));
                            index += 1;
                            continue;
                        }

                        if let Some(child) = self.resolve_default_child()?
                            && probe_long_equals(child, name)?
                        {
                            child_tokens.push(arg.to_owned());
                            index += 1;
                            continue;
                        }

                        return Err(ParseError::new(
                            ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption),
                            format!("unknown flag: --{name}"),
                        ));
                    }

                    let name = rest;

                    match self.resolve_long(name)? {
                        Some(LongMatch::Flag(flag, value)) => {
                            matches.set_flag(flag.name(), value);
                            index += 1;
                            continue;
                        }
                        Some(LongMatch::Option(option)) => {
                            let value = args.get(index + 1).ok_or_else(|| {
                                ParseError::new(
                                    ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue),
                                    format!("missing value for option: --{name}"),
                                )
                            })?;
                            matches
                                .options
                                .push((option.name().to_owned(), value.to_owned()));
                            index += 2;
                            continue;
                        }
                        Some(LongMatch::EnumOption(option)) => {
                            let value = args.get(index + 1).ok_or_else(|| {
                                ParseError::new(
                                    ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue),
                                    format!("missing value for option: --{name}"),
                                )
                            })?;
                            matches
                                .enum_options
                                .push((option.name().to_owned(), value.to_owned()));
                            index += 2;
                            continue;
                        }
                        None => {
                            if let Some(child) = self.resolve_default_child()? {
                                match probe_long(child, name)? {
                                    Some(OptionArity::Flag) => {
                                        child_tokens.push(arg.to_owned());
                                        index += 1;
                                        continue;
                                    }
                                    Some(OptionArity::Value) => {
                                        child_tokens.push(arg.to_owned());
                                        if let Some(value) = args.get(index + 1) {
                                            child_tokens.push(value.clone());
                                            index += 2;
                                        } else {
                                            index += 1;
                                        }
                                        continue;
                                    }
                                    None => {}
                                }
                            }

                            return Err(ParseError::new(
                                ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption),
                                format!("unknown flag: --{name}"),
                            ));
                        }
                    }
                }

                if let Some(rest) = arg.strip_prefix('-') {
                    if let Some((name, value)) = rest.split_once('=') {
                        if let Some(short) = single_char(name) {
                            let (string_match, enum_match) = self.short_equals_candidates(short)?;

                            if let Some(option) = string_match {
                                matches
                                    .options
                                    .push((option.name().to_owned(), value.to_owned()));
                                index += 1;
                                continue;
                            }

                            if let Some(option) = enum_match {
                                matches
                                    .enum_options
                                    .push((option.name().to_owned(), value.to_owned()));
                                index += 1;
                                continue;
                            }

                            if let Some(child) = self.resolve_default_child()?
                                && probe_short_equals(child, short)?
                            {
                                child_tokens.push(arg.to_owned());
                                index += 1;
                                continue;
                            }
                        }

                        return Err(ParseError::new(
                            ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption),
                            format!("unknown flag: -{rest}"),
                        ));
                    }

                    if let Some(short) = single_char(rest) {
                        match self.resolve_short(short)? {
                            Some(ShortMatch::Flag(flag)) => {
                                matches.set_flag(flag.name(), true);
                                index += 1;
                                continue;
                            }
                            Some(ShortMatch::Option(option)) => {
                                let value = args.get(index + 1).ok_or_else(|| {
                                    ParseError::new(
                                        ParseErrorKind::Argument(
                                            ArgumentErrorKind::MissingOptionValue,
                                        ),
                                        format!("missing value for option: -{short}"),
                                    )
                                })?;
                                matches
                                    .options
                                    .push((option.name().to_owned(), value.to_owned()));
                                index += 2;
                                continue;
                            }
                            Some(ShortMatch::EnumOption(option)) => {
                                let value = args.get(index + 1).ok_or_else(|| {
                                    ParseError::new(
                                        ParseErrorKind::Argument(
                                            ArgumentErrorKind::MissingOptionValue,
                                        ),
                                        format!("missing value for option: -{short}"),
                                    )
                                })?;
                                matches
                                    .enum_options
                                    .push((option.name().to_owned(), value.to_owned()));
                                index += 2;
                                continue;
                            }
                            None => {
                                if let Some(child) = self.resolve_default_child()? {
                                    match probe_short(child, short)? {
                                        Some(OptionArity::Flag) => {
                                            child_tokens.push(arg.to_owned());
                                            index += 1;
                                            continue;
                                        }
                                        Some(OptionArity::Value) => {
                                            child_tokens.push(arg.to_owned());
                                            if let Some(value) = args.get(index + 1) {
                                                child_tokens.push(value.clone());
                                                index += 2;
                                            } else {
                                                index += 1;
                                            }
                                            continue;
                                        }
                                        None => {}
                                    }
                                }
                            }
                        }
                    }

                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption),
                        format!("unknown flag: -{rest}"),
                    ));
                }

                let candidates = self.subcommands_matching(arg);

                if candidates.len() > 1 {
                    return Err(ParseError::new(
                        ParseErrorKind::AmbiguousCommand,
                        format!("ambiguous command: {arg}"),
                    ));
                }

                if let Some(child) = candidates.first() {
                    self.finalize(&mut matches, positional)?;
                    let child_matches = child.parse_tokens(&args[index + 1..])?;
                    matches.subcommand = Some(child.name().to_owned());
                    matches.subcommand_matches = Some(Box::new(child_matches));
                    return Ok(matches);
                }
            }

            if let Some(argument) = self.arguments.get(positional) {
                matches
                    .arguments
                    .push((argument.name().to_owned(), arg.to_owned()));
                positional += 1;
                index += 1;
                continue;
            }

            // A bare token neither self nor an explicit child claims falls
            // through to the default child, whatever its own positional
            // capacity (or further default chain) turns out to be — the
            // child's own `parse_tokens` call is the one that decides
            // whether it fits or is itself excess.
            if self.resolve_default_child()?.is_some() {
                if terminated && !child_terminator_sent {
                    child_tokens.push("--".to_owned());
                    child_terminator_sent = true;
                }
                child_tokens.push(arg.to_owned());
                index += 1;
                continue;
            }

            if !self.subcommands.is_empty() {
                return Err(ParseError::new(
                    ParseErrorKind::UnknownCommand,
                    format!("unknown command: {arg}"),
                ));
            }

            return Err(ParseError::new(
                ParseErrorKind::Argument(ArgumentErrorKind::UnexpectedArgument),
                format!("unexpected argument: {arg}"),
            ));
        }

        self.finalize(&mut matches, positional)?;

        if self.default_subcommand.is_some() {
            let child = self
                .resolve_default_child()?
                .expect("resolve_default_child returns Some or errors when configured");

            let child_matches = child.parse_tokens(&child_tokens)?;
            matches.subcommand = Some(child.name().to_owned());
            matches.subcommand_matches = Some(Box::new(child_matches));
        }

        Ok(matches)
    }

    /// Applies positional/flag/option/enum defaults and required checks for
    /// this command's own schema against tokens already consumed into
    /// `matches`. Runs once the current command's own token prefix is fully
    /// parsed, before any recursion into a selected child's tokens.
    fn finalize(&self, matches: &mut Matches, positional: usize) -> Result<(), ParseError> {
        for argument in self.arguments.iter().skip(positional) {
            match argument.default_value() {
                Some(default) => matches
                    .arguments
                    .push((argument.name().to_owned(), default.to_owned())),
                None if argument.is_required() => {
                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredArgument),
                        format!("missing required argument: {}", argument.name()),
                    ));
                }
                None => {}
            }
        }

        for flag in &self.flags {
            if matches.flags.iter().any(|(name, _)| name == flag.name()) {
                continue;
            }

            match flag.default_value() {
                Some(default) => matches.set_flag(flag.name(), default),
                None if flag.is_required() => {
                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption),
                        format!("missing required option: --{}", flag.name()),
                    ));
                }
                None => {}
            }
        }

        for option in &self.options {
            if matches.option(option.name()).is_some() {
                continue;
            }

            match option.default_value() {
                Some(default) => matches
                    .options
                    .push((option.name().to_owned(), default.to_owned())),
                None if option.is_required() => {
                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption),
                        format!("missing required option: --{}", option.name()),
                    ));
                }
                None => {}
            }
        }

        for option in &self.enum_options {
            if matches.enum_option(option.name()).is_some() {
                continue;
            }

            match option.default_value() {
                Some(default) => matches
                    .enum_options
                    .push((option.name().to_owned(), default.to_owned())),
                None if option.is_required() => {
                    return Err(ParseError::new(
                        ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption),
                        format!("missing required option: --{}", option.name()),
                    ));
                }
                None => {}
            }
        }

        for option in &self.enum_options {
            if let Some(value) = matches.enum_option(option.name()) {
                validate_enum_value(option, value)?;
            }
        }

        Ok(())
    }

    /// Returns the command name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the command description.
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the command version.
    pub fn get_version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Renders usage for this command under `display_name`, falling back to
    /// `inherited_version` when this command declares none of its own. The
    /// `display_name`/`inherited_version` split exists so a future nested
    /// help traversal can render e.g. `root remote add` without redesigning
    /// the renderer.
    fn render_usage_named(&self, display_name: &str, inherited_version: Option<&str>) -> String {
        let effective_version = self.version.as_deref().or(inherited_version);
        let mut sections = Vec::new();

        if let Some(header) =
            render_header(self.description.as_deref(), display_name, effective_version)
        {
            sections.push(header);
        }

        sections.push(render_usage_line(self, display_name));

        if !self.arguments.is_empty() {
            let rows: Vec<UsageRow> = self.arguments.iter().map(render_argument_row).collect();
            sections.push(format!("ARGUMENTS\n\n{}", render_rows(&rows)));
        }

        let mut option_rows: Vec<UsageRow> = Vec::new();
        for flag in &self.flags {
            option_rows.extend(render_flag_rows(flag));
        }
        for option in &self.options {
            option_rows.push(render_string_option_row(option));
        }
        for option in &self.enum_options {
            option_rows.push(render_enum_option_row(option));
        }
        if !option_rows.is_empty() {
            sections.push(format!("OPTIONS\n\n{}", render_rows(&option_rows)));
        }

        let command_rows = render_command_rows(self);
        if !command_rows.is_empty() {
            sections.push(format!("COMMANDS\n\n{}", render_rows(&command_rows)));
        }

        sections.join("\n\n")
    }

    /// Renders deterministic, plain-text usage for this command. Hidden
    /// subcommands are omitted from the listing and the synopsis, but remain
    /// fully parseable — hidden is presentation-only.
    pub fn render_usage(&self) -> String {
        self.render_usage_named(self.name(), None)
    }

    /// Writes the rendered usage to stdout, propagating any I/O error rather
    /// than panicking.
    pub fn show_usage(&self) -> std::io::Result<()> {
        use std::io::Write;
        writeln!(std::io::stdout(), "{}", self.render_usage())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_command() {
        let command = Command::new("ritty");

        assert_eq!(command.name(), "ritty");
        assert_eq!(command.get_description(), None);
        assert_eq!(command.get_version(), None);
    }

    #[test]
    fn configures_command_metadata() {
        let command = Command::new("ritty")
            .description("Elegant CLI builder for Rust")
            .version("0.1.0");

        assert_eq!(command.name(), "ritty");
        assert_eq!(
            command.get_description(),
            Some("Elegant CLI builder for Rust")
        );
        assert_eq!(command.get_version(), Some("0.1.0"));
    }

    #[test]
    fn adds_subcommand() {
        let command = Command::new("ritty").command(Command::new("build"));

        assert_eq!(command.subcommands().len(), 1);
        assert_eq!(command.subcommands()[0].name(), "build");
    }

    #[test]
    fn adds_argument() {
        let command = Command::new("ritty").arg(Arg::new("name"));

        assert_eq!(command.arguments().len(), 1);
        assert_eq!(command.arguments()[0].name(), "name");
    }

    #[test]
    fn adds_flag() {
        let command = Command::new("ritty").flag(Flag::new("verbose").short('v'));

        assert_eq!(command.flags().len(), 1);
        assert_eq!(command.flags()[0].name(), "verbose");
        assert_eq!(command.flags()[0].short_name(), Some('v'));
    }

    #[test]
    fn parses_long_flag() {
        let command = Command::new("ritty").flag(Flag::new("verbose"));

        let matches = command.parse_from(["--verbose"]).unwrap();

        assert!(matches.flag("verbose"));
    }

    #[test]
    fn parses_short_flag() {
        let command = Command::new("ritty").flag(Flag::new("verbose").short('v'));

        let matches = command.parse_from(["-v"]).unwrap();

        assert!(matches.flag("verbose"));
    }

    #[test]
    fn parses_positional_argument() {
        let command = Command::new("ritty").arg(Arg::new("name"));

        let matches = command.parse_from(["world"]).unwrap();

        assert_eq!(matches.argument("name"), Some("world"));
    }

    #[test]
    fn rejects_missing_required_argument() {
        let command = Command::new("ritty").arg(Arg::new("name").required());

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.message(), "missing required argument: name");
    }

    #[test]
    fn accepts_required_argument_when_supplied() {
        let command = Command::new("ritty").arg(Arg::new("name").required());

        let matches = command.parse_from(["world"]).unwrap();

        assert_eq!(matches.argument("name"), Some("world"));
    }

    #[test]
    fn parses_multiple_positional_arguments_in_order() {
        let command = Command::new("ritty")
            .arg(Arg::new("first").required())
            .arg(Arg::new("second").required());

        let matches = command.parse_from(["one", "two"]).unwrap();

        assert_eq!(matches.argument("first"), Some("one"));
        assert_eq!(matches.argument("second"), Some("two"));
    }

    #[test]
    fn rejects_missing_later_required_argument() {
        let command = Command::new("ritty")
            .arg(Arg::new("first"))
            .arg(Arg::new("second").required());

        let error = command.parse_from(["one"]).unwrap_err();

        assert_eq!(error.message(), "missing required argument: second");
    }

    #[test]
    fn flag_does_not_satisfy_required_argument() {
        let command = Command::new("ritty")
            .arg(Arg::new("name").required())
            .flag(Flag::new("verbose"));

        let error = command.parse_from(["--verbose"]).unwrap_err();

        assert_eq!(error.message(), "missing required argument: name");
    }

    #[test]
    fn subcommand_does_not_satisfy_required_argument() {
        let command = Command::new("ritty")
            .arg(Arg::new("name").required())
            .command(Command::new("build"));

        let error = command.parse_from(["build"]).unwrap_err();

        assert_eq!(error.message(), "missing required argument: name");
    }

    #[test]
    fn missing_optional_positional_uses_default() {
        let command = Command::new("ritty").arg(Arg::new("name").default("world"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.argument("name"), Some("world"));
    }

    #[test]
    fn explicit_input_overrides_default() {
        let command = Command::new("ritty").arg(Arg::new("name").default("world"));

        let matches = command.parse_from(["alice"]).unwrap();

        assert_eq!(matches.argument("name"), Some("alice"));
    }

    #[test]
    fn multiple_defaults_apply_independently() {
        let command = Command::new("ritty")
            .arg(Arg::new("first").default("a"))
            .arg(Arg::new("second").default("b"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.argument("first"), Some("a"));
        assert_eq!(matches.argument("second"), Some("b"));
    }

    #[test]
    fn mixed_explicit_and_default_values_bind_in_order() {
        let command = Command::new("ritty")
            .arg(Arg::new("first").default("a"))
            .arg(Arg::new("second").default("b"));

        let matches = command.parse_from(["x"]).unwrap();

        assert_eq!(matches.argument("first"), Some("x"));
        assert_eq!(matches.argument("second"), Some("b"));
    }

    #[test]
    fn required_argument_with_default_is_satisfied_when_omitted() {
        let command = Command::new("ritty").arg(Arg::new("name").required().default("world"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.argument("name"), Some("world"));
    }

    #[test]
    fn flag_does_not_suppress_positional_default() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose"))
            .arg(Arg::new("name").default("world"));

        let matches = command.parse_from(["--verbose"]).unwrap();

        assert!(matches.flag("verbose"));
        assert_eq!(matches.argument("name"), Some("world"));
    }

    #[test]
    fn subcommand_does_not_suppress_positional_default() {
        let command = Command::new("ritty")
            .command(Command::new("build"))
            .arg(Arg::new("name").default("world"));

        let matches = command.parse_from(["build"]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
        assert_eq!(matches.argument("name"), Some("world"));
    }

    #[test]
    fn argument_metadata_defaults_to_none() {
        let arg = Arg::new("name");

        assert_eq!(arg.get_description(), None);
        assert_eq!(arg.get_value_hint(), None);
    }

    #[test]
    fn configures_argument_metadata() {
        let arg = Arg::new("output")
            .description("Output directory")
            .value_hint("dir")
            .required()
            .default(".");

        assert_eq!(arg.name(), "output");
        assert_eq!(arg.get_description(), Some("Output directory"));
        assert_eq!(arg.get_value_hint(), Some("dir"));
        assert!(arg.is_required());
        assert_eq!(arg.default_value(), Some("."));
    }

    #[test]
    fn argument_metadata_does_not_affect_parsing() {
        let command = Command::new("ritty").arg(
            Arg::new("output")
                .description("Output directory")
                .value_hint("dir"),
        );

        let matches = command.parse_from(["build"]).unwrap();

        assert_eq!(matches.argument("output"), Some("build"));
    }

    #[test]
    fn parses_subcommand() {
        let command = Command::new("ritty").command(Command::new("build"));

        let matches = command.parse_from(["build"]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn adds_string_option() {
        let command = Command::new("ritty").option(StringOption::new("name"));

        assert_eq!(command.options().len(), 1);
        assert_eq!(command.options()[0].name(), "name");
    }

    #[test]
    fn parses_string_option_separate_token() {
        let command = Command::new("ritty").option(StringOption::new("name"));

        let matches = command.parse_from(["--name", "alice"]).unwrap();

        assert_eq!(matches.option("name"), Some("alice"));
    }

    #[test]
    fn parses_string_option_equals_syntax() {
        let command = Command::new("ritty").option(StringOption::new("name"));

        let matches = command.parse_from(["--name=alice"]).unwrap();

        assert_eq!(matches.option("name"), Some("alice"));
    }

    #[test]
    fn preserves_exact_string_option_value() {
        let command = Command::new("ritty").option(StringOption::new("name"));

        let matches = command.parse_from(["--name", "Alice-Smith"]).unwrap();

        assert_eq!(matches.option("name"), Some("Alice-Smith"));
    }

    #[test]
    fn rejects_missing_string_option_value() {
        let command = Command::new("ritty").option(StringOption::new("name"));

        let error = command.parse_from(["--name"]).unwrap_err();

        assert_eq!(error.message(), "missing value for option: --name");
    }

    #[test]
    fn string_option_consumes_hyphen_prefixed_value() {
        let command = Command::new("ritty").option(StringOption::new("pattern"));

        let matches = command.parse_from(["--pattern", "-foo"]).unwrap();

        assert_eq!(matches.option("pattern"), Some("-foo"));
    }

    #[test]
    fn string_option_consumes_double_hyphen_prefixed_value() {
        let command = Command::new("ritty").option(StringOption::new("pattern"));

        let matches = command.parse_from(["--pattern", "--literal"]).unwrap();

        assert_eq!(matches.option("pattern"), Some("--literal"));
    }

    #[test]
    fn string_option_value_is_not_mistaken_for_subcommand() {
        let command = Command::new("ritty")
            .option(StringOption::new("target"))
            .command(Command::new("build"));

        let matches = command.parse_from(["--target", "build"]).unwrap();

        assert_eq!(matches.option("target"), Some("build"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn subcommand_resolves_after_string_option_value() {
        let command = Command::new("ritty")
            .option(StringOption::new("target"))
            .command(Command::new("build"));

        let matches = command
            .parse_from(["--target", "release", "build"])
            .unwrap();

        assert_eq!(matches.option("target"), Some("release"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn string_option_and_boolean_flag_coexist() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose"))
            .option(StringOption::new("name"));

        let matches = command
            .parse_from(["--verbose", "--name", "alice"])
            .unwrap();

        assert!(matches.flag("verbose"));
        assert_eq!(matches.option("name"), Some("alice"));
    }

    #[test]
    fn string_option_value_does_not_advance_positional_cursor() {
        let command = Command::new("ritty")
            .option(StringOption::new("name"))
            .arg(Arg::new("target"));

        let matches = command.parse_from(["--name", "alice", "world"]).unwrap();

        assert_eq!(matches.option("name"), Some("alice"));
        assert_eq!(matches.argument("target"), Some("world"));
    }

    #[test]
    fn rejects_unknown_long_option() {
        let command = Command::new("ritty");

        let error = command.parse_from(["--wat"]).unwrap_err();

        assert_eq!(error.message(), "unknown flag: --wat");
    }

    #[test]
    fn string_option_metadata_defaults_to_none() {
        let option = StringOption::new("output");

        assert!(!option.is_required());
        assert_eq!(option.default_value(), None);
        assert_eq!(option.get_description(), None);
        assert_eq!(option.get_value_hint(), None);
    }

    #[test]
    fn configures_string_option_metadata() {
        let option = StringOption::new("output")
            .description("Output directory")
            .value_hint("dir")
            .required()
            .default(".");

        assert_eq!(option.name(), "output");
        assert_eq!(option.get_description(), Some("Output directory"));
        assert_eq!(option.get_value_hint(), Some("dir"));
        assert!(option.is_required());
        assert_eq!(option.default_value(), Some("."));
    }

    #[test]
    fn missing_optional_string_option_remains_absent() {
        let command = Command::new("ritty").option(StringOption::new("name"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.option("name"), None);
    }

    #[test]
    fn missing_string_option_uses_default() {
        let command = Command::new("ritty").option(StringOption::new("name").default("world"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.option("name"), Some("world"));
    }

    #[test]
    fn explicit_string_option_value_overrides_default() {
        let command = Command::new("ritty").option(StringOption::new("name").default("world"));

        let separate = command.parse_from(["--name", "alice"]).unwrap();
        let equals = command.parse_from(["--name=alice"]).unwrap();

        assert_eq!(separate.option("name"), Some("alice"));
        assert_eq!(equals.option("name"), Some("alice"));
    }

    #[test]
    fn rejects_missing_required_string_option() {
        let command = Command::new("ritty").option(StringOption::new("name").required());

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.message(), "missing required option: --name");
    }

    #[test]
    fn accepts_required_string_option_when_supplied() {
        let command = Command::new("ritty").option(StringOption::new("name").required());

        let matches = command.parse_from(["--name", "alice"]).unwrap();

        assert_eq!(matches.option("name"), Some("alice"));
    }

    #[test]
    fn required_string_option_with_default_is_satisfied_when_omitted() {
        let command =
            Command::new("ritty").option(StringOption::new("name").required().default("world"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.option("name"), Some("world"));
    }

    #[test]
    fn multiple_string_option_defaults_apply_independently() {
        let command = Command::new("ritty")
            .option(StringOption::new("first").default("a"))
            .option(StringOption::new("second").default("b"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.option("first"), Some("a"));
        assert_eq!(matches.option("second"), Some("b"));
    }

    #[test]
    fn mixed_explicit_and_default_string_options() {
        let command = Command::new("ritty")
            .option(StringOption::new("first").default("a"))
            .option(StringOption::new("second").default("b"));

        let matches = command.parse_from(["--first", "x"]).unwrap();

        assert_eq!(matches.option("first"), Some("x"));
        assert_eq!(matches.option("second"), Some("b"));
    }

    #[test]
    fn hyphen_prefixed_explicit_value_overrides_string_option_default() {
        let command = Command::new("ritty").option(StringOption::new("pattern").default("default"));

        let matches = command.parse_from(["--pattern", "--literal"]).unwrap();

        assert_eq!(matches.option("pattern"), Some("--literal"));
    }

    #[test]
    fn flag_does_not_satisfy_required_string_option() {
        let command = Command::new("ritty")
            .option(StringOption::new("name").required())
            .flag(Flag::new("verbose"));

        let error = command.parse_from(["--verbose"]).unwrap_err();

        assert_eq!(error.message(), "missing required option: --name");
    }

    #[test]
    fn subcommand_does_not_satisfy_required_string_option() {
        let command = Command::new("ritty")
            .option(StringOption::new("name").required())
            .command(Command::new("build"));

        let error = command.parse_from(["build"]).unwrap_err();

        assert_eq!(error.message(), "missing required option: --name");
    }

    #[test]
    fn string_option_default_does_not_affect_positional_state() {
        let command = Command::new("ritty")
            .option(StringOption::new("name").default("world"))
            .arg(Arg::new("target").required());

        let matches = command.parse_from(["value"]).unwrap();

        assert_eq!(matches.option("name"), Some("world"));
        assert_eq!(matches.argument("target"), Some("value"));
    }

    #[test]
    fn canonical_string_option_repeated_uses_first_occurrence() {
        // Established parser behavior: `Matches::option` looks up the
        // first stored occurrence, so the first explicit value wins.
        let command = Command::new("ritty").option(StringOption::new("output"));

        let matches = command
            .parse_from(["--output", "first", "--output", "second"])
            .unwrap();

        assert_eq!(matches.option("output"), Some("first"));
    }

    #[test]
    fn string_option_aliases_default_to_empty() {
        let option = StringOption::new("output");

        assert!(option.aliases().is_empty());
    }

    #[test]
    fn string_option_retains_aliases_in_order() {
        let option = StringOption::new("output").alias("o").alias("out");

        assert_eq!(option.aliases(), ["o", "out"]);
    }

    #[test]
    fn parses_short_string_option_alias() {
        let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

        let matches = command.parse_from(["-o", "dist"]).unwrap();

        assert_eq!(matches.option("output"), Some("dist"));
    }

    #[test]
    fn parses_short_string_option_alias_with_equals() {
        let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

        let matches = command.parse_from(["-o=dist"]).unwrap();

        assert_eq!(matches.option("output"), Some("dist"));
    }

    #[test]
    fn unicode_scalar_string_option_alias_works_separate_and_equals() {
        let command = Command::new("ritty").option(StringOption::new("output").alias("é"));

        let separate = command.parse_from(["-é", "dist"]).unwrap();
        let equals = command.parse_from(["-é=dist"]).unwrap();

        assert_eq!(separate.option("output"), Some("dist"));
        assert_eq!(equals.option("output"), Some("dist"));
    }

    #[test]
    fn multi_scalar_unicode_string_option_alias_is_not_a_short_option() {
        let command = Command::new("ritty").option(StringOption::new("output").alias("日本"));

        assert!(command.parse_from(["-日", "dist"]).is_err());
        assert!(command.parse_from(["-日=dist"]).is_err());
    }

    #[test]
    fn short_string_option_alias_equals_value_preserves_extra_equals() {
        let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

        let matches = command.parse_from(["-o=a=b"]).unwrap();

        assert_eq!(matches.option("output"), Some("a=b"));
    }

    #[test]
    fn short_string_option_alias_equals_empty_value_is_explicit() {
        let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

        let matches = command.parse_from(["-o="]).unwrap();

        assert_eq!(matches.option("output"), Some(""));
    }

    #[test]
    fn canonical_long_option_equals_empty_value_is_explicit() {
        let command = Command::new("ritty").option(StringOption::new("name"));

        let matches = command.parse_from(["--name="]).unwrap();

        assert_eq!(matches.option("name"), Some(""));
    }

    #[test]
    fn short_string_option_alias_consumes_hyphen_prefixed_value() {
        let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

        let matches = command.parse_from(["-o", "--literal"]).unwrap();

        assert_eq!(matches.option("output"), Some("--literal"));
    }

    #[test]
    fn parses_long_string_option_alias() {
        let command =
            Command::new("ritty").option(StringOption::new("output").alias("destination"));

        let matches = command.parse_from(["--destination", "dist"]).unwrap();

        assert_eq!(matches.option("output"), Some("dist"));
    }

    #[test]
    fn parses_long_string_option_alias_with_equals() {
        let command =
            Command::new("ritty").option(StringOption::new("output").alias("destination"));

        let matches = command.parse_from(["--destination=dist"]).unwrap();

        assert_eq!(matches.option("output"), Some("dist"));
    }

    #[test]
    fn all_alias_spellings_resolve_to_canonical_name() {
        let command = Command::new("ritty")
            .option(StringOption::new("output").alias("o").alias("destination"));

        for args in [
            &["--output", "dist"][..],
            &["-o", "dist"][..],
            &["-o=dist"][..],
            &["--destination", "dist"][..],
            &["--destination=dist"][..],
        ] {
            let matches = command.parse_from(args.to_vec()).unwrap();
            assert_eq!(matches.option("output"), Some("dist"));
        }
    }

    #[test]
    fn short_string_option_alias_overrides_default() {
        let command =
            Command::new("ritty").option(StringOption::new("output").alias("o").default("default"));

        let matches = command.parse_from(["-o", "explicit"]).unwrap();

        assert_eq!(matches.option("output"), Some("explicit"));
    }

    #[test]
    fn short_string_option_alias_equals_overrides_default() {
        let command =
            Command::new("ritty").option(StringOption::new("output").alias("o").default("default"));

        let matches = command.parse_from(["-o=explicit"]).unwrap();

        assert_eq!(matches.option("output"), Some("explicit"));
    }

    #[test]
    fn required_string_option_satisfied_through_short_alias() {
        let command =
            Command::new("ritty").option(StringOption::new("output").required().alias("o"));

        let matches = command.parse_from(["-o", "dist"]).unwrap();

        assert_eq!(matches.option("output"), Some("dist"));
    }

    #[test]
    fn string_option_alias_value_is_not_mistaken_for_subcommand() {
        let command = Command::new("ritty")
            .option(StringOption::new("target").alias("t"))
            .command(Command::new("build"));

        let matches = command.parse_from(["-t", "build"]).unwrap();

        assert_eq!(matches.option("target"), Some("build"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn subcommand_resolves_after_string_option_alias_value() {
        let command = Command::new("ritty")
            .option(StringOption::new("target").alias("t"))
            .command(Command::new("build"));

        let matches = command.parse_from(["-t", "release", "build"]).unwrap();

        assert_eq!(matches.option("target"), Some("release"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn string_option_alias_value_does_not_advance_positional_cursor() {
        let command = Command::new("ritty")
            .option(StringOption::new("output").alias("o"))
            .arg(Arg::new("target"));

        let matches = command.parse_from(["-o", "dist", "world"]).unwrap();

        assert_eq!(matches.option("output"), Some("dist"));
        assert_eq!(matches.argument("target"), Some("world"));
    }

    #[test]
    fn rejects_unknown_short_string_option_alias() {
        let command = Command::new("ritty");

        let error = command.parse_from(["-x"]).unwrap_err();

        assert_eq!(error.message(), "unknown flag: -x");
    }

    #[test]
    fn rejects_unknown_long_string_option_alias() {
        let command = Command::new("ritty");

        let error = command.parse_from(["--destination"]).unwrap_err();

        assert_eq!(error.message(), "unknown flag: --destination");
    }

    #[test]
    fn boolean_short_flag_still_works_alongside_string_option_aliases() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose").short('v'))
            .option(StringOption::new("output").alias("o"));

        let matches = command.parse_from(["-v", "-o", "dist"]).unwrap();

        assert!(matches.flag("verbose"));
        assert_eq!(matches.option("output"), Some("dist"));
    }

    #[test]
    fn boolean_short_flag_and_string_option_alias_collision_errors() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose").short('v'))
            .option(StringOption::new("value").alias("v"));

        let error = command.parse_from(["-v"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: -v");
    }

    #[test]
    fn duplicate_string_option_alias_collision_errors() {
        let command = Command::new("ritty")
            .option(StringOption::new("first").alias("x"))
            .option(StringOption::new("second").alias("x"));

        let error = command.parse_from(["-x", "value"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: -x");
    }

    #[test]
    fn flag_metadata_defaults_to_none() {
        let flag = Flag::new("color");

        assert!(flag.aliases().is_empty());
        assert_eq!(flag.get_description(), None);
        assert_eq!(flag.get_negative_description(), None);
        assert_eq!(flag.get_value_hint(), None);
        assert!(!flag.is_required());
        assert_eq!(flag.default_value(), None);
    }

    #[test]
    fn configures_flag_metadata() {
        let flag = Flag::new("color")
            .short('c')
            .alias("colour")
            .alias("colors")
            .description("Enable color output")
            .negative_description("Disable color output")
            .value_hint("bool")
            .required()
            .default(true);

        assert_eq!(flag.name(), "color");
        assert_eq!(flag.short_name(), Some('c'));
        assert_eq!(flag.aliases(), ["colour", "colors"]);
        assert_eq!(flag.get_description(), Some("Enable color output"));
        assert_eq!(
            flag.get_negative_description(),
            Some("Disable color output")
        );
        assert_eq!(flag.get_value_hint(), Some("bool"));
        assert!(flag.is_required());
        assert_eq!(flag.default_value(), Some(true));
    }

    #[test]
    fn dedicated_short_still_works_alongside_aliases() {
        let command = Command::new("ritty").flag(Flag::new("verbose").short('v').alias("chatty"));

        let matches = command.parse_from(["-v"]).unwrap();

        assert!(matches.flag("verbose"));
    }

    #[test]
    fn parses_long_flag_alias() {
        let command = Command::new("ritty").flag(Flag::new("verbose").alias("chatty"));

        let matches = command.parse_from(["--chatty"]).unwrap();

        assert!(matches.flag("verbose"));
    }

    #[test]
    fn single_char_flag_alias_works_as_short_and_long() {
        let command = Command::new("ritty").flag(Flag::new("verbose").alias("q"));

        let short = command.parse_from(["-q"]).unwrap();
        let long = command.parse_from(["--q"]).unwrap();

        assert!(short.flag("verbose"));
        assert!(long.flag("verbose"));
    }

    #[test]
    fn dedicated_short_accepts_multi_byte_unicode_scalar() {
        let command = Command::new("ritty").flag(Flag::new("verbose").short('é'));

        let matches = command.parse_from(["-é"]).unwrap();

        assert!(matches.flag("verbose"));
    }

    #[test]
    fn dedicated_short_multi_byte_unicode_matches_usage_spelling() {
        let command = Command::new("ritty").flag(Flag::new("verbose").short('é'));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -é, --verbose"
        );
    }

    #[test]
    fn unicode_scalar_flag_alias_works_as_short() {
        let command = Command::new("ritty").flag(Flag::new("verbose").alias("é"));

        let matches = command.parse_from(["-é"]).unwrap();

        assert!(matches.flag("verbose"));
    }

    #[test]
    fn multi_scalar_unicode_alias_is_not_a_short_option() {
        let command = Command::new("ritty").flag(Flag::new("verbose").alias("日本"));

        let result = command.parse_from(["-日"]);

        assert!(result.is_err());
    }

    #[test]
    fn flag_default_true_applies_when_absent() {
        let command = Command::new("ritty").flag(Flag::new("color").default(true));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert!(matches.flag("color"));
    }

    #[test]
    fn flag_default_false_applies_when_absent() {
        let command = Command::new("ritty").flag(Flag::new("color").default(false));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert!(!matches.flag("color"));
    }

    #[test]
    fn explicit_positive_overrides_false_default() {
        let command = Command::new("ritty").flag(Flag::new("color").default(false));

        let matches = command.parse_from(["--color"]).unwrap();

        assert!(matches.flag("color"));
    }

    #[test]
    fn negation_overrides_true_default() {
        let command = Command::new("ritty").flag(Flag::new("color").default(true));

        let matches = command.parse_from(["--no-color"]).unwrap();

        assert!(!matches.flag("color"));
    }

    #[test]
    fn parses_canonical_negation() {
        let command = Command::new("ritty").flag(Flag::new("color"));

        let matches = command.parse_from(["--no-color"]).unwrap();

        assert!(!matches.flag("color"));
    }

    #[test]
    fn parses_long_alias_negation() {
        let command = Command::new("ritty").flag(Flag::new("color").alias("colour"));

        let matches = command.parse_from(["--no-colour"]).unwrap();

        assert!(!matches.flag("color"));
    }

    #[test]
    fn parses_single_char_alias_negation() {
        let command = Command::new("ritty").flag(Flag::new("color").alias("c"));

        let matches = command.parse_from(["--no-c"]).unwrap();

        assert!(!matches.flag("color"));
    }

    #[test]
    fn dedicated_short_does_not_support_negation() {
        let command = Command::new("ritty").flag(Flag::new("color").short('c'));

        let error = command.parse_from(["--no-c"]).unwrap_err();

        assert_eq!(error.message(), "unknown flag: --no-c");
    }

    #[test]
    fn rejects_missing_required_flag() {
        let command = Command::new("ritty").flag(Flag::new("confirm").required());

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.message(), "missing required option: --confirm");
    }

    #[test]
    fn required_flag_satisfied_by_positive() {
        let command = Command::new("ritty").flag(Flag::new("confirm").required());

        let matches = command.parse_from(["--confirm"]).unwrap();

        assert!(matches.flag("confirm"));
    }

    #[test]
    fn required_flag_satisfied_by_negation() {
        let command = Command::new("ritty").flag(Flag::new("confirm").required());

        let matches = command.parse_from(["--no-confirm"]).unwrap();

        assert!(!matches.flag("confirm"));
    }

    #[test]
    fn required_flag_satisfied_by_default() {
        let command = Command::new("ritty").flag(Flag::new("confirm").required().default(false));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert!(!matches.flag("confirm"));
    }

    #[test]
    fn repeated_positive_then_negative_yields_negative() {
        let command = Command::new("ritty").flag(Flag::new("verbose"));

        let matches = command.parse_from(["--verbose", "--no-verbose"]).unwrap();

        assert!(!matches.flag("verbose"));
    }

    #[test]
    fn repeated_negative_then_positive_yields_positive() {
        let command = Command::new("ritty").flag(Flag::new("verbose"));

        let matches = command.parse_from(["--no-verbose", "--verbose"]).unwrap();

        assert!(matches.flag("verbose"));
    }

    #[test]
    fn rejects_unknown_negation() {
        let command = Command::new("ritty").flag(Flag::new("color"));

        let error = command.parse_from(["--no-verbose"]).unwrap_err();

        assert_eq!(error.message(), "unknown flag: --no-verbose");
    }

    #[test]
    fn two_boolean_long_aliases_colliding_errors() {
        let command = Command::new("ritty")
            .flag(Flag::new("first").alias("x"))
            .flag(Flag::new("second").alias("x"));

        let error = command.parse_from(["--x"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: --x");
    }

    #[test]
    fn two_boolean_short_spellings_colliding_errors() {
        let command = Command::new("ritty")
            .flag(Flag::new("first").short('x'))
            .flag(Flag::new("second").short('x'));

        let error = command.parse_from(["-x"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: -x");
    }

    #[test]
    fn boolean_long_and_string_long_alias_collision_errors() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose").alias("mode"))
            .option(StringOption::new("output").alias("mode"));

        let error = command.parse_from(["--mode"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: --mode");
    }

    #[test]
    fn flag_named_no_cache_parses_as_exact_positive() {
        let command = Command::new("ritty").flag(Flag::new("no-cache"));

        let matches = command.parse_from(["--no-cache"]).unwrap();

        assert!(matches.flag("no-cache"));
    }

    #[test]
    fn no_cache_ambiguous_between_positive_and_negation() {
        let command = Command::new("ritty")
            .flag(Flag::new("cache"))
            .flag(Flag::new("no-cache"));

        let error = command.parse_from(["--no-cache"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: --no-cache");
    }

    #[test]
    fn boolean_negation_and_string_option_exact_collision_errors() {
        let command = Command::new("ritty")
            .flag(Flag::new("cache"))
            .option(StringOption::new("no-cache"));

        let error = command.parse_from(["--no-cache"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: --no-cache");
    }

    #[test]
    fn exact_string_option_no_cache_works_without_boolean_collision() {
        let command = Command::new("ritty").option(StringOption::new("no-cache"));

        let matches = command.parse_from(["--no-cache", "value"]).unwrap();

        assert_eq!(matches.option("no-cache"), Some("value"));
    }

    #[test]
    fn boolean_positive_followed_by_subcommand() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose"))
            .command(Command::new("build"));

        let matches = command.parse_from(["--verbose", "build"]).unwrap();

        assert!(matches.flag("verbose"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn boolean_negative_followed_by_subcommand() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose"))
            .command(Command::new("build"));

        let matches = command.parse_from(["--no-verbose", "build"]).unwrap();

        assert!(!matches.flag("verbose"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn boolean_negation_does_not_advance_positional_cursor() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose"))
            .arg(Arg::new("target"));

        let matches = command.parse_from(["--no-verbose", "world"]).unwrap();

        assert!(!matches.flag("verbose"));
        assert_eq!(matches.argument("target"), Some("world"));
    }

    #[test]
    fn flag_metadata_does_not_affect_parsing() {
        let command = Command::new("ritty").flag(
            Flag::new("color")
                .description("Enable colors")
                .negative_description("Disable colors")
                .value_hint("bool"),
        );

        let matches = command.parse_from(["--no-color"]).unwrap();

        assert!(!matches.flag("color"));
    }

    #[test]
    fn adds_enum_option_and_retains_allowed_values_in_order() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info", "warn", "error"]));

        assert_eq!(command.enum_options().len(), 1);
        assert_eq!(command.enum_options()[0].name(), "level");
        assert_eq!(
            command.enum_options()[0].values(),
            ["debug", "info", "warn", "error"]
        );
    }

    #[test]
    fn enum_option_metadata_defaults_to_none() {
        let option = EnumOption::new("level", ["debug", "info"]);

        assert!(option.aliases().is_empty());
        assert_eq!(option.get_description(), None);
        assert_eq!(option.get_value_hint(), None);
        assert!(!option.is_required());
        assert_eq!(option.default_value(), None);
    }

    #[test]
    fn configures_enum_option_metadata() {
        let option = EnumOption::new("level", ["debug", "info", "warn", "error"])
            .alias("l")
            .alias("log-level")
            .description("Logging level")
            .value_hint("level")
            .required()
            .default("info");

        assert_eq!(option.name(), "level");
        assert_eq!(option.aliases(), ["l", "log-level"]);
        assert_eq!(option.get_description(), Some("Logging level"));
        assert_eq!(option.get_value_hint(), Some("level"));
        assert!(option.is_required());
        assert_eq!(option.default_value(), Some("info"));
    }

    #[test]
    fn parses_enum_option_separate_token_and_equals() {
        let command =
            Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

        let separate = command.parse_from(["--level", "info"]).unwrap();
        let equals = command.parse_from(["--level=info"]).unwrap();

        assert_eq!(separate.enum_option("level"), Some("info"));
        assert_eq!(equals.enum_option("level"), Some("info"));
    }

    #[test]
    fn rejects_invalid_enum_value() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info", "warn", "error"]));

        let error = command.parse_from(["--level", "verbose"]).unwrap_err();

        assert_eq!(
            error.message(),
            "invalid value for option: --level: verbose (expected one of: debug, info, warn, error)"
        );
    }

    #[test]
    fn enum_validation_is_case_sensitive() {
        let command = Command::new("ritty").enum_option(EnumOption::new("level", ["info"]));

        let error = command.parse_from(["--level", "INFO"]).unwrap_err();

        assert_eq!(
            error.message(),
            "invalid value for option: --level: INFO (expected one of: info)"
        );
    }

    #[test]
    fn empty_allowed_value_list_accepts_anything() {
        let command = Command::new("ritty").enum_option(EnumOption::new("level", [] as [&str; 0]));

        let matches = command.parse_from(["--level", "anything"]).unwrap();

        assert_eq!(matches.enum_option("level"), Some("anything"));
    }

    #[test]
    fn missing_optional_enum_option_remains_absent() {
        let command =
            Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.enum_option("level"), None);
    }

    #[test]
    fn missing_enum_option_uses_default() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).default("info"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.enum_option("level"), Some("info"));
    }

    #[test]
    fn explicit_enum_value_overrides_default() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).default("info"));

        let separate = command.parse_from(["--level", "debug"]).unwrap();

        assert_eq!(separate.enum_option("level"), Some("debug"));
    }

    #[test]
    fn explicit_equals_enum_value_overrides_default() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).default("info"));

        let matches = command.parse_from(["--level=debug"]).unwrap();

        assert_eq!(matches.enum_option("level"), Some("debug"));
    }

    #[test]
    fn invalid_effective_enum_default_errors() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).default("verbose"));

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(
            error.message(),
            "invalid value for option: --level: verbose (expected one of: debug, info)"
        );
    }

    #[test]
    fn valid_explicit_enum_value_beats_invalid_unused_default() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).default("verbose"));

        let matches = command.parse_from(["--level", "info"]).unwrap();

        assert_eq!(matches.enum_option("level"), Some("info"));
    }

    #[test]
    fn rejects_missing_required_enum_option() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).required());

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.message(), "missing required option: --level");
    }

    #[test]
    fn required_enum_option_satisfied_by_separate_explicit() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).required());

        let matches = command.parse_from(["--level", "info"]).unwrap();

        assert_eq!(matches.enum_option("level"), Some("info"));
    }

    #[test]
    fn required_enum_option_satisfied_by_equals_explicit() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).required());

        let matches = command.parse_from(["--level=info"]).unwrap();

        assert_eq!(matches.enum_option("level"), Some("info"));
    }

    #[test]
    fn required_enum_option_satisfied_by_valid_default() {
        let command = Command::new("ritty").enum_option(
            EnumOption::new("level", ["debug", "info"])
                .required()
                .default("info"),
        );

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.enum_option("level"), Some("info"));
    }

    #[test]
    fn required_enum_option_with_invalid_default_errors() {
        let command = Command::new("ritty").enum_option(
            EnumOption::new("level", ["debug", "info"])
                .required()
                .default("verbose"),
        );

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(
            error.message(),
            "invalid value for option: --level: verbose (expected one of: debug, info)"
        );
    }

    #[test]
    fn parses_short_enum_option_alias_separate_and_equals() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).alias("l"));

        let separate = command.parse_from(["-l", "info"]).unwrap();
        let equals = command.parse_from(["-l=info"]).unwrap();

        assert_eq!(separate.enum_option("level"), Some("info"));
        assert_eq!(equals.enum_option("level"), Some("info"));
    }

    #[test]
    fn unicode_scalar_enum_option_alias_works_separate_and_equals() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).alias("é"));

        let separate = command.parse_from(["-é", "info"]).unwrap();
        let equals = command.parse_from(["-é=info"]).unwrap();

        assert_eq!(separate.enum_option("level"), Some("info"));
        assert_eq!(equals.enum_option("level"), Some("info"));
    }

    #[test]
    fn unicode_scalar_enum_option_alias_still_validates_value() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).alias("é"));

        let error = command.parse_from(["-é", "verbose"]).unwrap_err();

        assert_eq!(
            error.message(),
            "invalid value for option: --level: verbose (expected one of: debug, info)"
        );
    }

    #[test]
    fn parses_long_enum_option_alias_separate_and_equals() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).alias("log-level"));

        let separate = command.parse_from(["--log-level", "info"]).unwrap();
        let equals = command.parse_from(["--log-level=info"]).unwrap();

        assert_eq!(separate.enum_option("level"), Some("info"));
        assert_eq!(equals.enum_option("level"), Some("info"));
    }

    #[test]
    fn all_enum_alias_spellings_resolve_to_canonical_name() {
        let command = Command::new("ritty").enum_option(
            EnumOption::new("level", ["debug", "info"])
                .alias("l")
                .alias("log-level"),
        );

        for args in [
            &["--level", "info"][..],
            &["-l", "info"][..],
            &["-l=info"][..],
            &["--log-level", "info"][..],
            &["--log-level=info"][..],
        ] {
            let matches = command.parse_from(args.to_vec()).unwrap();
            assert_eq!(matches.enum_option("level"), Some("info"));
        }
    }

    #[test]
    fn enum_alias_value_overrides_default() {
        let command = Command::new("ritty").enum_option(
            EnumOption::new("level", ["debug", "info"])
                .alias("l")
                .default("info"),
        );

        let matches = command.parse_from(["-l", "debug"]).unwrap();

        assert_eq!(matches.enum_option("level"), Some("debug"));
    }

    #[test]
    fn enum_option_consumes_hyphen_prefixed_allowed_value() {
        let command =
            Command::new("ritty").enum_option(EnumOption::new("mode", ["-fast", "--safe"]));

        let fast = command.parse_from(["--mode", "-fast"]).unwrap();
        let safe = command.parse_from(["--mode", "--safe"]).unwrap();

        assert_eq!(fast.enum_option("mode"), Some("-fast"));
        assert_eq!(safe.enum_option("mode"), Some("--safe"));
    }

    #[test]
    fn enum_value_matching_subcommand_is_not_mistaken_for_subcommand() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("target", ["build", "test"]))
            .command(Command::new("build"));

        let matches = command.parse_from(["--target", "build"]).unwrap();

        assert_eq!(matches.enum_option("target"), Some("build"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn subcommand_resolves_after_consumed_enum_value() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("target", ["build", "test"]))
            .command(Command::new("build"));

        let matches = command.parse_from(["--target", "test", "build"]).unwrap();

        assert_eq!(matches.enum_option("target"), Some("test"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn enum_option_value_does_not_advance_positional_cursor() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]))
            .arg(Arg::new("target"));

        let matches = command.parse_from(["--level", "info", "world"]).unwrap();

        assert_eq!(matches.enum_option("level"), Some("info"));
        assert_eq!(matches.argument("target"), Some("world"));
    }

    #[test]
    fn canonical_enum_option_repeated_uses_first_occurrence() {
        let command =
            Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

        let matches = command
            .parse_from(["--level", "info", "--level", "debug"])
            .unwrap();

        assert_eq!(matches.enum_option("level"), Some("info"));
    }

    #[test]
    fn string_option_and_enum_option_long_collision_errors() {
        let command = Command::new("ritty")
            .option(StringOption::new("mode"))
            .enum_option(EnumOption::new("mode", ["fast", "safe"]));

        let error = command.parse_from(["--mode"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: --mode");
    }

    #[test]
    fn flag_and_enum_option_long_collision_errors() {
        let command = Command::new("ritty")
            .flag(Flag::new("mode"))
            .enum_option(EnumOption::new("mode", ["fast", "safe"]));

        let bare = command.parse_from(["--mode"]).unwrap_err();
        let equals = command.parse_from(["--mode=fast"]).unwrap_err();

        assert_eq!(bare.message(), "ambiguous option: --mode");
        assert_eq!(equals.message(), "ambiguous option: --mode");
    }

    #[test]
    fn boolean_negation_and_enum_option_no_prefix_collision_errors() {
        let command = Command::new("ritty")
            .flag(Flag::new("cache"))
            .enum_option(EnumOption::new("no-cache", ["on", "off"]));

        let error = command.parse_from(["--no-cache"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: --no-cache");
    }

    #[test]
    fn string_option_and_enum_option_short_alias_collision_errors() {
        let command = Command::new("ritty")
            .option(StringOption::new("output").alias("o"))
            .enum_option(EnumOption::new("format", ["json", "text"]).alias("o"));

        let error = command.parse_from(["-o", "x"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: -o");
    }

    #[test]
    fn boolean_and_enum_option_short_collision_errors() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose").short('v'))
            .enum_option(EnumOption::new("value", ["a", "b"]).alias("v"));

        let error = command.parse_from(["-v"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: -v");
    }

    #[test]
    fn multiple_enum_options_sharing_alias_collision_errors() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("first", ["a"]).alias("x"))
            .enum_option(EnumOption::new("second", ["a"]).alias("x"));

        let error = command.parse_from(["-x", "a"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous option: -x");
    }

    #[test]
    fn enum_option_equals_value_preserves_extra_equals() {
        let command =
            Command::new("ritty").enum_option(EnumOption::new("mode", ["a=b"]).alias("m"));

        let long = command.parse_from(["--mode=a=b"]).unwrap();
        let short = command.parse_from(["-m=a=b"]).unwrap();

        assert_eq!(long.enum_option("mode"), Some("a=b"));
        assert_eq!(short.enum_option("mode"), Some("a=b"));
    }

    #[test]
    fn enum_option_explicit_empty_value_is_validated_not_treated_as_absent() {
        let allowed = Command::new("ritty").enum_option(EnumOption::new("mode", ["", "a"]));
        let matches = allowed.parse_from(["--mode="]).unwrap();
        assert_eq!(matches.enum_option("mode"), Some(""));

        let disallowed = Command::new("ritty").enum_option(EnumOption::new("mode", ["a"]));
        let error = disallowed.parse_from(["--mode="]).unwrap_err();
        assert_eq!(
            error.message(),
            "invalid value for option: --mode:  (expected one of: a)"
        );
    }

    // --- Subcommand aliases and recursive parsing ---

    #[test]
    fn command_aliases_default_to_empty() {
        let command = Command::new("build");

        assert!(command.aliases().is_empty());
    }

    #[test]
    fn command_retains_aliases_in_order() {
        let command = Command::new("install").alias("i").alias("add");

        assert_eq!(command.aliases(), ["i", "add"]);
    }

    #[test]
    fn subcommand_alias_canonicalizes_to_name() {
        let command = Command::new("root").command(Command::new("install").alias("i").alias("add"));

        for token in ["install", "i", "add"] {
            let matches = command.parse_from([token]).unwrap();
            assert_eq!(matches.subcommand(), Some("install"));
        }
    }

    #[test]
    fn duplicate_alias_on_one_command_does_not_self_collide() {
        let command = Command::new("root").command(Command::new("build").alias("b").alias("b"));

        let matches = command.parse_from(["b"]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn two_child_aliases_colliding_errors() {
        let command = Command::new("root")
            .command(Command::new("install").alias("x"))
            .command(Command::new("inspect").alias("x"));

        let error = command.parse_from(["x"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous command: x");
    }

    #[test]
    fn canonical_name_vs_sibling_alias_collision_errors() {
        let command = Command::new("root")
            .command(Command::new("build"))
            .command(Command::new("deploy").alias("build"));

        let error = command.parse_from(["build"]).unwrap_err();

        assert_eq!(error.message(), "ambiguous command: build");
    }

    #[test]
    fn parent_string_option_before_subcommand() {
        let command = Command::new("root")
            .option(StringOption::new("profile"))
            .command(Command::new("build").option(StringOption::new("target")));

        let matches = command
            .parse_from(["--profile", "release", "build", "--target", "wasm"])
            .unwrap();

        assert_eq!(matches.option("profile"), Some("release"));
        assert_eq!(matches.subcommand(), Some("build"));
        assert_eq!(
            matches.subcommand_matches().unwrap().option("target"),
            Some("wasm")
        );
    }

    #[test]
    fn parent_string_short_alias_before_subcommand() {
        let command = Command::new("root")
            .option(StringOption::new("profile").alias("p"))
            .command(Command::new("build"));

        let matches = command.parse_from(["-p", "release", "build"]).unwrap();

        assert_eq!(matches.option("profile"), Some("release"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn parent_enum_value_matching_subcommand_does_not_select_it() {
        let command = Command::new("root")
            .enum_option(EnumOption::new("mode", ["build", "run"]))
            .command(Command::new("deploy"));

        let matches = command.parse_from(["--mode", "build", "deploy"]).unwrap();

        assert_eq!(matches.enum_option("mode"), Some("build"));
        assert_eq!(matches.subcommand(), Some("deploy"));
    }

    #[test]
    fn parent_boolean_positive_before_subcommand() {
        let command = Command::new("root")
            .flag(Flag::new("verbose"))
            .command(Command::new("build"));

        let matches = command.parse_from(["--verbose", "build"]).unwrap();

        assert!(matches.flag("verbose"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn parent_boolean_negation_before_subcommand() {
        let command = Command::new("root")
            .flag(Flag::new("verbose"))
            .command(Command::new("build"));

        let matches = command.parse_from(["--no-verbose", "build"]).unwrap();

        assert!(!matches.flag("verbose"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn child_string_option_after_subcommand() {
        let command =
            Command::new("root").command(Command::new("build").option(StringOption::new("target")));

        let matches = command.parse_from(["build", "--target", "wasm"]).unwrap();

        assert_eq!(
            matches.subcommand_matches().unwrap().option("target"),
            Some("wasm")
        );
    }

    #[test]
    fn child_enum_option_after_subcommand() {
        let command = Command::new("root").command(
            Command::new("build").enum_option(EnumOption::new("mode", ["debug", "release"])),
        );

        let matches = command.parse_from(["build", "--mode", "release"]).unwrap();

        assert_eq!(
            matches.subcommand_matches().unwrap().enum_option("mode"),
            Some("release")
        );
    }

    #[test]
    fn child_boolean_option_after_subcommand() {
        let command =
            Command::new("root").command(Command::new("build").flag(Flag::new("verbose")));

        let matches = command.parse_from(["build", "--verbose"]).unwrap();

        assert!(matches.subcommand_matches().unwrap().flag("verbose"));
    }

    #[test]
    fn parent_only_option_after_child_selection_errors_against_child() {
        let command = Command::new("root")
            .option(StringOption::new("profile"))
            .command(Command::new("build"));

        let error = command
            .parse_from(["build", "--profile", "release"])
            .unwrap_err();

        assert_eq!(error.message(), "unknown flag: --profile");
    }

    #[test]
    fn string_option_value_matching_command_is_not_command() {
        let command = Command::new("root")
            .option(StringOption::new("target"))
            .command(Command::new("build"));

        let matches = command.parse_from(["--target", "build"]).unwrap();

        assert_eq!(matches.option("target"), Some("build"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn string_option_alias_value_matching_command_is_not_command() {
        let command = Command::new("root")
            .option(StringOption::new("target").alias("t"))
            .command(Command::new("build"));

        let matches = command.parse_from(["-t", "build"]).unwrap();

        assert_eq!(matches.option("target"), Some("build"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn enum_option_value_matching_command_is_not_command() {
        let command = Command::new("root")
            .enum_option(EnumOption::new("mode", ["build"]))
            .command(Command::new("build"));

        let matches = command.parse_from(["--mode", "build"]).unwrap();

        assert_eq!(matches.enum_option("mode"), Some("build"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn enum_option_alias_value_matching_command_is_not_command() {
        let command = Command::new("root")
            .enum_option(EnumOption::new("mode", ["build"]).alias("m"))
            .command(Command::new("build"));

        let matches = command.parse_from(["-m", "build"]).unwrap();

        assert_eq!(matches.enum_option("mode"), Some("build"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn equals_string_value_matching_command_is_not_command() {
        let command = Command::new("root")
            .option(StringOption::new("target"))
            .command(Command::new("build"));

        let matches = command.parse_from(["--target=build"]).unwrap();

        assert_eq!(matches.option("target"), Some("build"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn equals_enum_value_matching_command_is_not_command() {
        let command = Command::new("root")
            .enum_option(EnumOption::new("mode", ["build"]))
            .command(Command::new("build"));

        let matches = command.parse_from(["--mode=build"]).unwrap();

        assert_eq!(matches.enum_option("mode"), Some("build"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn two_level_nested_subcommand_parsing() {
        let command =
            Command::new("root").command(Command::new("remote").command(Command::new("add")));

        let matches = command.parse_from(["remote", "add"]).unwrap();

        assert_eq!(matches.subcommand(), Some("remote"));
        assert_eq!(
            matches.subcommand_matches().unwrap().subcommand(),
            Some("add")
        );
    }

    #[test]
    fn three_level_nested_subcommand_parsing() {
        let command = Command::new("root").command(
            Command::new("remote").command(Command::new("add").command(Command::new("verify"))),
        );

        let matches = command.parse_from(["remote", "add", "verify"]).unwrap();

        let remote = matches.subcommand_matches().unwrap();
        let add = remote.subcommand_matches().unwrap();

        assert_eq!(matches.subcommand(), Some("remote"));
        assert_eq!(remote.subcommand(), Some("add"));
        assert_eq!(add.subcommand(), Some("verify"));
    }

    #[test]
    fn nested_subcommand_alias_canonicalizes() {
        let command = Command::new("root").command(
            Command::new("remote")
                .alias("r")
                .command(Command::new("add").alias("a")),
        );

        let matches = command.parse_from(["r", "a"]).unwrap();

        assert_eq!(matches.subcommand(), Some("remote"));
        assert_eq!(
            matches.subcommand_matches().unwrap().subcommand(),
            Some("add")
        );
    }

    #[test]
    fn nested_child_option_parsing() {
        let command = Command::new("root").command(
            Command::new("remote").command(Command::new("add").option(StringOption::new("name"))),
        );

        let matches = command
            .parse_from(["remote", "add", "--name", "origin"])
            .unwrap();

        let add = matches
            .subcommand_matches()
            .unwrap()
            .subcommand_matches()
            .unwrap();
        assert_eq!(add.option("name"), Some("origin"));
    }

    #[test]
    fn nested_child_required_validation() {
        let command = Command::new("root").command(
            Command::new("remote")
                .command(Command::new("add").option(StringOption::new("name").required())),
        );

        let error = command.parse_from(["remote", "add"]).unwrap_err();

        assert_eq!(error.message(), "missing required option: --name");
    }

    #[test]
    fn parent_required_validation_still_occurs_with_subcommand() {
        let command = Command::new("root")
            .option(StringOption::new("profile").required())
            .command(Command::new("build"));

        let error = command.parse_from(["build"]).unwrap_err();

        assert_eq!(error.message(), "missing required option: --profile");
    }

    #[test]
    fn parent_defaults_survive_child_selection() {
        let command = Command::new("root")
            .option(StringOption::new("profile").default("default-profile"))
            .command(
                Command::new("build").option(StringOption::new("target").default("default-target")),
            );

        let matches = command.parse_from(["build"]).unwrap();

        assert_eq!(matches.option("profile"), Some("default-profile"));
        assert_eq!(matches.subcommand(), Some("build"));
        assert_eq!(
            matches.subcommand_matches().unwrap().option("target"),
            Some("default-target")
        );
    }

    #[test]
    fn child_defaults_stored_in_child_matches_only() {
        let command = Command::new("root")
            .command(Command::new("build").option(StringOption::new("target").default("wasm")));

        let matches = command.parse_from(["build"]).unwrap();

        assert_eq!(matches.option("target"), None);
        assert_eq!(
            matches.subcommand_matches().unwrap().option("target"),
            Some("wasm")
        );
    }

    #[test]
    fn subcommand_selected_over_positional_when_both_could_match() {
        let command = Command::new("root")
            .arg(Arg::new("value"))
            .command(Command::new("build"));

        let matches = command.parse_from(["build"]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
        assert_eq!(matches.argument("value"), None);
    }

    #[test]
    fn non_command_bare_token_fills_positional_when_available() {
        let command = Command::new("root")
            .arg(Arg::new("value"))
            .command(Command::new("build"));

        let matches = command.parse_from(["something-else"]).unwrap();

        assert_eq!(matches.argument("value"), Some("something-else"));
        assert_eq!(matches.subcommand(), None);
    }

    #[test]
    fn unknown_command_when_no_positional_can_accept_token() {
        let command = Command::new("root").command(Command::new("build"));

        let error = command.parse_from(["foo"]).unwrap_err();

        assert_eq!(error.message(), "unknown command: foo");
    }

    #[test]
    fn selected_child_prevents_parent_selecting_later_sibling() {
        let command = Command::new("root")
            .command(Command::new("build").arg(Arg::new("rest")))
            .command(Command::new("test"));

        let matches = command.parse_from(["build", "test"]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
        assert_eq!(
            matches.subcommand_matches().unwrap().argument("rest"),
            Some("test")
        );
    }

    #[test]
    fn subcommand_matches_returns_correct_child() {
        let command = Command::new("root")
            .command(Command::new("build"))
            .command(Command::new("test"));

        let matches = command.parse_from(["test"]).unwrap();

        assert!(matches.subcommand_matches().is_some());
        assert_eq!(matches.subcommand_matches().unwrap().subcommand(), None);
    }

    #[test]
    fn no_subcommand_selected_returns_normal_matches() {
        let command = Command::new("root").command(Command::new("build"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.subcommand(), None);
        assert!(matches.subcommand_matches().is_none());
    }

    #[test]
    fn new_command_has_no_default_subcommand() {
        let command = Command::new("root");

        assert_eq!(command.get_default_subcommand(), None);
    }

    #[test]
    fn default_subcommand_builder_stores_exact_spelling() {
        let command = Command::new("root").default_subcommand("b");

        assert_eq!(command.get_default_subcommand(), Some("b"));
        assert_eq!(command.aliases(), &[] as &[String]);
    }

    #[test]
    fn empty_argv_selects_default_subcommand() {
        let command = Command::new("root")
            .default_subcommand("build")
            .command(Command::new("build"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
        assert!(matches.subcommand_matches().is_some());
    }

    #[test]
    fn default_subcommand_alias_resolves_to_canonical_name() {
        let command = Command::new("root")
            .default_subcommand("b")
            .command(Command::new("build").alias("b"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn explicit_child_overrides_default_subcommand() {
        let command = Command::new("root")
            .default_subcommand("build")
            .command(Command::new("build"))
            .command(Command::new("test"));

        let matches = command.parse_from(["test"]).unwrap();

        assert_eq!(matches.subcommand(), Some("test"));
    }

    #[test]
    fn explicit_child_alias_overrides_default_subcommand() {
        let command = Command::new("root")
            .default_subcommand("build")
            .command(Command::new("build"))
            .command(Command::new("test").alias("t"));

        let matches = command.parse_from(["t"]).unwrap();

        assert_eq!(matches.subcommand(), Some("test"));
    }

    #[test]
    fn parent_boolean_option_consumed_before_default_subcommand() {
        let command = Command::new("root")
            .flag(Flag::new("verbose"))
            .default_subcommand("build")
            .command(Command::new("build"));

        let matches = command.parse_from(["--verbose"]).unwrap();

        assert!(matches.flag("verbose"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn parent_value_option_consumed_before_default_subcommand() {
        let command = Command::new("root")
            .option(StringOption::new("profile"))
            .default_subcommand("build")
            .command(Command::new("build"));

        let matches = command.parse_from(["--profile", "release"]).unwrap();

        assert_eq!(matches.option("profile"), Some("release"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn parent_positional_retained_before_default_subcommand() {
        let command = Command::new("root")
            .arg(Arg::new("workspace"))
            .default_subcommand("build")
            .command(Command::new("build"));

        let matches = command.parse_from(["project"]).unwrap();

        assert_eq!(matches.argument("workspace"), Some("project"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn parent_defaults_applied_before_default_subcommand_selection() {
        let command = Command::new("root")
            .option(StringOption::new("profile").default("release"))
            .default_subcommand("build")
            .command(Command::new("build"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.option("profile"), Some("release"));
        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn parent_required_validation_precedes_default_subcommand_selection() {
        let command = Command::new("root")
            .option(StringOption::new("profile").required())
            .default_subcommand("build")
            .command(Command::new("build"));

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.message(), "missing required option: --profile");
    }

    #[test]
    fn default_subcommand_child_receives_own_defaults() {
        let command = Command::new("root")
            .default_subcommand("build")
            .command(Command::new("build").option(StringOption::new("target").default("native")));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(
            matches.subcommand_matches().unwrap().option("target"),
            Some("native")
        );
    }

    #[test]
    fn default_subcommand_child_required_validation_propagates() {
        let command = Command::new("root")
            .default_subcommand("build")
            .command(Command::new("build").option(StringOption::new("target").required()));

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.message(), "missing required option: --target");
    }

    #[test]
    fn recursive_default_subcommand_chain_resolves_at_every_level() {
        let command = Command::new("root").default_subcommand("remote").command(
            Command::new("remote")
                .default_subcommand("status")
                .command(Command::new("status")),
        );

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.subcommand(), Some("remote"));
        let remote_matches = matches.subcommand_matches().unwrap();
        assert_eq!(remote_matches.subcommand(), Some("status"));
        assert!(remote_matches.subcommand_matches().is_some());
    }

    #[test]
    fn missing_configured_default_subcommand_errors() {
        let command = Command::new("root").default_subcommand("build");

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.message(), "default subcommand not found: build");
    }

    #[test]
    fn ambiguous_configured_default_subcommand_errors() {
        let command = Command::new("root")
            .default_subcommand("x")
            .command(Command::new("first").alias("x"))
            .command(Command::new("second").alias("x"));

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.message(), "ambiguous command: x");
    }

    #[test]
    fn default_canonical_vs_alias_collision_errors() {
        let command = Command::new("root")
            .default_subcommand("build")
            .command(Command::new("build"))
            .command(Command::new("deploy").alias("build"));

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.message(), "ambiguous command: build");
    }

    #[test]
    fn duplicate_alias_within_one_child_does_not_self_collide_as_default() {
        let command = Command::new("root")
            .default_subcommand("b")
            .command(Command::new("build").alias("b").alias("b"));

        let matches = command.parse_from([] as [&str; 0]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
    }

    #[test]
    fn option_owned_by_default_subcommand_is_forwarded_to_it() {
        let command = Command::new("root")
            .default_subcommand("build")
            .command(Command::new("build").option(StringOption::new("target")));

        let matches = command.parse_from(["--target", "wasm"]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
        assert_eq!(
            matches.subcommand_matches().unwrap().option("target"),
            Some("wasm")
        );
    }

    #[test]
    fn explicit_child_argv_behavior_unchanged_with_default_configured() {
        let command = Command::new("root")
            .default_subcommand("build")
            .command(Command::new("build").option(StringOption::new("target")))
            .command(Command::new("test"));

        let matches = command.parse_from(["build", "--target", "wasm"]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
        assert_eq!(
            matches.subcommand_matches().unwrap().option("target"),
            Some("wasm")
        );
    }

    // -- Default-subcommand input: options, flags, positionals --

    #[test]
    fn default_subcommand_receives_canonical_long_string_option() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").option(StringOption::new("format")));

        let matches = command.parse_from(["--format", "json"]).unwrap();

        assert_eq!(matches.subcommand(), Some("run"));
        assert_eq!(
            matches.subcommand_matches().unwrap().option("format"),
            Some("json")
        );
    }

    #[test]
    fn default_subcommand_receives_short_string_option_alias() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").option(StringOption::new("format").alias("f")));

        let matches = command.parse_from(["-f", "json"]).unwrap();

        assert_eq!(
            matches.subcommand_matches().unwrap().option("format"),
            Some("json")
        );
    }

    #[test]
    fn default_subcommand_receives_unicode_scalar_short_alias() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").option(StringOption::new("format").alias("é")));

        let separate = command.parse_from(["-é", "json"]).unwrap();
        let equals = command.parse_from(["-é=json"]).unwrap();

        assert_eq!(
            separate.subcommand_matches().unwrap().option("format"),
            Some("json")
        );
        assert_eq!(
            equals.subcommand_matches().unwrap().option("format"),
            Some("json")
        );
    }

    #[test]
    fn default_subcommand_receives_name_equals_value() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").option(StringOption::new("format")));

        let matches = command.parse_from(["--format=json"]).unwrap();

        assert_eq!(
            matches.subcommand_matches().unwrap().option("format"),
            Some("json")
        );
    }

    #[test]
    fn default_subcommand_receives_enum_option() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").enum_option(EnumOption::new("level", ["debug", "info"])));

        let matches = command.parse_from(["--level", "debug"]).unwrap();

        assert_eq!(
            matches.subcommand_matches().unwrap().enum_option("level"),
            Some("debug")
        );
    }

    #[test]
    fn default_subcommand_receives_boolean_flag() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").flag(Flag::new("verbose")));

        let matches = command.parse_from(["--verbose"]).unwrap();

        assert!(matches.subcommand_matches().unwrap().flag("verbose"));
    }

    #[test]
    fn default_subcommand_receives_positional() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").arg(Arg::new("file")));

        let matches = command.parse_from(["main.rs"]).unwrap();

        assert_eq!(matches.subcommand(), Some("run"));
        assert_eq!(
            matches.subcommand_matches().unwrap().argument("file"),
            Some("main.rs")
        );
    }

    #[test]
    fn default_subcommand_required_positional_satisfied_from_argv() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").arg(Arg::new("file").required()));

        let matches = command.parse_from(["main.rs"]).unwrap();

        assert_eq!(
            matches.subcommand_matches().unwrap().argument("file"),
            Some("main.rs")
        );
    }

    #[test]
    fn default_subcommand_required_option_satisfied_from_argv() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").option(StringOption::new("format").required()));

        let matches = command.parse_from(["--format", "json"]).unwrap();

        assert_eq!(
            matches.subcommand_matches().unwrap().option("format"),
            Some("json")
        );
    }

    #[test]
    fn default_subcommand_defaults_still_apply_alongside_argv() {
        let command = Command::new("root").default_subcommand("run").command(
            Command::new("run")
                .option(StringOption::new("format"))
                .option(StringOption::new("target").default("native")),
        );

        let matches = command.parse_from(["--format", "json"]).unwrap();

        let child = matches.subcommand_matches().unwrap();
        assert_eq!(child.option("format"), Some("json"));
        assert_eq!(child.option("target"), Some("native"));
    }

    #[test]
    fn parent_option_and_default_subcommand_option_coexist() {
        let command = Command::new("root")
            .option(StringOption::new("profile"))
            .default_subcommand("run")
            .command(Command::new("run").option(StringOption::new("format")));

        let matches = command
            .parse_from(["--profile", "release", "--format", "json"])
            .unwrap();

        assert_eq!(matches.option("profile"), Some("release"));
        assert_eq!(
            matches.subcommand_matches().unwrap().option("format"),
            Some("json")
        );
    }

    #[test]
    fn parent_flag_and_default_subcommand_option_coexist() {
        let command = Command::new("root")
            .flag(Flag::new("quiet"))
            .default_subcommand("run")
            .command(Command::new("run").option(StringOption::new("format")));

        let matches = command.parse_from(["--quiet", "--format", "json"]).unwrap();

        assert!(matches.flag("quiet"));
        assert_eq!(
            matches.subcommand_matches().unwrap().option("format"),
            Some("json")
        );
    }

    #[test]
    fn option_owned_by_neither_parent_nor_default_subcommand_errors() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").option(StringOption::new("format")));

        let error = command.parse_from(["--bogus", "x"]).unwrap_err();

        assert_eq!(error.message(), "unknown flag: --bogus");
    }

    #[test]
    fn nested_default_subcommand_chain_forwards_argv_at_every_level() {
        let command = Command::new("root").default_subcommand("remote").command(
            Command::new("remote")
                .default_subcommand("status")
                .command(Command::new("status").option(StringOption::new("format"))),
        );

        let matches = command.parse_from(["--format", "json"]).unwrap();

        let remote_matches = matches.subcommand_matches().unwrap();
        let status_matches = remote_matches.subcommand_matches().unwrap();
        assert_eq!(status_matches.option("format"), Some("json"));
    }

    // -- `--` terminator --

    #[test]
    fn terminator_itself_is_not_stored_as_positional() {
        let command = Command::new("root").arg(Arg::new("name"));

        let matches = command.parse_from(["--", "value"]).unwrap();

        assert_eq!(matches.argument("name"), Some("value"));
    }

    #[test]
    fn terminator_treats_long_flag_spelling_literally() {
        let command = Command::new("root")
            .flag(Flag::new("verbose"))
            .arg(Arg::new("value"));

        let matches = command.parse_from(["--", "--verbose"]).unwrap();

        assert!(!matches.flag("verbose"));
        assert_eq!(matches.argument("value"), Some("--verbose"));
    }

    #[test]
    fn terminator_treats_short_flag_spelling_literally() {
        let command = Command::new("root")
            .flag(Flag::new("verbose").short('x'))
            .arg(Arg::new("value"));

        let matches = command.parse_from(["--", "-x"]).unwrap();

        assert!(!matches.flag("verbose"));
        assert_eq!(matches.argument("value"), Some("-x"));
    }

    #[test]
    fn terminator_treats_name_equals_value_literally() {
        let command = Command::new("root")
            .option(StringOption::new("name"))
            .arg(Arg::new("value"));

        let matches = command.parse_from(["--", "--name=value"]).unwrap();

        assert_eq!(matches.option("name"), None);
        assert_eq!(matches.argument("value"), Some("--name=value"));
    }

    #[test]
    fn terminator_does_not_negate_flag_via_no_prefix() {
        let command = Command::new("root")
            .flag(Flag::new("verbose").default(true))
            .arg(Arg::new("value"));

        let matches = command.parse_from(["--", "--no-verbose"]).unwrap();

        assert!(matches.flag("verbose"));
        assert_eq!(matches.argument("value"), Some("--no-verbose"));
    }

    #[test]
    fn terminator_prevents_child_name_spelling_from_selecting_subcommand() {
        let command = Command::new("root")
            .arg(Arg::new("value"))
            .command(Command::new("build"));

        let matches = command.parse_from(["--", "build"]).unwrap();

        assert_eq!(matches.subcommand(), None);
        assert_eq!(matches.argument("value"), Some("build"));
    }

    #[test]
    fn explicit_child_followed_by_its_own_terminator_works() {
        let command = Command::new("root").command(
            Command::new("build")
                .flag(Flag::new("release"))
                .arg(Arg::new("value")),
        );

        let matches = command.parse_from(["build", "--", "--release"]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
        let child = matches.subcommand_matches().unwrap();
        assert!(!child.flag("release"));
        assert_eq!(child.argument("value"), Some("--release"));
    }

    #[test]
    fn positionals_before_and_after_terminator_bind_in_declaration_order() {
        let command = Command::new("root")
            .arg(Arg::new("first"))
            .arg(Arg::new("second"));

        let matches = command.parse_from(["one", "--", "two"]).unwrap();

        assert_eq!(matches.argument("first"), Some("one"));
        assert_eq!(matches.argument("second"), Some("two"));
    }

    #[test]
    fn excess_positional_after_terminator_errors() {
        let command = Command::new("root").arg(Arg::new("only"));

        let error = command.parse_from(["--", "one", "two"]).unwrap_err();

        assert_eq!(error.message(), "unexpected argument: two");
    }

    #[test]
    fn terminator_forwards_literal_positional_to_default_subcommand() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").arg(Arg::new("file")));

        let matches = command.parse_from(["--", "--verbose"]).unwrap();

        assert_eq!(matches.subcommand(), Some("run"));
        assert_eq!(
            matches.subcommand_matches().unwrap().argument("file"),
            Some("--verbose")
        );
    }

    #[test]
    fn terminator_before_explicit_child_argv_is_forwarded_intact() {
        let command = Command::new("root").command(
            Command::new("build")
                .flag(Flag::new("release"))
                .arg(Arg::new("value")),
        );

        let matches = command.parse_from(["build", "--", "--release"]).unwrap();

        assert_eq!(matches.subcommand(), Some("build"));
        let child = matches.subcommand_matches().unwrap();
        assert_eq!(child.argument("value"), Some("--release"));
    }

    // -- Existing error parity --

    #[test]
    fn unexpected_positional_errors_when_no_subcommands_declared() {
        let command = Command::new("root").arg(Arg::new("only"));

        let error = command.parse_from(["one", "two"]).unwrap_err();

        assert_eq!(error.message(), "unexpected argument: two");
    }

    #[test]
    fn excess_positional_beyond_default_subcommand_capacity_errors() {
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").arg(Arg::new("file")));

        let error = command.parse_from(["a.rs", "b.rs"]).unwrap_err();

        assert_eq!(error.message(), "unexpected argument: b.rs");
    }

    #[test]
    fn terminator_before_default_subcommand_selection_still_forwards_ambiguous_spelling() {
        // Root has no explicit child; "build" only exists as the default
        // child's own name. A leading `--` still must not turn "build" into
        // subcommand recognition — it becomes a literal positional handed to
        // the default child, exactly as when no terminator is present, since
        // the default child was always going to be selected regardless.
        let command = Command::new("root")
            .default_subcommand("run")
            .command(Command::new("run").arg(Arg::new("target")));

        let matches = command.parse_from(["--", "build"]).unwrap();

        assert_eq!(matches.subcommand(), Some("run"));
        assert_eq!(
            matches.subcommand_matches().unwrap().argument("target"),
            Some("build")
        );
    }

    // -- Hidden subcommands --

    #[test]
    fn new_command_is_visible_by_default() {
        let command = Command::new("build");

        assert!(!command.is_hidden());
    }

    #[test]
    fn hidden_marks_command_hidden() {
        let command = Command::new("internal").hidden();

        assert!(command.is_hidden());
    }

    #[test]
    fn hidden_command_remains_explicitly_parseable() {
        let root = Command::new("root")
            .command(Command::new("public"))
            .command(Command::new("internal").hidden());

        let matches = root.parse_from(["internal"]).unwrap();

        assert_eq!(matches.subcommand(), Some("internal"));
    }

    #[test]
    fn hidden_command_alias_remains_parseable() {
        let root = Command::new("root").command(Command::new("internal").alias("i").hidden());

        let matches = root.parse_from(["i"]).unwrap();

        assert_eq!(matches.subcommand(), Some("internal"));
    }

    #[test]
    fn hidden_default_subcommand_still_resolves() {
        let root = Command::new("root")
            .default_subcommand("internal")
            .command(Command::new("internal").hidden());

        let matches = root.parse_from(Vec::<&str>::new()).unwrap();

        assert_eq!(matches.subcommand(), Some("internal"));
    }

    // -- Usage: header --

    #[test]
    fn usage_basic_command_name() {
        let command = Command::new("ritty");

        assert_eq!(command.render_usage(), "USAGE ritty");
    }

    #[test]
    fn usage_description() {
        let command = Command::new("ritty").description("Elegant CLI builder");

        assert_eq!(command.render_usage(), "Elegant CLI builder\n\nUSAGE ritty");
    }

    #[test]
    fn usage_version() {
        let command = Command::new("ritty").version("1.0.0");

        assert_eq!(command.render_usage(), "ritty v1.0.0\n\nUSAGE ritty");
    }

    #[test]
    fn usage_description_and_version() {
        let command = Command::new("ritty")
            .description("Elegant CLI builder")
            .version("1.0.0");

        assert_eq!(
            command.render_usage(),
            "Elegant CLI builder (ritty v1.0.0)\n\nUSAGE ritty"
        );
    }

    // -- Usage: positionals --

    #[test]
    fn usage_required_positional() {
        let command = Command::new("ritty").arg(Arg::new("target").required());

        assert_eq!(
            command.render_usage(),
            "USAGE ritty <TARGET>\n\nARGUMENTS\n\n  TARGET  (Required)"
        );
    }

    #[test]
    fn usage_optional_positional() {
        let command = Command::new("ritty").arg(Arg::new("target"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [TARGET]\n\nARGUMENTS\n\n  TARGET"
        );
    }

    #[test]
    fn usage_positional_default() {
        let command = Command::new("ritty").arg(Arg::new("target").required().default("main"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [TARGET]\n\nARGUMENTS\n\n  TARGET  (Default: main)"
        );
    }

    #[test]
    fn usage_positional_description() {
        let command = Command::new("ritty").arg(Arg::new("target").description("Build target"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [TARGET]\n\nARGUMENTS\n\n  TARGET  Build target"
        );
    }

    #[test]
    fn usage_positional_value_hint() {
        let command = Command::new("ritty").arg(Arg::new("target").value_hint("dir"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [TARGET]\n\nARGUMENTS\n\n  TARGET <dir>"
        );
    }

    // -- Usage: string options --

    #[test]
    fn usage_string_option() {
        let command = Command::new("ritty").option(StringOption::new("output"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --output=<output>"
        );
    }

    #[test]
    fn usage_short_string_alias() {
        let command = Command::new("ritty").option(StringOption::new("output").alias("o"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -o, --output=<output>"
        );
    }

    #[test]
    fn usage_unicode_scalar_string_alias_matches_parser_spelling() {
        let command = Command::new("ritty").option(StringOption::new("output").alias("é"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -é, --output=<output>"
        );
        assert_eq!(
            command.parse_from(["-é", "dist"]).unwrap().option("output"),
            Some("dist")
        );
    }

    #[test]
    fn usage_long_string_alias() {
        let command =
            Command::new("ritty").option(StringOption::new("output").alias("destination"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --destination, --output=<output>"
        );
    }

    #[test]
    fn usage_string_value_hint() {
        let command = Command::new("ritty").option(StringOption::new("output").value_hint("dir"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --output=<dir>"
        );
    }

    #[test]
    fn usage_string_default() {
        let command = Command::new("ritty").option(StringOption::new("output").default("dist"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --output=<output>  (Default: dist)"
        );
    }

    #[test]
    fn usage_required_string_option() {
        let command = Command::new("ritty").option(StringOption::new("output").required());

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS] --output=<output>\n\nOPTIONS\n\n  --output=<output>  (Required)"
        );
    }

    // -- Usage: enum options --

    #[test]
    fn usage_enum_choices() {
        let command =
            Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info", "warn"]));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --level=<debug|info|warn>"
        );
    }

    #[test]
    fn usage_enum_aliases() {
        let command = Command::new("ritty").enum_option(
            EnumOption::new("level", ["debug", "info"])
                .alias("l")
                .alias("log-level"),
        );

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -l, --log-level, --level=<debug|info>"
        );
    }

    #[test]
    fn usage_enum_default() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).default("info"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --level=<debug|info>  (Default: info)"
        );
    }

    // -- Usage: boolean flags --

    #[test]
    fn usage_boolean_canonical_flag() {
        let command = Command::new("ritty").flag(Flag::new("verbose"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --verbose"
        );
    }

    #[test]
    fn usage_dedicated_short_flag() {
        let command = Command::new("ritty").flag(Flag::new("verbose").short('v'));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -v, --verbose"
        );
    }

    #[test]
    fn usage_boolean_aliases() {
        let command = Command::new("ritty").flag(Flag::new("verbose").short('v').alias("chatty"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -v, --chatty, --verbose"
        );
    }

    #[test]
    fn usage_boolean_default() {
        let command = Command::new("ritty").flag(Flag::new("verbose").default(false));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --verbose  (Default: false)"
        );
    }

    #[test]
    fn usage_boolean_required_marker() {
        let command = Command::new("ritty").flag(Flag::new("verbose").required());

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS] --verbose\n\nOPTIONS\n\n  --verbose  (Required)"
        );
    }

    // -- Usage: negative booleans --

    #[test]
    fn usage_negative_boolean_from_default_true() {
        let command = Command::new("ritty").flag(Flag::new("color").default(true));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --color     (Default: true)\n  --no-color"
        );
    }

    #[test]
    fn usage_negative_boolean_from_negative_description() {
        let command =
            Command::new("ritty").flag(Flag::new("color").negative_description("Disable color"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --color\n  --no-color  Disable color"
        );
    }

    #[test]
    fn usage_negative_boolean_from_default_true_and_negative_description() {
        let command = Command::new("ritty").flag(
            Flag::new("color")
                .default(true)
                .description("Enable color")
                .negative_description("Disable color"),
        );

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --color     Enable color (Default: true)\n  --no-color  Disable color"
        );
    }

    #[test]
    fn usage_no_double_negative_when_canonical_already_negative() {
        let command = Command::new("ritty").flag(Flag::new("no-cache").default(true));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  --no-cache  (Default: true)"
        );
    }

    #[test]
    fn usage_negative_boolean_does_not_advertise_unparseable_short_negation() {
        // `.short('v')` alone does not register "v" as a long alias, so
        // `--no-v` is not something the parser accepts; only `--no-verbose`
        // should be advertised.
        let command = Command::new("ritty").flag(Flag::new("verbose").short('v').default(true));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -v, --verbose  (Default: true)\n  --no-verbose"
        );
    }

    // -- Usage: alias deduplication --

    #[test]
    fn usage_aliases_deduplicate_visually() {
        let command = Command::new("ritty").flag(Flag::new("verbose").short('v').alias("v"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS]\n\nOPTIONS\n\n  -v, --verbose"
        );
    }

    // -- Usage: declaration order --

    #[test]
    fn usage_multiple_arguments_and_options_retain_declaration_order() {
        let command = Command::new("ritty")
            .arg(Arg::new("first"))
            .arg(Arg::new("second"))
            .flag(Flag::new("alpha"))
            .option(StringOption::new("beta"))
            .enum_option(EnumOption::new("gamma", ["x", "y"]));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty [OPTIONS] [FIRST] [SECOND]\n\n\
             ARGUMENTS\n\n  FIRST\n  SECOND\n\n\
             OPTIONS\n\n  --alpha\n  --beta=<beta>\n  --gamma=<x|y>"
        );
    }

    // -- Usage: subcommands --

    #[test]
    fn usage_visible_subcommands() {
        let command = Command::new("ritty")
            .command(Command::new("build").description("Build the project"))
            .command(Command::new("test").description("Run tests"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty build|test\n\n\
             COMMANDS\n\n  build  Build the project\n  test   Run tests"
        );
    }

    #[test]
    fn usage_subcommand_aliases() {
        let command = Command::new("ritty").command(Command::new("build").alias("b"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty build|b\n\nCOMMANDS\n\n  build, b"
        );
    }

    #[test]
    fn usage_subcommand_descriptions() {
        let command =
            Command::new("ritty").command(Command::new("build").description("Build the project"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty build\n\nCOMMANDS\n\n  build  Build the project"
        );
    }

    #[test]
    fn usage_hidden_subcommands_omitted() {
        let command = Command::new("ritty")
            .command(Command::new("build"))
            .command(Command::new("internal").hidden());

        assert_eq!(
            command.render_usage(),
            "USAGE ritty build\n\nCOMMANDS\n\n  build"
        );
    }

    #[test]
    fn usage_hidden_subcommand_aliases_omitted() {
        let command = Command::new("ritty")
            .command(Command::new("build"))
            .command(Command::new("internal").alias("i").hidden());

        assert_eq!(
            command.render_usage(),
            "USAGE ritty build\n\nCOMMANDS\n\n  build"
        );
    }

    #[test]
    fn usage_visible_command_alternatives_in_synopsis() {
        let command = Command::new("ritty")
            .command(Command::new("build").alias("b"))
            .command(Command::new("test").alias("t"));

        assert_eq!(
            command.render_usage(),
            "USAGE ritty build|b|test|t\n\nCOMMANDS\n\n  build, b\n  test, t"
        );
    }

    #[test]
    fn usage_all_hidden_subcommands_leave_no_commands_section() {
        let command = Command::new("ritty").command(Command::new("internal").hidden());

        assert_eq!(command.render_usage(), "USAGE ritty");
    }

    // -- Usage: empty / clean rendering --

    #[test]
    fn usage_command_with_no_metadata_renders_cleanly() {
        let command = Command::new("ritty");

        assert_eq!(command.render_usage(), "USAGE ritty");
    }

    #[test]
    fn usage_missing_descriptions_do_not_produce_artifacts() {
        let command = Command::new("ritty")
            .arg(Arg::new("target"))
            .flag(Flag::new("verbose"))
            .command(Command::new("build"));

        let rendered = command.render_usage();

        assert!(!rendered.contains("None"));
        assert!(!rendered.contains("Some("));
        assert!(!rendered.contains("undefined"));
    }

    #[test]
    fn usage_has_no_trailing_whitespace_on_any_line() {
        let command = Command::new("ritty")
            .description("Elegant CLI builder")
            .version("1.0.0")
            .arg(Arg::new("target"))
            .flag(Flag::new("verbose").short('v'))
            .option(StringOption::new("output").default("dist"))
            .enum_option(EnumOption::new("level", ["debug", "info"]).required())
            .command(
                Command::new("build")
                    .alias("b")
                    .description("Build the project"),
            )
            .command(Command::new("internal").hidden());

        let rendered = command.render_usage();

        for line in rendered.lines() {
            assert_eq!(line, line.trim_end());
        }
    }

    #[test]
    fn usage_rendering_is_deterministic_across_repeated_calls() {
        let command = Command::new("ritty")
            .description("Elegant CLI builder")
            .version("1.0.0")
            .arg(Arg::new("target").required())
            .flag(Flag::new("verbose").short('v'))
            .option(StringOption::new("output").alias("o").default("dist"))
            .command(Command::new("build").alias("b"));

        let first = command.render_usage();
        let second = command.render_usage();

        assert_eq!(first, second);
    }

    #[test]
    fn unknown_long_option_has_unknown_option_kind() {
        let command = Command::new("ritty");

        let error = command.parse_from(["--wat"]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
        );
    }

    #[test]
    fn unknown_short_option_has_unknown_option_kind() {
        let command = Command::new("ritty");

        let error = command.parse_from(["-x"]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
        );
    }

    #[test]
    fn ambiguous_option_has_ambiguous_option_kind() {
        let command = Command::new("ritty")
            .flag(Flag::new("verbose").short('v'))
            .option(StringOption::new("value").alias("v"));

        let error = command.parse_from(["-v"]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::AmbiguousOption)
        );
    }

    #[test]
    fn missing_string_option_value_has_missing_option_value_kind() {
        let command = Command::new("ritty").option(StringOption::new("name"));

        let error = command.parse_from(["--name"]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue)
        );
    }

    #[test]
    fn missing_enum_option_value_has_missing_option_value_kind() {
        let command =
            Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

        let error = command.parse_from(["--level"]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::MissingOptionValue)
        );
    }

    #[test]
    fn invalid_enum_value_has_invalid_option_value_kind() {
        let command =
            Command::new("ritty").enum_option(EnumOption::new("level", ["debug", "info"]));

        let error = command.parse_from(["--level", "nope"]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::InvalidOptionValue)
        );
    }

    #[test]
    fn missing_required_positional_has_missing_required_argument_kind() {
        let command = Command::new("ritty").arg(Arg::new("target").required());

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredArgument)
        );
    }

    #[test]
    fn missing_required_boolean_flag_has_missing_required_option_kind() {
        let command = Command::new("ritty").flag(Flag::new("confirm").required());

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption)
        );
    }

    #[test]
    fn missing_required_string_option_has_missing_required_option_kind() {
        let command = Command::new("ritty").option(StringOption::new("name").required());

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption)
        );
    }

    #[test]
    fn missing_required_enum_option_has_missing_required_option_kind() {
        let command = Command::new("ritty")
            .enum_option(EnumOption::new("level", ["debug", "info"]).required());

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption)
        );
    }

    #[test]
    fn unexpected_positional_has_unexpected_argument_kind() {
        let command = Command::new("ritty");

        let error = command.parse_from(["extra"]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::UnexpectedArgument)
        );
    }

    #[test]
    fn unknown_command_has_unknown_command_kind() {
        let command = Command::new("ritty").command(Command::new("build"));

        let error = command.parse_from(["deploy"]).unwrap_err();

        assert_eq!(error.kind(), ParseErrorKind::UnknownCommand);
    }

    #[test]
    fn ambiguous_command_has_ambiguous_command_kind() {
        let command = Command::new("ritty")
            .command(Command::new("install").alias("x"))
            .command(Command::new("inspect").alias("x"));

        let error = command.parse_from(["x"]).unwrap_err();

        assert_eq!(error.kind(), ParseErrorKind::AmbiguousCommand);
    }

    #[test]
    fn missing_configured_default_subcommand_has_default_subcommand_not_found_kind() {
        let command = Command::new("root").default_subcommand("build");

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.kind(), ParseErrorKind::DefaultSubcommandNotFound);
    }

    #[test]
    fn nested_default_child_error_retains_original_kind() {
        let command = Command::new("root")
            .default_subcommand("build")
            .command(Command::new("build").option(StringOption::new("output").required()));

        let error = command.parse_from([] as [&str; 0]).unwrap_err();

        assert_eq!(
            error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::MissingRequiredOption)
        );
    }

    #[test]
    fn parse_error_display_equals_message() {
        let command = Command::new("ritty");

        let error = command.parse_from(["--wat"]).unwrap_err();

        assert_eq!(error.to_string(), error.message());
    }

    #[test]
    fn parse_error_implements_std_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}

        let command = Command::new("ritty");
        let error = command.parse_from(["--wat"]).unwrap_err();

        assert_error(&error);
    }

    // --- Execution ---

    use std::sync::Mutex;

    #[derive(Debug)]
    struct Boom;

    impl std::fmt::Display for Boom {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("boom")
        }
    }

    impl std::error::Error for Boom {}

    #[test]
    fn handler_defaults_to_absent() {
        let command = Command::new("root");

        assert!(!command.has_handler());
    }

    #[test]
    fn root_handler_executes() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let command = Command::new("root").handler(move |_ctx| {
            recorded.lock().unwrap().push("root");
            Ok(())
        });

        command.run_from([] as [&str; 0]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["root"]);
    }

    #[test]
    fn handler_receives_its_local_matches() {
        let command = Command::new("root").arg(Arg::new("name")).handler(|ctx| {
            assert_eq!(ctx.matches().argument("name"), Some("alice"));
            Ok(())
        });

        command.run_from(["alice"]).unwrap();
    }

    #[test]
    fn handler_receives_root_matches() {
        let command = Command::new("root")
            .flag(Flag::new("verbose").short('v'))
            .command(Command::new("build").handler(|ctx| {
                assert!(ctx.root_matches().flag("verbose"));
                assert_eq!(ctx.root_matches().subcommand(), Some("build"));
                Ok(())
            }));

        command.run_from(["-v", "build"]).unwrap();
    }

    #[test]
    fn captured_closure_handler_works() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let command = Command::new("build").handler(move |_ctx| {
            recorded.lock().unwrap().push("build");
            Ok(())
        });

        command.run_from([] as [&str; 0]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["build"]);
    }

    #[test]
    fn cloned_command_retains_working_handler() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let command = Command::new("root").handler(move |_ctx| {
            recorded.lock().unwrap().push("root");
            Ok(())
        });

        let cloned = command.clone();
        cloned.run_from([] as [&str; 0]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["root"]);
    }

    #[test]
    fn root_options_are_parsed_before_root_handler_runs() {
        let command = Command::new("root")
            .flag(Flag::new("verbose").short('v'))
            .handler(|ctx| {
                assert!(ctx.matches().flag("verbose"));
                Ok(())
            });

        command.run_from(["-v"]).unwrap();
    }

    #[test]
    fn explicit_child_handler_executes() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let command = Command::new("root").command(Command::new("build").handler(move |_ctx| {
            recorded.lock().unwrap().push("build");
            Ok(())
        }));

        command.run_from(["build"]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["build"]);
    }

    #[test]
    fn parent_handler_is_suppressed_when_child_selected() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let root_calls = Arc::clone(&calls);
        let build_calls = Arc::clone(&calls);
        let command = Command::new("root")
            .handler(move |_ctx| {
                root_calls.lock().unwrap().push("root");
                Ok(())
            })
            .command(Command::new("build").handler(move |_ctx| {
                build_calls.lock().unwrap().push("build");
                Ok(())
            }));

        command.run_from(["build"]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["build"]);
    }

    #[test]
    fn subcommand_alias_executes_canonical_child_handler() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let command =
            Command::new("root").command(Command::new("build").alias("b").handler(move |_ctx| {
                recorded.lock().unwrap().push("build");
                Ok(())
            }));

        command.run_from(["b"]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["build"]);
    }

    #[test]
    fn nested_leaf_handler_executes() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let command = Command::new("root").command(Command::new("remote").command(
            Command::new("add").handler(move |_ctx| {
                recorded.lock().unwrap().push("add");
                Ok(())
            }),
        ));

        command.run_from(["remote", "add"]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["add"]);
    }

    #[test]
    fn intermediate_parent_handlers_are_not_executed() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let root_calls = Arc::clone(&calls);
        let remote_calls = Arc::clone(&calls);
        let add_calls = Arc::clone(&calls);
        let command = Command::new("root")
            .handler(move |_ctx| {
                root_calls.lock().unwrap().push("root");
                Ok(())
            })
            .command(
                Command::new("remote")
                    .handler(move |_ctx| {
                        remote_calls.lock().unwrap().push("remote");
                        Ok(())
                    })
                    .command(Command::new("add").handler(move |_ctx| {
                        add_calls.lock().unwrap().push("add");
                        Ok(())
                    })),
            );

        command.run_from(["remote", "add"]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["add"]);
    }

    #[test]
    fn default_child_handler_executes_on_empty_argv() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let command =
            Command::new("root")
                .default_subcommand("dev")
                .command(Command::new("dev").handler(move |_ctx| {
                    recorded.lock().unwrap().push("dev");
                    Ok(())
                }));

        command.run_from([] as [&str; 0]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["dev"]);
    }

    #[test]
    fn default_subcommand_alias_executes_canonical_child() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let command = Command::new("root").default_subcommand("d").command(
            Command::new("dev").alias("d").handler(move |_ctx| {
                recorded.lock().unwrap().push("dev");
                Ok(())
            }),
        );

        command.run_from([] as [&str; 0]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["dev"]);
    }

    #[test]
    fn hidden_subcommand_remains_executable() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let recorded = Arc::clone(&calls);
        let command =
            Command::new("root").command(Command::new("secret").hidden().handler(move |_ctx| {
                recorded.lock().unwrap().push("secret");
                Ok(())
            }));

        command.run_from(["secret"]).unwrap();

        assert_eq!(*calls.lock().unwrap(), vec!["secret"]);
    }

    #[test]
    fn child_handler_receives_child_matches() {
        let command =
            Command::new("root").command(Command::new("build").arg(Arg::new("target")).handler(
                |ctx| {
                    assert_eq!(ctx.matches().argument("target"), Some("web"));
                    Ok(())
                },
            ));

        command.run_from(["build", "web"]).unwrap();
    }

    #[test]
    fn child_handler_can_inspect_parent_and_root_matches() {
        let command = Command::new("root")
            .flag(Flag::new("verbose").short('v'))
            .command(Command::new("build").handler(|ctx| {
                assert!(ctx.root_matches().flag("verbose"));
                assert!(ctx.matches().argument("target").is_none());
                Ok(())
            }));

        command.run_from(["-v", "build"]).unwrap();
    }

    #[test]
    fn parse_failure_becomes_run_error_parse() {
        let command = Command::new("root");

        let error = command.run_from(["--wat"]).unwrap_err();

        assert!(matches!(error, RunError::Parse(_)));
    }

    #[test]
    fn run_error_parse_kind_is_preserved() {
        let command = Command::new("root");

        let error = command.run_from(["--wat"]).unwrap_err();

        let RunError::Parse(parse_error) = error else {
            panic!("expected RunError::Parse");
        };
        assert_eq!(
            parse_error.kind(),
            ParseErrorKind::Argument(ArgumentErrorKind::UnknownOption)
        );
    }

    #[test]
    fn run_error_parse_message_is_preserved() {
        let command = Command::new("root");

        let direct = command.parse_from(["--wat"]).unwrap_err();
        let error = command.run_from(["--wat"]).unwrap_err();

        let RunError::Parse(parse_error) = error else {
            panic!("expected RunError::Parse");
        };
        assert_eq!(parse_error.message(), direct.message());
    }

    #[test]
    fn unresolved_required_child_returns_no_command() {
        let command = Command::new("root").command(Command::new("build").handler(|_ctx| Ok(())));

        let error = command.run_from([] as [&str; 0]).unwrap_err();

        assert!(matches!(error, RunError::NoCommand));
    }

    #[test]
    fn empty_no_handler_command_succeeds_as_no_op() {
        let command = Command::new("root");

        command.run_from([] as [&str; 0]).unwrap();
    }

    #[test]
    fn selected_leaf_without_handler_succeeds_as_no_op() {
        let command = Command::new("root").command(Command::new("build"));

        command.run_from(["build"]).unwrap();
    }

    #[test]
    fn handler_failure_becomes_handler_error_variant() {
        let command = Command::new("root").handler(|_ctx| Err(Box::new(Boom) as BoxError));

        let error = command.run_from([] as [&str; 0]).unwrap_err();

        assert!(matches!(error, RunError::Handler(_)));
    }

    #[test]
    fn handler_error_is_exposed_through_source() {
        let command = Command::new("root").handler(|_ctx| Err(Box::new(Boom) as BoxError));

        let error = command.run_from([] as [&str; 0]).unwrap_err();

        let source = std::error::Error::source(&error).expect("handler error has a source");
        assert_eq!(source.to_string(), "boom");
    }

    #[test]
    fn run_error_implements_display() {
        let command = Command::new("root").command(Command::new("build").handler(|_ctx| Ok(())));

        let error = command.run_from([] as [&str; 0]).unwrap_err();

        assert_eq!(error.to_string(), "no command specified");
    }

    #[test]
    fn run_error_implements_std_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}

        let command = Command::new("root");
        let error = command.run_from(["--wat"]).unwrap_err();

        assert_error(&error);
    }

    #[test]
    fn parse_error_prevents_any_handler_call() {
        let calls = Arc::new(Mutex::new(0));
        let recorded = Arc::clone(&calls);
        let command = Command::new("root")
            .handler(move |_ctx| {
                *recorded.lock().unwrap() += 1;
                Ok(())
            })
            .command(Command::new("build").handler(|_ctx| {
                panic!("child handler must not run on parse failure");
            }));

        let error = command.run_from(["--wat"]).unwrap_err();

        assert!(matches!(error, RunError::Parse(_)));
        assert_eq!(*calls.lock().unwrap(), 0);
    }
}
