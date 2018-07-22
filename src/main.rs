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

struct GabcFile<'a> {
    attributes: Vec<(&'a str, &'a str)>,
    notes: Vec<Note<'a>>,
}

fn main() {
    let first = env::args().nth(1).expect("Please supply a filename");
    let mut file = File::open(&first).expect("Error opening file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("Error reading file");
    //derived from example from the book:
    let parse_result = GABCParser::parse(Rule::file, &text);
    let output: GabcFile;
    match parse_result {
        Err(e) => { println!("Parse error: {}", e);
                    std::process::exit(1); },
        Ok(pairs) => { print_rule_tree(pairs.clone(), 0);
                       output = parsed_file_to_struct(pairs);}
    }
    for attribute in output.attributes {
        println!("Attribute: {:?}", attribute);
    }
    for note in output.notes {
        println!("Note: {:?}", note);
    }
}

///Pretty-print a parse output tree
fn print_rule_tree(rules: pest::iterators::Pairs<Rule>, tabs: usize) {
    for rule in rules {
        for _ in 0..tabs { print!("\t"); }
        print!("{:?}: {}\n", rule.as_rule(), rule.as_str());
        print_rule_tree(rule.into_inner(), tabs + 1)
    }
}

///Turns a parse result into a GabcFile. This relies on unchecked unwrap() calls that should not
///fail because of the characteristics of the pest PEG.
fn parsed_file_to_struct<'b>(mut parsed_file: pest::iterators::Pairs<'b, Rule>) -> GabcFile<'b> {
    let mut notes: Vec<Note> = Vec::new();
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    for pair in parsed_file.next().unwrap().into_inner() {
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
    GabcFile { attributes, notes }
}
