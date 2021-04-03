use colored::Colorize;
use std::io;
use std::io::Write;

#[must_use]
pub fn one() -> String {
    let mut words = String::new();
    io::stdin().read_line(&mut words).ok();
    words
}

#[must_use]
pub fn yes_or_no(question: &str) -> bool {
    let state = loop {
        println!("    {}", question);
        print!("{}", "yes/no =>".bright_yellow().bold());
        io::stdout().flush().unwrap_or_default();
        let state = one().trim().to_uppercase();

        if state == *"YES" || state == *"NO" {
            break state;
        }

        println!("Please write either yes or no.")
    };
    matches!(state.as_str(), "YES")
}
