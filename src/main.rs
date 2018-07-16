extern crate itertools;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use itertools::Itertools;
use pest::Parser;
use std::env;
use std::fs::File;
use std::io::Read;

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
    music: Vec<&'a str>,
}

fn main() {
    let first = env::args().nth(1).expect("Please supply a filename");
    let mut file = File::open(&first).expect("Error opening file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Error reading file");
    //derived from example from the book:
    let parsed_file = GABCParser::parse(Rule::file, &text)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails
    let mut notes: Vec<Note> = Vec::new();
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    for pair in parsed_file.into_inner() {
        match pair.as_rule() {
            Rule::attribute => {
                let attribute: (&str, &str) = pair.into_inner().map(|x| x.as_str())
                                                                    .next_tuple().unwrap();
                attributes.push(attribute);
            },
            Rule::syllable => {
                let strings: Vec<&str> = pair.into_inner().map(|x| x.as_str()).collect();
                notes.push(Note { text: strings[0], music: strings[1..].to_vec() } );
            },
            _ => {}
        }
    }
    for attribute in attributes {
        println!("Attribute: {:?}", attribute);
    }
    for note in notes {
        println!("Note: {:?}", note);

    }
}
