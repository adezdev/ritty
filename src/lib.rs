//! Ritty — an elegant CLI builder for Rust.

/// A command in a Ritty CLI application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    name: String,
}

impl Command {
    /// Creates a new command.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the command name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_command() {
        let command = Command::new("ritty");

        assert_eq!(command.name(), "ritty");
    }
}
