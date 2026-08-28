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
    pub(crate) fn new() -> Self {
        Self {
            flags: Vec::new(),
            arguments: Vec::new(),
            options: Vec::new(),
            enum_options: Vec::new(),
            subcommand: None,
            subcommand_matches: None,
        }
    }

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
    pub(crate) fn set_flag(&mut self, name: &str, value: bool) {
        if let Some(entry) = self.flags.iter_mut().find(|(flag, _)| flag == name) {
            entry.1 = value;
        } else {
            self.flags.push((name.to_owned(), value));
        }
    }

    pub(crate) fn has_flag(&self, name: &str) -> bool {
        self.flags.iter().any(|(flag, _)| flag == name)
    }

    pub(crate) fn push_argument(&mut self, name: String, value: String) {
        self.arguments.push((name, value));
    }

    pub(crate) fn push_option(&mut self, name: String, value: String) {
        self.options.push((name, value));
    }

    pub(crate) fn push_enum_option(&mut self, name: String, value: String) {
        self.enum_options.push((name, value));
    }

    pub(crate) fn select_subcommand(&mut self, name: String, matches: Matches) {
        self.subcommand = Some(name);
        self.subcommand_matches = Some(Box::new(matches));
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
