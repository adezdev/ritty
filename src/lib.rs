//! Ritty — an elegant CLI builder for Rust.

/// A command in a Ritty CLI application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    name: String,
    description: Option<String>,
    version: Option<String>,
}

impl Command {
    /// Creates a new command.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            version: None,
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
}
