use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::env;
use rand::prelude::*;

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
//TODO: Allow multiple entries to be returned if multiple are matching?
//TODO: Add --list to list all possibilities for category and keyword
fn main() {
    let mut category: String = String::new();
    let mut keyword: String = String::new();
    let mut colors: bool = true;
    let mut random: bool = false;
    let mut list: bool = false;

    let mut arg_counter: u8 = 0;
    let mut args = env::args();
    args.next();
    for mut argument in args{
        if argument.starts_with("--"){
            argument.drain(..2);
            match argument.as_str() {
                "help" => {
                    print!("General usage: dict <category> <keyword>\n\
                        Options:\n\
                            \t--help          show this menu\n\
                            \t--no-colors     print output text without colors\n\
                            \t--list          list all categories, if given a category list keywords for that category\n\
                            \t--random        show a random entry for a given category");
                    return;
                }
                "list" => {
                    list = true;
                }
                "random" => {
                    random = true;
                }
                "no-colors" => {
                    colors = false;
                }
                _ => {
                    print!("Invalid argument: {0}", argument);
                    return;
                }
            }
        } else {
            match arg_counter {
                0 => category = argument,
                1 => keyword = argument,
                _ => {
                    print!("Argument count exceeds maximum");
                    return;
                }
            }
            arg_counter+=1;
        }
    }
    if (category.is_empty() || keyword.is_empty()) && !random {
        print!("Missing argument\nUsage: dict <category> <keyword>");
        return;
    }
    //TODO: make ~ work and use it to make location more universal
    // let home_path  = env::home_dir().map(|p| {p.to_str()}).unwrap().unwrap();
    let home_path = "/home/flavius/".to_owned();
    let dictionaries_location: String = home_path + "projects/scripts/dictionaries/";
    let dictionary = dictionaries_location + &category;
    let mut file = File::open(dictionary).unwrap_or_else(|error|{
        panic!("Unable to open dictionary:\n{error}");
    });
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let dictionary_data:Dictionary = serde_json::from_str::<Dictionary>(&data).expect("JSON not well-formatted");

    let entry: Option<&Vocabulary>;
    if random {
        let mut rng = rand::rng();
        let random_index = rng.random_range(0..dictionary_data.entries.len());
        entry = dictionary_data.entries.get(random_index);
    } else {
        entry = dictionary_data.entries.iter().find(|&v| {
            v.french == keyword || v.english == keyword || v.spanish == keyword
        });
    }

    let mut white = "";
    let mut green = "";
    let mut blue = "";
    let mut yellow = "";
    let mut red = "";
    let mut gray = "";
    let mut bold = "";
    if colors {
        white = "\x1b[0m";
        green = "\x1b[92m";
        blue = "\x1b[94m";
        yellow = "\x1b[93m";
        red = "\x1b[91m";
        gray = "\x1b[90m";
        bold = "\x1b[97m";
    }
    match entry {
        Some(e) => {
            if !e.spanish.is_empty() {
                println!("{1}{0}{2}", e.spanish, yellow, white);
            }
            if !e.english.is_empty() {
                println!("{1}{0}{2}", e.english, green, white);
            }
            if !e.french.is_empty() {
                println!("{1}{0}{2}", e.french, blue, white);
            }
            if !e.definition.is_empty() {
                println!("Definition: {0}", e.definition);
            }
            if !e.word_type.is_empty() {
                println!("{1}Type: {0}{2}", e.word_type, gray, white);
            }
            if !e.notes.is_empty() {
                println!("{1}Notes: {0}{2}", e.notes, bold, white);
            }
            if !e.irregularities.is_empty() {
                println!("{1}Irregularities:{2} {0}", e.irregularities, red, white);
            }
            if !e.related.is_empty() {
                println!("Related: {0}", e.related);
            }
            if !e.category.is_empty() {
                println!("{1}Category: {0}\x1b[0m", e.category, gray);
            }
        }
        None => println!("No entry found matching keywords")
    }
}
