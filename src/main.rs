use colored::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let message = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        "Hello from Rust!".to_string()
    };
    crab_says(&message);
}

fn crab_says(message: &str) {
    let crab = r#"
           \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
    "#;

    let mut max_line_length = 0;
    let mut lines: Vec<&str> = Vec::new();

    for line in message.split_whitespace() {
        max_line_length = max_line_length.max(line.chars().count());
        lines.push(line);
    }

    let bubble_width = max_line_length + 2;

    println!("{}", "_".repeat(bubble_width).red());
    for line in lines {
        println!(
            "| {}{} |",
            line.green(),
            " ".repeat(max_line_length - line.chars().count())
        );
    }
    println!("{}", "-".repeat(bubble_width).red());
    println!("{}", crab.red());
}
