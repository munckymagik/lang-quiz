use std::io;
use std::io::prelude::*;
use std::time::Duration;
use trivial_colours::{Colour, Reset};

pub fn read_input() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input");
    buffer.trim().to_owned()
}

pub fn on_start_question(
    word: &str,
    time_remaining: Duration,
    num_words: i32,
    num_mistakes: i32,
    notes: Option<&str>,
) {
    if num_mistakes > 0 {
        print!("{}", Colour::Red)
    };
    print!("{}{}/{} ", num_mistakes, Reset, num_words);
    print!("{}{}s{} ", Colour::Magenta, time_remaining.as_secs(), Reset);
    print!("{}{}", Colour::Cyan, word);
    if let Some(notes) = notes {
        print!(" ({})", notes);
    }
    print!("{}", Reset);
    println!();
    print!("> ");
    io::stdout().flush().expect("Error flushing stdout");
}

pub fn on_game_over(num_words: i32, num_mistakes: i32) {
    println!("Time's up!");
    println!(
        "{}{}{} words, {}{}{} mistakes",
        Colour::Green,
        num_words,
        Reset,
        Colour::Red,
        num_mistakes,
        Reset
    );
}

pub fn on_wrong_answer(answer: &str) {
    print!("{}Try again:{} {}\n", Colour::Red, Reset, answer);
}

pub fn on_end_question() {
    println!();
}

pub fn show_banner() {
    println!(
        "{}{} v{}{}",
        Colour::Blue,
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        Reset
    );
}
