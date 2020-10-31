use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

// a corresponding field in the CSV data's header record.
#[derive(Debug, Deserialize)]
struct Word {
    word: String,
    english: String,
    gender: Option<String>,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Word = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
