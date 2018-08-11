//Copyright (c) 2018 Lydia Simmons
//This software is licensed under the GNU General Public License v3.0.
//See the LICENSE file in this distribution for license terms.

extern crate itertools;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use itertools::Itertools;
use pest::iterators::Pairs;
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
    pub position: char,
    pub suffix: &'a str,
    pub current_clef: &'a str,
}

impl<'a> Note<'a> {
    pub fn absolute_pitch(&self) -> &str {
        let ly_notes = vec![
            "a,", "b,", "c", "d", "e", "f", "g", "a", "b", "c'", "d'", "e'", "f'", "g'", "a'",
            "b'", "c''", "d''", "e''", "f''", "g''", "a'''",
        ];
        let start_index = match self.current_clef {
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
        let position = self.position.to_lowercase().next().unwrap() as usize - 'a' as usize;
        assert!(position < ly_notes.len());
        return ly_notes.get(position + start_index).unwrap();
    }
}

#[derive(Debug, Serialize)]
pub enum NoteElem<'a> {
    Separator(&'a str),
    Barline(&'a str),
    Note(Note<'a>),
}

impl<'a> NoteElem<'a> {
    pub fn to_ly(&self) -> &str {
        match self {
            NoteElem::Barline(s) => match *s {
                "'" => "\\divisioMinima",
                ";" => "\\divisioMaior",
                ":" => "\\divisioMaior",
                "::" => "\\finalis",
                _ => "\\divisioMinima",
            },
            NoteElem::Note(n) => n.absolute_pitch(),
            NoteElem::Separator(_) => "",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Syllable<'a> {
    pub text: &'a str,
    pub music: Vec<NoteElem<'a>>,
}

impl<'a> Syllable<'a> {
    pub fn ly_notes(&self) -> String {
        let mut result = String::new();
        let mut notes_iter = self.music.iter();
        match notes_iter.next() {
            None => return result,
            Some(s) => result.push_str(s.to_ly()),
        }
        match notes_iter.next() {
            None => return result,
            Some(s) => {
                result.push_str("(");
                result.push_str(s.to_ly());
            }
        }
        while let Some(s) = notes_iter.next() {
            result.push_str(" ");
            result.push_str(s.to_ly());
        }
        result.push_str(")");
        result
    }
    pub fn ly_text(&self) -> String {
        let mut flag = false;
        for ne in &self.music {
            if let NoteElem::Note(_) = ne {
                flag = true;
            }
        }
        if !flag {
            return format!(" \\set stanza = \"{}\" ", &self.text);
        } else {
            return self.text.to_string();
        }
    }
}

#[derive(Serialize)]
pub struct GabcFile<'a> {
    pub attributes: Vec<(&'a str, &'a str)>,
    pub syllables: Vec<Syllable<'a>>,
}

impl<'a> GabcFile<'a> {
    pub fn new(gabc_input: &str) -> GabcFile {
        let parse_result = parse_file(gabc_input);
        parsed_file_to_struct(parse_result)
    }
    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    pub fn as_lilypond(&self) -> String {
        let mut notes = String::new();
        for syllable in &self.syllables {
            notes.push_str(&syllable.ly_notes());
            notes.push_str("\n");
        }
        format!("{}{}{}{}{}", LY_1, notes, LY_2, &self.ly_lyrics(), LY_3)
    }
    pub fn ly_lyrics(&self) -> String {
        let mut result = String::new();
        let syllable_iter = &mut self.syllables.iter().peekable();
        while let Some(syll) = syllable_iter.next() {
            let s = &syll.ly_text();
            result.push_str(&s);
            if let Some(next_syll) = syllable_iter.peek() {
                let next_s = next_syll.ly_text();
                if s.trim_right() == s && next_s.trim_left() == next_s {
                    result.push_str(" -- ");
                }
            }
        }
        result
    }
}

//Parse a GABC file into pest's Pairs type
pub fn parse_file(text: &str) -> Pairs<Rule> {
    let parse_result = GABCParser::parse(Rule::file, &text);
    match parse_result {
        Err(e) => {
            println!("Parse error: {}", e);
            std::process::exit(1);
        }
        Ok(pairs) => {
            return pairs;
        }
    }
}

//Pretty-print a parse output tree
pub fn debug_print(rules: Pairs<Rule>) -> String {
    print_rule_tree(rules, 0)
}

///Pretty-print a parse output tree (recursive version)
fn print_rule_tree(rules: Pairs<Rule>, tabs: usize) -> String {
    let mut output = String::new();
    for rule in rules {
        for _ in 0..tabs {
            output.push_str("\t");
        }
        output.push_str(format!("{:?}: {}\n", rule.as_rule(), rule.as_str()).as_ref());
        output.push_str(print_rule_tree(rule.into_inner(), tabs + 1).as_ref());
    }
    output
}

///Turns a parse result into a GabcFile. This relies on unchecked unwrap() calls that should not
///fail because of the characteristics of the pest PEG.
fn parsed_file_to_struct<'b>(mut parsed_file: pest::iterators::Pairs<'b, Rule>) -> GabcFile<'b> {
    let mut syllables: Vec<Syllable> = Vec::new();
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    let mut current_clef = "no clef set";
    for pair in parsed_file.next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::attribute => {
                let attribute: (&str, &str) =
                    pair.into_inner().map(|x| x.as_str()).next_tuple().unwrap();
                attributes.push(attribute);
            }
            Rule::syllable => {
                let mut syllable_components = pair.into_inner();
                let text = syllable_components.next().unwrap().as_str();
                let mut music: Vec<NoteElem> = Vec::new();
                while let Some(pair) = syllable_components.next() {
                    match pair.as_rule() {
                        Rule::note => {
                            let mut prefix = "";
                            let mut position = 'z';
                            let mut suffix = "";
                            for p in pair.into_inner() {
                                match &p.as_rule() {
                                    Rule::prefix => prefix = p.as_str(),
                                    Rule::position => position = p.as_str().chars().next().unwrap(),
                                    Rule::suffix => suffix = p.as_str(),
                                    _ => unreachable!("impossible note sub-rule"),
                                }
                            }
                            assert!(position != 'z'); //note rule MUST have a position sub-rule
                            music.push(NoteElem::Note(Note {
                                prefix,
                                position,
                                suffix,
                                current_clef,
                            }));
                        }
                        Rule::barline => {
                            music.push(NoteElem::Barline(pair.as_str()));
                        }
                        Rule::separator => {
                            music.push(NoteElem::Separator(pair.as_str()));
                        }
                        Rule::clef => {
                            current_clef = pair.as_str();
                        }
                        _ => unreachable!("impossible syllable sub-rule"),
                    }
                }
                //let strings: Vec<&str> = pair.into_inner().map(|x| x.as_str()).collect();
                syllables.push(Syllable { text, music }); //strings[1..].to_vec() } );
            }
            _ => {}
        }
    }
    GabcFile {
        attributes,
        syllables,
    }
}

static LY_1: &'static str = r#"\include "gregorian.ly"

chant = \absolute {
  \set Score.timing = ##f
  "#;
// f4 a2 \divisioMinima
// g4 b a2 f2 \divisioMaior
// g4( f) f( g) a2 \finalis
static LY_2: &'static str = r#"
}

verba = \lyricmode {
  "#;
// Lo -- rem ip -- sum do -- lor sit a -- met
static LY_3: &'static str = r#"
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
