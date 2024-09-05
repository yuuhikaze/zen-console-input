use std::{
    error::Error,
    io::{self, Write},
    str::FromStr,
};

pub struct ZenConsoleInput {
    prompt: Option<String>,
}

impl ZenConsoleInput {
    pub fn new() -> Self {
        Self { prompt: None }
    }

    /// Prompt a message to the user
    pub fn prompt(&self, prompt: &str) -> Self {
        Self {
            prompt: Some(prompt.to_string()),
        }
    }

    fn prompt_and_read_line(&self) -> String {
        if let Some(p) = &self.prompt {
            print!("{p}");
            io::stdout().flush().unwrap();
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    /// Read single value
    pub fn read_value<T, E>(&self) -> T
    where
        T: FromStr<Err = E>,
        E: Error + 'static,
    {
        match self.prompt_and_read_line().parse() {
            Ok(it) => it,
            Err(err) => {
                eprintln!("{err}");
                return self.read_value();
            }
        }
    }

    /// Read multiple inputs separated by whitespaces
    pub fn read_vector<T, E>(&self) -> Vec<T>
    where
        T: FromStr<Err = E>,
        E: Error + 'static,
    {
        self.prompt_and_read_line()
            .split_whitespace()
            .map(|x| match x.parse() {
                Ok(it) => Ok(it),
                Err(err) => {
                    eprintln!("{err}");
                    Err(err)
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_else(|_| self.read_vector())
    }
}
