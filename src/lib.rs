//! Ritty — an elegant CLI builder for Rust.

/// A positional argument in a Ritty command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arg {
    name: String,
}

impl Arg {
    /// Creates a new positional argument.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the argument name.
    pub fn name(&self) -> &str {
        &self.name
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
}

impl Matches {
    /// Returns whether a flag was present.
    pub fn flag(&self, name: &str) -> bool {
        self.flags.iter().any(|flag| flag == name)
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

    /// Parses command-line arguments.
    pub fn parse_from<I, S>(&self, args: I) -> Result<Matches, ParseError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut matches = Matches { flags: Vec::new() };

        for arg in args {
            let arg = arg.as_ref();

            if let Some(name) = arg.strip_prefix("--") {
                if self.flags.iter().any(|flag| flag.name() == name) {
                    matches.flags.push(name.to_owned());
                    continue;
                }

                return Err(ParseError {
                    message: format!("unknown flag: --{name}"),
                });
            }

            if let Some(short) = arg.strip_prefix('-') {
                if short.len() == 1 {
                    let short = short.chars().next().unwrap();

                    if let Some(flag) = self
                        .flags
                        .iter()
                        .find(|flag| flag.short_name() == Some(short))
                    {
                        matches.flags.push(flag.name().to_owned());
                        continue;
                    }
                }

                return Err(ParseError {
                    message: format!("unknown flag: -{short}"),
                });
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
}
