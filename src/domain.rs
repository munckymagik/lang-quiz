use serde::Deserialize;
use std::time::Instant;

#[derive(Debug, Deserialize)]
pub(crate) struct Word {
    pub(crate) left: String,
    pub(crate) right: String,
    pub(crate) notes: Option<String>,
}

pub(crate) struct Question<'a> {
    pub(crate) prompt: &'a str,
    pub(crate) answer: &'a str,
    pub(crate) notes: Option<&'a str>,
}

pub(crate) struct GameState {
    pub(crate) num_mistakes: i32,
    pub(crate) num_words: i32,
    pub(crate) start_time: Instant,
}

impl Default for GameState {
    fn default() -> Self {
        Self { num_mistakes: Default::default(), num_words: Default::default(), start_time: Instant::now() }
    }
}

pub(crate) fn load_words(file_path: &str) -> Vec<Word> {
    let mut rdr = csv::Reader::from_path(file_path).expect("failed to load data file");
    rdr.deserialize()
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse words")
}
