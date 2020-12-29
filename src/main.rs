use std::time::{Duration, Instant};

use rand::seq::SliceRandom;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Specify the CSV quiz data to load
    quiz_data_file: String,

    /// Flip the language to use as the prompt
    #[structopt(short, long)]
    flip: bool,

    /// Enable shuffling of the questions
    #[structopt(short, long)]
    shuffle: bool,

    /// Set the time limit for the quiz in seconds
    #[structopt(short, long, default_value = "120")]
    time_limit: u64,
}

#[derive(Debug, Deserialize)]
struct Word {
    left: String,
    right: String,
    notes: Option<String>,
}

fn main() {
    let opt = Opt::from_args();
    ui::show_banner();

    let timeout = Duration::from_secs(opt.time_limit);
    let words = load_words(&opt.quiz_data_file);

    let game = Game::new(&words, timeout, opt.flip, opt.shuffle);

    game.run();
}

struct Game<'a> {
    words: &'a [Word],
    timeout: Duration,
    flip: bool,
    shuffle: bool,
    start_time: Instant,
}

impl<'a> Game<'a> {
    fn new(words: &'a [Word], timeout: Duration, flip: bool, shuffle: bool) -> Self {
        Game {
            words,
            timeout,
            flip,
            shuffle,
            start_time: Instant::now(),
        }
    }

    fn run(self) {
        let mut num_mistakes: i32 = 0;
        let mut num_words: i32 = 0;
        let word_order = self.word_order();

        for i in word_order {
            if self.is_game_over() {
                ui::on_game_over(num_words, num_mistakes);
                break;
            }

            let mut buffer = String::new();
            let mut attempts: i32 = 0;
            num_words += 1;

            let (prompt, answer, notes) = self.get_question(i);

            while answer != buffer {
                attempts += 1;
                if attempts > 1 {
                    ui::on_wrong_answer(answer);

                    if attempts > 5 {
                        break;
                    }
                }

                ui::on_start_question(
                    prompt,
                    self.time_remaining(),
                    num_words,
                    num_mistakes + attempts - 1,
                    notes,
                );

                buffer = ui::read_input();
            }

            num_mistakes += attempts - 1;

            ui::on_end_question();
        }
    }

    fn get_question(&self, index: usize) -> (&str, &str, Option<&str>) {
        if self.flip {
            (
                &self.words[index].left,
                &self.words[index].right,
                self.words[index].notes.as_deref(),
            )
        } else {
            (
                &self.words[index].right,
                &self.words[index].left,
                self.words[index].notes.as_deref(),
            )
        }
    }

    fn time_remaining(&self) -> Duration {
        self.timeout
            .checked_sub(self.start_time.elapsed())
            .unwrap_or_default()
    }

    fn is_game_over(&self) -> bool {
        self.start_time.elapsed() >= self.timeout
    }

    fn word_order(&self) -> Vec<usize> {
        let mut word_order: Vec<_> = (0..self.words.len()).collect();

        if self.shuffle {
            let mut rng = rand::thread_rng();
            word_order.shuffle(&mut rng);
        }

        word_order
    }
}

mod ui {
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
}

fn load_words(file_path: &str) -> Vec<Word> {
    let mut rdr = csv::Reader::from_path(file_path).expect("failed to load data file");
    rdr.deserialize()
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse words")
}
