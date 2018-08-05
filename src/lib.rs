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
    pub prefix: &'a str,
    pub pitch: char,
    pub suffix: &'a str,
}

#[derive(Debug, Serialize)]
pub struct Syllable<'a> {
    pub text: &'a str,
    pub music: Vec<Note<'a>>,
}

#[derive(Serialize)]
pub struct GabcFile<'a> {
    pub attributes: Vec<(&'a str, &'a str)>,
    pub syllables: Vec<Syllable<'a>>,
}

pub fn debug_print(filename: &str) -> String {
    print_rule_tree(GABCParser::parse(Rule::file, &filename).unwrap(), 0)
}

///Pretty-print a parse output tree
fn print_rule_tree(rules: pest::iterators::Pairs<Rule>, tabs: usize) -> String {
    let mut output = String::new();
    for rule in rules {
        for _ in 0..tabs { output.push_str("\t"); }
        output.push_str(format!("{:?}: {}\n", rule.as_rule(), rule.as_str()).as_ref());
        output.push_str(print_rule_tree(rule.into_inner(), tabs + 1).as_ref());
    }
    output
}

///Reads a filename and parses it into a GabcFile
pub fn parse_to_struct(filename: &str) -> GabcFile {
    let parse_result = GABCParser::parse(Rule::file, &filename);
    let output: GabcFile;
    match parse_result {
        Err(e) => { println!("Parse error: {}", e);
                    std::process::exit(1); },
        Ok(pairs) => { output = parsed_file_to_struct(pairs); }
    }
    output
}

///Reads a filename and parses it into JSON
pub fn parse_to_json(filename: &str) -> String {
    serde_json::to_string(&parse_to_struct(filename)).unwrap()
}

pub fn parse_to_lilypond(filename: &str) -> String {
    let full_struct = parse_to_struct(filename);
    let clef = "c1";
    let mut notes = String::new();
    let mut text = String::new();
    for syllable in &full_struct.syllables {
        text.push_str(syllable.text);
        text.push_str(" ");
        for tone in &syllable.music {
            notes.push_str(gabc_to_absolute_pitch(tone.pitch, clef));
            notes.push_str(" ");
        }
    }
    format!("{}{}{}{}{}", LY_1, notes, LY_2, text, LY_3)
}

///Transforms a gabc note position under a specific clef to an absolute musical pitch
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
    let mut syllables: Vec<Syllable> = Vec::new();
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    for pair in parsed_file.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::attribute => {
                let attribute: (&str, &str) = pair.into_inner().map(|x| x.as_str())
                                                                    .next_tuple().unwrap();
                attributes.push(attribute);
            },
            Rule::syllable => {
                let mut syllable_components = pair.into_inner();
                let text = syllable_components.next().unwrap().as_str();
                let mut music: Vec<Note> = Vec::new();
                while let Some(pair) = syllable_components.next() {
                    match pair.as_rule() {
                        Rule::note => {
                            let mut prefix = "";
                            let mut pitch = 'z';
                            let mut suffix = "";
                            for p in pair.into_inner() {
                                match &p.as_rule() {
                                    Rule::prefix => prefix = p.as_str(),
                                    Rule::position => pitch = p.as_str().chars().next().unwrap(),
                                    Rule::suffix => suffix = p.as_str(),
                                    _ => panic!("that's not part of a note!")
                                }
                            }
                            music.push(Note { prefix, pitch, suffix });
                        },
                        Rule::clef => {

                        },
                        _ => unreachable!("impossible syllable sub-rule"),
                    }
                }
                //let strings: Vec<&str> = pair.into_inner().map(|x| x.as_str()).collect();
                syllables.push(Syllable { text, music } ); //strings[1..].to_vec() } );
            },
            _ => {}
        }
    }
    GabcFile { attributes, syllables }
}

static LY_1: &'static str =
r#"\include "gregorian.ly"

chant = \relative c' {
  \set Score.timing = ##f
  "#;
  // f4 a2 \divisioMinima
  // g4 b a2 f2 \divisioMaior
  // g4( f) f( g) a2 \finalis
static LY_2: &'static str =
r#"
}

verba = \lyricmode {
  "#;
  // Lo -- rem ip -- sum do -- lor sit a -- met
static LY_3: &'static str =
r#"
}

\score {
  \new Staff <<
    \new Voice = "melody" \chant
    \new Lyrics = "one" \lyricsto melody \verba
  >>
  \layout {
    \context {
      \Staff
      \remove "Time_signature_engraver"
      \remove "Bar_engraver"
      \hide Stem
    }
    \context {
      \Voice
      \override Stem.length = #0
    }
    \context {
      \Score
      barAlways = ##t
    }
  }
}"#;
