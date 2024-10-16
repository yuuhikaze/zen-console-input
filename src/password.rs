//! Provides functionality for getting password input from the user.

use rpassword::read_password;
use std::io::{self, Write};

/// Struct for handling password input from the user.
pub struct Password {
    message: String,
}

impl Password {
    pub(crate) fn new() -> Self {
        Password {
            message: String::new(),
        }
    }

    /// Sets the prompt message for the password input.
    pub fn message(mut self, msg: &str) -> Self {
        self.message = msg.to_string();
        self
    }

    /// Gets the password input from the user.
    pub fn get_password(&self) -> String {
        loop {
            print!("{}", self.message);
            io::stdout().flush().unwrap();
            match read_password() {
                Ok(value) => return value,
                Err(e) => {
                    eprintln!("Error reading password: {}", e);
                    continue;
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_prompt() {
        let password = Password::new().message("Enter your password");
        assert_eq!(password.message, "Enter your password");
    }
}
