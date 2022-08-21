use std::time::Duration;
use structopt::StructOpt;

mod domain;
mod game;
mod ui;

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

fn main() {
    let opt = Opt::from_args();
    ui::show_banner();

    let timeout = Duration::from_secs(opt.time_limit);
    let words = domain::load_words(&opt.quiz_data_file);

    let game = game::Game::new(&words, timeout, opt.flip, opt.shuffle);

    game.run();
}
