use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Word {
    pub(crate) left: String,
    pub(crate) right: String,
    pub(crate) notes: Option<String>,
}

pub(crate) fn load_words(file_path: &str) -> Vec<Word> {
    let mut rdr = csv::Reader::from_path(file_path).expect("failed to load data file");
    rdr.deserialize()
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse words")
}
