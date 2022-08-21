use std::io;
use std::io::prelude::*;
use std::time::Duration;
use trivial_colours::{Colour, Reset};

pub(crate) trait Ui {
    fn read_input(&self) -> String;
    fn start_question(
        &self,
        word: &str,
        time_remaining: Duration,
        num_words: i32,
        num_mistakes: i32,
        notes: Option<&str>,
    );
    fn game_over(&self, num_words: i32, num_mistakes: i32);
    fn wrong_answer(&self, answer: &str);
    fn end_question(&self);
    fn show_banner(&self);
}

pub(crate) struct TerminalUi;

impl Ui for TerminalUi {
    fn read_input(&self) -> String {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading input");
        buffer.trim().to_owned()
    }

    fn start_question(
        &self,
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

    fn game_over(&self, num_words: i32, num_mistakes: i32) {
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

    fn wrong_answer(&self, answer: &str) {
        print!("{}Try again:{} {}\n", Colour::Red, Reset, answer);
    }

    fn end_question(&self) {
        println!();
    }

    fn show_banner(&self) {
        println!(
            "{}{} v{}{}",
            Colour::Blue,
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            Reset
        );
    }
}
