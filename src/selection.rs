//! Provides functionality for getting a selection from a list of options.

use std::io::{self, Write};

/// Opinionated struct for handling selection input from the user.
pub struct Selection {
    message: String,
    options: Vec<String>,
}

impl Selection {
    pub(crate) fn new() -> Self {
        Selection {
            message: String::new(),
            options: Vec::new(),
        }
    }

    /// Sets the prompt message for the selection.
    pub fn message(mut self, msg: &str) -> Self {
        self.message = msg.to_string();
        self
    }

    /// Sets the list of options for the selection.
    pub fn options(mut self, opts: Vec<String>) -> Self {
        self.options = opts;
        self
    }

    /// Gets the selected option from the user.
    pub fn get_selection(&self) -> String {
        println!("{}", self.message);
        for (i, option) in self.options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        loop {
            print!("Enter your choice (1-{}): ", self.options.len());
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            if let Ok(choice) = input.parse::<usize>() {
                if choice > 0 && choice <= self.options.len() {
                    return self.options[choice - 1].clone();
                }
            }

            println!("Invalid choice. Please try again.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_options() {
        let selection = Selection::new().message("Choose a color").options(vec![
            "Red".to_string(),
            "Green".to_string(),
            "Blue".to_string(),
        ]);
        assert_eq!(selection.message, "Choose a color");
        assert_eq!(selection.options, vec!["Red", "Green", "Blue"]);
    }
}
