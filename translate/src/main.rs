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
//TODO: Add options to output only certain fields so it can be easily piped to clipboard
//TODO: Add example field for Vocabulary
//TODO: Make a quiz/random word mode
fn main() {
    let args: Vec<String> = env::args().collect();
    //TODO: make ~ work and use it to make location more universal
    // let home_path  = env::home_dir().map(|p| {p.to_str()}).unwrap().unwrap();
    let home_path = "/home/flavius/".to_owned();
    let dictionaries_location: String = home_path + "projects/scripts/dictionaries/";
    let category: &str = args.get(1).expect("Missing argument\nUsage: dict <category> <keyword>").trim();
    let keyword: &str = args.get(2).expect("Missing argument\nUsage: dict <category> <keyword>").trim();
    let dictionary = dictionaries_location + category;
    let mut file = File::open(dictionary).unwrap_or_else(|error|{
        panic!("Unable to open dictionary:\n{error}");
    });
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let dictionary_data:Dictionary = serde_json::from_str::<Dictionary>(&data).expect("JSON was not well-formatted");

    let entry = dictionary_data.entries.iter().find(|&v| {
        v.french == keyword || v.english == keyword || v.spanish == keyword
    });

    match entry {
        Some(e) => {
            if !e.spanish.is_empty() {
                println!("\x1b[93m{0}\x1b[0m", e.spanish);
            }
            if !e.english.is_empty() {
                println!("\x1b[92m{0}\x1b[0m", e.english);
            }
            if !e.french.is_empty() {
                println!("\x1b[94m{0}\x1b[0m", e.french);
            }
            if !e.definition.is_empty() {
                println!("Definition: {0}\x1b[0m", e.definition);
            }
            if !e.word_type.is_empty() {
                println!("\x1b[90mType: {0}\x1b[0m", e.word_type);
            }
            if !e.notes.is_empty() {
                println!("\x1b[97mNotes: {0}\x1b[0m", e.notes);
            }
            if !e.irregularities.is_empty() {
                println!("\x1b[91mIrregularities:\x1b[0m {0}", e.irregularities);
            }
            if !e.related.is_empty() {
                println!("Related: {0}\x1b[0m", e.related);
            }
            if !e.category.is_empty() {
                println!("\x1b[90mCategory: {0}\x1b[0m", e.category);
            }

        }
        None => println!("No entry found matching keywords")
    }
}
