//! ZenConsoleInput: A simple, cross-platform console input library for Rust.
//!
//! This library provides an easy-to-use interface for getting various types of
//! user input from the console, including single-line input, multiline input,
//! password input, and selection from a list of options.

mod input;
mod password;
mod selection;

pub use input::Input;
pub use password::Password;
pub use selection::Selection;

/// The main struct for ZenConsoleInput.
///
/// Use this struct to create new input, password, or selection prompts.
pub struct ZenConsoleInput;

impl ZenConsoleInput {
    /// Creates a new ZenConsoleInput instance.
    pub fn new() -> Self {
        ZenConsoleInput
    }

    /// Creates a new Input instance for text input.
    pub fn input(&self) -> Input {
        Input::new()
    }

    /// Creates a new Selection instance for choosing from a list of options.
    pub fn selection(&self) -> Selection {
        Selection::new()
    }

    /// Creates a new Password instance for password input.
    pub fn password(&self) -> Password {
        Password::new()
    }

    /// Opens the user's default text editor to edit the given content.
    ///
    /// This function is only available when the "editor" feature is enabled.
    #[cfg(feature = "editor")]
    pub(crate) fn edit_in_external_editor(initial_content: &str) -> std::io::Result<String> {
        use std::fs::File;
        use std::io::{self, Read, Write};
        use std::process::Command;
        use tempfile::NamedTempFile;

        let mut temp_file = NamedTempFile::new()?;
        write!(temp_file, "{}", initial_content)?;

        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        let status = Command::new(editor).arg(temp_file.path()).status()?;

        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Editor exited with non-zero status",
            ));
        }

        let mut content = String::new();
        File::open(temp_file.path())?.read_to_string(&mut content)?;
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zen_console_input_creation() {
        let _zci = ZenConsoleInput::new();
    }
}
