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

/// A boolean flag in a Ritty command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flag {
    name: String,
    short: Option<char>,
}

impl Flag {
    /// Creates a new flag.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            short: None,
        }
    }

    /// Sets the short flag name.
    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
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
    flags: Vec<String>,
    arguments: Vec<(String, String)>,
    options: Vec<(String, String)>,
    subcommand: Option<String>,
}

impl Matches {
    /// Returns whether a flag was present.
    pub fn flag(&self, name: &str) -> bool {
        self.flags.iter().any(|flag| flag == name)
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
                    let candidates = self.options_matching_long(name);
                    if candidates.len() > 1 {
                        return Err(ParseError {
                            message: format!("ambiguous option: --{name}"),
                        });
                    }

                    if let Some(option) = candidates.first() {
                        matches
                            .options
                            .push((option.name().to_owned(), value.to_owned()));
                        index += 1;
                        continue;
                    }

                    return Err(ParseError {
                        message: format!("unknown flag: --{name}"),
                    });
                }

                let name = rest;

                if self.flags.iter().any(|flag| flag.name() == name) {
                    matches.flags.push(name.to_owned());
                    index += 1;
                    continue;
                }

                let candidates = self.options_matching_long(name);
                if candidates.len() > 1 {
                    return Err(ParseError {
                        message: format!("ambiguous option: --{name}"),
                    });
                }

                if let Some(option) = candidates.first() {
                    let value = args.get(index + 1).ok_or_else(|| ParseError {
                        message: format!("missing value for option: --{name}"),
                    })?;
                    matches
                        .options
                        .push((option.name().to_owned(), value.to_owned()));
                    index += 2;
                    continue;
                }

                return Err(ParseError {
                    message: format!("unknown flag: --{name}"),
                });
            }

            if let Some(rest) = arg.strip_prefix('-') {
                if let Some((name, value)) = rest.split_once('=') {
                    if name.len() == 1 {
                        let short = name.chars().next().unwrap();
                        let candidates = self.options_matching_short(short);

                        if candidates.len() > 1 {
                            return Err(ParseError {
                                message: format!("ambiguous option: -{short}"),
                            });
                        }

                        if let Some(option) = candidates.first() {
                            matches
                                .options
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
                    let flag_match = self
                        .flags
                        .iter()
                        .find(|flag| flag.short_name() == Some(short));
                    let option_candidates = self.options_matching_short(short);

                    if option_candidates.len() + usize::from(flag_match.is_some()) > 1 {
                        return Err(ParseError {
                            message: format!("ambiguous option: -{short}"),
                        });
                    }

                    if let Some(flag) = flag_match {
                        matches.flags.push(flag.name().to_owned());
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
}
