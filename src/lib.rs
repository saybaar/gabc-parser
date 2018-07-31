extern crate itertools;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use itertools::Itertools;
use pest::Parser;

//-----------------------------------------------------------------------
//Pest boilerplate from the book (https://pest-parser.github.io/book/)

const _GRAMMAR: &str = include_str!("gabc.pest");

#[derive(Parser)]
#[grammar = "gabc.pest"]
pub struct GABCParser;

//-----------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct Note<'a> {
    pub text: &'a str,
    pub music: Vec<&'a str>,
}

#[derive(Serialize)]
pub struct GabcFile<'a> {
    pub attributes: Vec<(&'a str, &'a str)>,
    pub notes: Vec<Note<'a>>,
}

///Pretty-print a parse output tree
pub fn print_rule_tree(rules: pest::iterators::Pairs<Rule>, tabs: usize) {
    for rule in rules {
        for _ in 0..tabs { print!("\t"); }
        print!("{:?}: {}\n", rule.as_rule(), rule.as_str());
        print_rule_tree(rule.into_inner(), tabs + 1)
    }
}

pub fn parse_to_struct(filename: &str) -> GabcFile {
    let parse_result = GABCParser::parse(Rule::file, &filename);
    let output: GabcFile;
    match parse_result {
        Err(e) => { println!("Parse error: {}", e);
                    std::process::exit(1); },
        Ok(pairs) => {
            //print_rule_tree(pairs.clone(), 0);
                       output = parsed_file_to_struct(pairs);}
    }
    output
}


fn gabc_to_absolute_pitch (gabc_pos: char, clef: &str) -> &str {
    assert!(gabc_pos >= 'a' && gabc_pos <= 'm');
    let ly_notes = vec!["a,", "b,", "c", "d", "e", "f", "g", "a", "b", "c'", "d'", "e'", "f'", "g'", "a'", "b'", "c''", "d''", "e''", "f''", "g''", "a'''"];
    let start_index = match clef {
        "c1" => 6,
        "c2" => 4,
        "c3" => 2,
        "c4" => 0, 
        "f1" => 9,
        "f2" => 7,
        "f3" => 5,
        "f4" => 3,
        x => panic!("invalid clef: {}", x),
    };
    let position = gabc_pos as usize - 'a' as usize;
    assert!(position < ly_notes.len());
    return ly_notes.get(position + start_index).unwrap();
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
