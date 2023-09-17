use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Dictionary {
    entries: Vec<Vocabulary>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Vocabulary {
    spanish: String,
    english: String,
    french: String,
    definition: String,
    notes: String,
    irregularities: String,
    related: String,
    category: String,
    word_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Entry {
    name: String,
    definition: String,
    keywords: Vec<String>,
    related: String,
    category: String,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file = args.get(1).and_then(|arg1| File::open(arg1.trim()).ok());
    let mut data = String::new();
    match file {
        Some(mut f) => {
            f.read_to_string(&mut data).unwrap();
        }
        None => println!("incorrect argument")
    }
    let dict: Dictionary = serde_json::from_str(&data).expect("JSON was not well-formatted");
    println!("{:?}", dict);
    let dictionary = "/home/flavius/projects/scripts/dictionaries/spanish/vocabulary.json"; // get package to make ~ work and use it to make location more universal
    // let dictionary = "vocabulary.json";
    let mut file = File::open(dictionary).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    // println!("{:?}", data);
    let dict: Dictionary = serde_json::from_str(&data).expect("JSON was not well-formatted");
    println!("{:?}", dict);
}