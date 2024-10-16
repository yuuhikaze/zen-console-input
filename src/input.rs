//! Provides functionality for getting text input from the user.

use std::io::{self, Write};
use std::str::FromStr;

/// Struct for handling text input from the user.
pub struct Input {
    message: String,
    default: Option<String>,
    tips: bool,
    multiline: bool,
}

impl Input {
    pub(crate) fn new() -> Self {
        Input {
            message: String::new(),
            default: None,
            tips: true,
            multiline: false,
        }
    }

    /// Sets the prompt message for the input.
    pub fn message(mut self, msg: &str) -> Self {
        self.message = msg.to_string();
        self
    }

    /// Sets a default value for the input.
    pub fn default(mut self, value: &str) -> Self {
        self.default = Some(value.to_string());
        self
    }

    /// Disables tips for the input.
    pub fn disable_tips(mut self) -> Self {
        self.tips = false;
        self
    }

    /// Enables multiline input mode.
    pub fn multiline(mut self) -> Self {
        self.multiline = true;
        self
    }

    /// Gets the input from the user as a String.
    pub fn get_input(&self) -> String {
        if self.multiline {
            self.get_multiline_input()
        } else {
            self.get_single_line_input()
        }
    }

    /// Gets the input from the user and parses it to the specified type.
    pub fn get_parsed_input<T: FromStr>(&self) -> T
    where
        <T as FromStr>::Err: std::fmt::Debug,
    {
        loop {
            let input = self.get_input();
            match input.parse() {
                Ok(value) => return value,
                Err(_) => {
                    eprintln!("Invalid input. Please try again.");
                    continue;
                }
            }
        }
    }

    fn get_single_line_input(&self) -> String {
        print!("{}", self.message);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input.is_empty() && self.default.is_some() {
            self.default.as_ref().unwrap().clone()
        } else {
            input
        }
    }

    fn get_multiline_input(&self) -> String {
        println!("{}", self.message);
        if self.tips {
            print!("(Press Enter, then Ctrl+D to send)");
            #[cfg(feature = "editor")]
            print!(" (In a blank line type @e, then press Enter to edit in external editor)");
            println!("");
        }

        let mut input = String::new();
        loop {
            let mut line = String::new();
            let bytes_read = io::stdin().read_line(&mut line);

            match bytes_read {
                Ok(0) => break, // EOF (Ctrl-D)
                Ok(_) => {
                    #[cfg(feature = "editor")]
                    if line.trim() == "@e" {
                        match super::ZenConsoleInput::edit_in_external_editor(&input) {
                            Ok(edited_content) => {
                                input = edited_content;
                                break;
                            }
                            Err(e) => {
                                println!("Error using external editor: {}", e);
                            }
                        }
                    }
                    input.push_str(&line);
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
        }

        if input.trim().is_empty() && self.default.is_some() {
            self.default.as_ref().unwrap().clone()
        } else {
            input.trim().to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_with_default() {
        let input = Input::new().message("Enter your name").default("John Doe");
        assert!(input.message.contains("Enter your name"));
        assert_eq!(input.default, Some("John Doe".to_string()));
    }

    #[test]
    fn test_multiline_input() {
        let input = Input::new().message("Enter your bio").multiline();
        assert!(input.multiline);
    }
}
