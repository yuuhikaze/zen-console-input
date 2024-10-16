# ZenConsoleInput

ZenConsoleInput is a simple, cross-platform console input library for Rust. It provides an easy-to-use interface for getting various types of user input from the console, including single-line input, multiline input, password input, and selection from a list of options.

## Features

-   Single-line and multiline text input
-   Password input (with masked characters)
-   Selection from a list of options
-   Default values for input
-   Optional external editor support for multiline input (feature-gated)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
zen_console_input = "0.1.0"
```

If you want to use the external editor feature, add this instead:

```toml
[dependencies]
zen_console_input = { version = "0.1.0", features = ["editor"] }
```

## Usage

Here's a quick example of how to use ZenConsoleInput:

```rust
use zen_console_input::ZenConsoleInput;

fn main() {
    let zci = ZenConsoleInput::new();

    // Simple input
    let name = zci.input().message("Enter your name: ").get_input();
    println!("Hello, {}!", name);

    // Input with default value
    let age: u32 = zci
        .input()
        .message("Enter your age [19]: ")
        .default("19")
        .get_parsed_input();
    println!("You are {} years old.", age);

    // Multiline input (with optional editor support)
    let bio = zci
        .input()
        .message("Enter your bio")
        .multiline()
        .get_input();
    println!("Your bio:\n{}", bio);

    // Selection
    let favorite_color = zci
        .selection()
        .message("Choose your favorite color")
        .options(vec![
            "Red".to_string(),
            "Green".to_string(),
            "Blue".to_string(),
        ])
        .get_selection();
    println!("Your favorite color is {}", favorite_color);

    // Password
    let password = zci
        .password()
        .message("Enter your password: ")
        .get_password();
    println!("Your password is {} characters long", password.len());
}
```

The code provided above is already included as an example, and can be executed with `cargo run --example demo` or `cargo run --example demo --features="editor"`.

## External Editor Support

If you've enabled the `editor` feature, you can use an external editor for multiline input. When in multiline input mode, type `@e` on a blank line and press Enter to open your default text editor (set by the `EDITOR` environment variable, or `vi` if not set).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
