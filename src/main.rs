extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::env;
use std::fs::File;
use std::io::Read;
use pest::Parser;

//-----------------------------------------------------------------------
//Pest boilerplate from the book (https://pest-parser.github.io/book/)

const _GRAMMAR: &str = include_str!("gabc.pest");

#[derive(Parser)]
#[grammar = "gabc.pest"]
pub struct GABCParser;

//-----------------------------------------------------------------------

#[derive(Debug)]
struct Note<'a> {
    text: &'a str,
    music: &'a str,
}

fn main() {
    let first = env::args().nth(1).expect("Please supply a filename");
    let mut file = File::open(&first).expect("Error opening file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Error reading file");
    println!("Processing string: {}", text);
    println!("{:?}", text_to_notes(&text.trim()));
}

//Processes (clumsily) a string like "text(music)text2(music2)..." into a Vec<Note>
fn text_to_notes<'b>(text: &'b str) -> Vec<Note<'b>> {
    let splits = text.split(')').filter(|s| s != &"");
    let mut v: Vec<Note> = Vec::new();
    for st in splits {
        let a: Vec<&str> = st.split('(').collect();
        let n: Note = Note {
            text: a[0],
            music: a[1]
        };
        v.push(n);
    }
    v
}
