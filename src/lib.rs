//! Ritty — an elegant CLI builder for Rust.

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

    /// Returns the selected subcommand.
    pub fn subcommand(&self) -> Option<&str> {
        self.subcommand.as_deref()
    }
}

/// An error produced while parsing command-line input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    /// Returns the parse error message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Resolution of a bare long-option token to either a flag (with its
/// effective positive/negative value) or a string option.
enum LongMatch<'a> {
    Flag(&'a Flag, bool),
    Option(&'a StringOption),
    EnumOption(&'a EnumOption),
}

/// Validates an enum option's effective value against its allowed values.
/// An empty allowed-value list means there is no restriction.
fn validate_enum_value(option: &EnumOption, value: &str) -> Result<(), ParseError> {
    if option.values().is_empty() || option.values().iter().any(|allowed| allowed == value) {
        return Ok(());
    }

    Err(ParseError {
        message: format!(
            "invalid value for option: --{}: {} (expected one of: {})",
            option.name(),
            value,
            option.values().join(", ")
        ),
    })
}

/// A command in a Ritty CLI application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    name: String,
    description: Option<String>,
    version: Option<String>,
    subcommands: Vec<Command>,
    arguments: Vec<Arg>,
    flags: Vec<Flag>,
    options: Vec<StringOption>,
    enum_options: Vec<EnumOption>,
}

impl Command {
    /// Creates a new command.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            version: None,
            subcommands: Vec::new(),
            arguments: Vec::new(),
            flags: Vec::new(),
            options: Vec::new(),
            enum_options: Vec::new(),
        }
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
                    .any(|alias| alias.len() == 1 && alias.starts_with(short))
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
                    .any(|alias| alias.len() == 1 && alias.starts_with(short))
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
                        .any(|alias| alias.len() == 1 && alias.starts_with(short))
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
            return Err(ParseError {
                message: format!("ambiguous option: --{name}"),
            });
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

    /// Parses command-line arguments.
    pub fn parse_from<I, S>(&self, args: I) -> Result<Matches, ParseError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut matches = Matches {
            flags: Vec::new(),
            arguments: Vec::new(),
            options: Vec::new(),
            enum_options: Vec::new(),
            subcommand: None,
        };
        let mut positional = 0;

        let args: Vec<String> = args
            .into_iter()
            .map(|arg| arg.as_ref().to_owned())
            .collect();
        let mut index = 0;

        while index < args.len() {
            let arg = args[index].as_str();

            if let Some(rest) = arg.strip_prefix("--") {
                if let Some((name, value)) = rest.split_once('=') {
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
                        return Err(ParseError {
                            message: format!("ambiguous option: --{name}"),
                        });
                    }

                    if let Some(option) = string_candidates.first() {
                        matches
                            .options
                            .push((option.name().to_owned(), value.to_owned()));
                        index += 1;
                        continue;
                    }

                    if let Some(option) = enum_candidates.first() {
                        matches
                            .enum_options
                            .push((option.name().to_owned(), value.to_owned()));
                        index += 1;
                        continue;
                    }

                    return Err(ParseError {
                        message: format!("unknown flag: --{name}"),
                    });
                }

                let name = rest;

                match self.resolve_long(name)? {
                    Some(LongMatch::Flag(flag, value)) => {
                        matches.set_flag(flag.name(), value);
                        index += 1;
                        continue;
                    }
                    Some(LongMatch::Option(option)) => {
                        let value = args.get(index + 1).ok_or_else(|| ParseError {
                            message: format!("missing value for option: --{name}"),
                        })?;
                        matches
                            .options
                            .push((option.name().to_owned(), value.to_owned()));
                        index += 2;
                        continue;
                    }
                    Some(LongMatch::EnumOption(option)) => {
                        let value = args.get(index + 1).ok_or_else(|| ParseError {
                            message: format!("missing value for option: --{name}"),
                        })?;
                        matches
                            .enum_options
                            .push((option.name().to_owned(), value.to_owned()));
                        index += 2;
                        continue;
                    }
                    None => {
                        return Err(ParseError {
                            message: format!("unknown flag: --{name}"),
                        });
                    }
                }
            }

            if let Some(rest) = arg.strip_prefix('-') {
                if let Some((name, value)) = rest.split_once('=') {
                    if name.len() == 1 {
                        let short = name.chars().next().unwrap();
                        let flag_candidates = self.flags_matching_short(short);
                        let string_candidates = self.options_matching_short(short);
                        let enum_candidates = self.enum_options_matching_short(short);

                        if flag_candidates.len() + string_candidates.len() + enum_candidates.len()
                            > 1
                        {
                            return Err(ParseError {
                                message: format!("ambiguous option: -{short}"),
                            });
                        }

                        if let Some(option) = string_candidates.first() {
                            matches
                                .options
                                .push((option.name().to_owned(), value.to_owned()));
                            index += 1;
                            continue;
                        }

                        if let Some(option) = enum_candidates.first() {
                            matches
                                .enum_options
                                .push((option.name().to_owned(), value.to_owned()));
                            index += 1;
                            continue;
                        }
                    }

                    return Err(ParseError {
                        message: format!("unknown flag: -{rest}"),
                    });
                }

                if rest.len() == 1 {
                    let short = rest.chars().next().unwrap();
                    let flag_candidates = self.flags_matching_short(short);
                    let option_candidates = self.options_matching_short(short);
                    let enum_candidates = self.enum_options_matching_short(short);

                    if flag_candidates.len() + option_candidates.len() + enum_candidates.len() > 1 {
                        return Err(ParseError {
                            message: format!("ambiguous option: -{short}"),
                        });
                    }

                    if let Some(flag) = flag_candidates.first() {
                        matches.set_flag(flag.name(), true);
                        index += 1;
                        continue;
                    }

                    if let Some(option) = option_candidates.first() {
                        let value = args.get(index + 1).ok_or_else(|| ParseError {
                            message: format!("missing value for option: -{short}"),
                        })?;
                        matches
                            .options
                            .push((option.name().to_owned(), value.to_owned()));
                        index += 2;
                        continue;
                    }

                    if let Some(option) = enum_candidates.first() {
                        let value = args.get(index + 1).ok_or_else(|| ParseError {
                            message: format!("missing value for option: -{short}"),
                        })?;
                        matches
                            .enum_options
                            .push((option.name().to_owned(), value.to_owned()));
                        index += 2;
                        continue;
                    }
                }

                return Err(ParseError {
                    message: format!("unknown flag: -{rest}"),
                });
            }

            if self.subcommands.iter().any(|command| command.name() == arg) {
                matches.subcommand = Some(arg.to_owned());
                index += 1;
                continue;
            }

            if let Some(argument) = self.arguments.get(positional) {
                matches
                    .arguments
                    .push((argument.name().to_owned(), arg.to_owned()));
                positional += 1;
                index += 1;
                continue;
            }

            return Err(ParseError {
                message: format!("unexpected argument: {arg}"),
            });
        }

        for argument in self.arguments.iter().skip(positional) {
            match argument.default_value() {
                Some(default) => matches
                    .arguments
                    .push((argument.name().to_owned(), default.to_owned())),
                None if argument.is_required() => {
                    return Err(ParseError {
                        message: format!("missing required argument: {}", argument.name()),
                    });
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
                    return Err(ParseError {
                        message: format!("missing required option: --{}", flag.name()),
                    });
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
                    return Err(ParseError {
                        message: format!("missing required option: --{}", option.name()),
                    });
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
                    return Err(ParseError {
                        message: format!("missing required option: --{}", option.name()),
                    });
                }
                None => {}
            }
        }

        for option in &self.enum_options {
            if let Some(value) = matches.enum_option(option.name()) {
                validate_enum_value(option, value)?;
            }
        }

        Ok(matches)
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
}
