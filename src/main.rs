use std::io;
use std::io::prelude::*;
use std::time::{Duration, Instant};

use serde::Deserialize;
use rand::prelude::*;
use rand::seq::SliceRandom;
use trivial_colours::{Colour, Reset};

const TIMEOUT_SECS: u64 = 60;

// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
struct Word {
    word: String,
    english: String,
    gender: Option<String>,
}

fn main() {
    show_banner();

    let start_time = Instant::now();
    let timeout = Duration::from_secs(TIMEOUT_SECS);
    let words = load_words();
    let mut rng = rand::thread_rng();

    run(start_time, timeout, &mut rng, &words);
}

fn run(start_time: Instant, timeout: Duration, rng: &mut ThreadRng, words: &[Word]) {
    let mut num_mistakes: i32 = 0;
    let mut num_words: i32 = 0;
    let mut word_order: Vec<_> = (0..words.len()).collect();
    word_order.shuffle(rng);

    for i in word_order {
        if start_time.elapsed() >= timeout {
            println!("Time's up!");
            println!("{}{}{} words, {}{}{} mistakes",
                     Colour::Green,
                     num_words,
                     Reset,
                     Colour::Red,
                     num_mistakes,
                     Reset);
            break;
        }

        let mut buffer = String::new();
        let mut attempts: i32 = 0;
        num_words += 1;

        let swap = true;
        let (prompt, answer) = if swap {
            (&words[i].english, &words[i].word)
        } else {
            (&words[i].word, &words[i].english)
        };

        while answer != buffer.trim() {
            attempts += 1;
            if attempts > 2 {
                print!("{}Answer:{} {}\n", Colour::Red, Reset, answer);
                break
            }
            let time_remaining = timeout.checked_sub(start_time.elapsed())
                                        .unwrap_or_default();

            show_prompt(prompt,
                        time_remaining,
                        num_words,
                        num_mistakes + attempts - 1);

            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Error reading input");
        }

        num_mistakes += attempts - 1;
    }
}

fn show_prompt(word: &str, time_remaining: Duration, num_words: i32, num_mistakes: i32) {
    if num_mistakes > 0 { print!("{}", Colour::Red) };
    print!("{}{}/{} ", num_mistakes, Reset, num_words);
    print!("{}{}s{} ", Colour::Magenta, time_remaining.as_secs(), Reset);
    print!("{}{}{}", Colour::Cyan, word, Reset);
    println!();
    print!("> ");
    io::stdout().flush().expect("Error flushing stdout");
}

fn show_banner() {
    println!("{}{} v{}{}",
             Colour::Blue,
             env!("CARGO_PKG_NAME"),
             env!("CARGO_PKG_VERSION"),
             Reset);
}

fn load_words() -> Vec<Word> {
    let mut rdr = csv::Reader::from_path("data/biglist.csv").expect("failed to load data file");
    rdr.deserialize().collect::<Result<Vec<_>, _>>().expect("failed to parse words")
}
