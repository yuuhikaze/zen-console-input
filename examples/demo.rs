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
