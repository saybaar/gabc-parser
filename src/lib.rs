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

///Struct representing a gabc note.
#[derive(Debug, Serialize)]
pub struct Note<'a> {
    ///Entire prefix of the note (uncommon, only implemented here for "-" which indicates an initio
    ///debilis)
    pub prefix: &'a str,
    ///Main character of the note: its position in the gabc staff (a-m)
    pub position: char,
    ///Entire suffix of the note, including shape indicators and rhythmic signs, e.g. "V."
    pub suffix: &'a str,
    ///Clef governing this note in its original context
    pub current_clef: &'a str,
}

impl<'a> Note<'a> {
    ///Create a new note from well-formed gabc input.
    pub fn new<'b>(gabc_input: &'b str, current_clef: &'b str) -> Note<'b> {
        let mut parse_result = parse_gabc(gabc_input, Rule::note);
        parsed_note_to_struct(parse_result.next().unwrap(), current_clef)
    }
    ///Get the absolute pitch of this note in modern (Lilypond) notation, between a, and a'''.
    ///Assumes that the clef indicates middle C or the F above middle C.
    ///```
    ///# use gabc_parser::*;
    ///let n = Note::new("h..", "c1");
    ///assert_eq!(n.absolute_pitch(), "g'");
    ///```
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

///Any element that can appear in the music for a given syllable, including bars (e.g. ":"),
///separators (e.g. "/"), and Notes
#[derive(Debug, Serialize)]
pub enum NoteElem<'a> {
    Spacer(&'a str),
    Barline(&'a str),
    Note(Note<'a>),
}

impl<'a> NoteElem<'a> {
    ///Get the Lilypond representation of this note element. gabc spacers (e.g. "/") are ignored;
    ///Note suffixes (e.g. ".") that have Lilypond equivalents are not yet implemented.
    ///```
    ///# use gabc_parser::*;
    ///let ne = NoteElem::Note(Note::new("h..", "c1"));
    ///assert_eq!(ne.to_ly(), "g'");
    ///let s = NoteElem::Spacer("/");
    ///assert_eq!(s.to_ly(), "");
    ///let b = NoteElem::Barline(":");
    ///assert_eq!(b.to_ly(), "\\divisioMaior");
    ///```
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
            NoteElem::Spacer(_) => "",
        }
    }
}

///Struct representing a gabc syllable with text and music, e.g. "Po(eh/hi)"
#[derive(Debug, Serialize)]
pub struct Syllable<'a> {
    ///Text in this syllable, e.g. "Po"
    pub text: &'a str,
    ///Music in this syllable, e.g. "eh/hi", as a Vec of NoteElems
    pub music: Vec<NoteElem<'a>>,
}

impl<'a> Syllable<'a> {
    ///Create a new syllable from well-formed gabc input.
    pub fn new<'b>(gabc_input: &'b str, current_clef: &'b str) -> Syllable<'b> {
        let mut parse_result = parse_gabc(gabc_input, Rule::syllable);
        parsed_syllable_to_struct(parse_result.next().unwrap(), current_clef)
    }
    ///Translate this syllable's music string into a tied sequence of Lilypond notes.
    ///```
    ///# use gabc_parser::*;
    ///let s = Syllable::new("Po(eh/hi)", "c3");
    ///assert_eq!(s.ly_notes(), "g(c' c' d')");
    ///```
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
            let t = s.to_ly();
            if t.trim() != "" { result.push_str(" "); };
            result.push_str(t);
        }
        result.push_str(")");
        result
    }
    ///Translate this syllable's text into Lilypond lyrics. If there are no Notes in this
    ///syllable's music string, add "\set stanza = " to prevent Lilypond matching this text
    ///to a note.
    ///Translate this syllable's music string into a tied sequence of Lilypond notes.
    ///```
    ///# use gabc_parser::*;
    ///let s = Syllable::new("*()", "c3");
    ///assert_eq!(s.ly_text(), " \\set stanza = \"*\" ");
    ///```
    pub fn ly_text(&self) -> String {
        let mut flag = false;
        for ne in &self.music {
            if let NoteElem::Note(_) = ne {
                flag = true;
            }
        }
        if !flag && !(self.text.trim() == "") {
            return format!(" \\set stanza = \"{}\" ", &self.text);
        } else {
            return self.text.to_string();
        }
    }
}

///Struct representing an entire gabc file.
#[derive(Debug, Serialize)]
pub struct GabcFile<'a> {
    pub attributes: Vec<(&'a str, &'a str)>,
    pub syllables: Vec<Syllable<'a>>,
}

impl<'a> GabcFile<'a> {
    ///Create a new GabcFile from well-formed gabc input.
    pub fn new(gabc_input: &str) -> GabcFile {
        let parse_result = parse_gabc(gabc_input, Rule::file);
        parsed_file_to_struct(parse_result)
    }
    ///Translate this GabcFile into JSON.
    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
    ///Translate this GabcFile into a well-formed Lilypond file, by translating its text and music
    ///and inserting them into a template derived from
    ///<http://lilypond.org/doc/v2.18/Documentation/snippets/templates#templates-ancient-notation-template-_002d-modern-transcription-of-gregorian-music>
    pub fn as_lilypond(&self) -> String {
        let mut notes = String::new();
        for syllable in &self.syllables {
            notes.push_str(&syllable.ly_notes());
            notes.push_str("\n");
        }
        format!("{}{}{}{}{}", LY_1, notes, LY_2, &self.ly_lyrics(), LY_3)
    }
    ///Extract the text of this file into well-formed Lilypond lyrics, inserting " -- " to join
    ///syllables where appropriate.
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

///Parses a gabc file into pest's Pairs type. This is useful if you want to process the raw pairs
///using a mechanism other than the GabcFile struct.
pub fn parse_gabc(text: &str, rule: Rule) -> Pairs<Rule> {
    let parse_result = GABCParser::parse(rule, &text);
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

///Pretty string representation of a Pairs parse tree. Useful for directly debugging the output of
///parse_gabc_file().
pub fn debug_print(rules: Pairs<Rule>) -> String {
    print_rule_tree(rules, 0)
}

///Pretty-print parsed Pairs (recursive version).
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

///Turns a file parse result into a GabcFile. This relies on unchecked unwrap() calls that should not
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
                            music.push(NoteElem::Note(parsed_note_to_struct(pair, current_clef)));
                        }
                        Rule::barline => {
                            music.push(NoteElem::Barline(pair.as_str()));
                        }
                        Rule::spacer => {
                            music.push(NoteElem::Spacer(pair.as_str()));
                        }
                        Rule::clef => {
                            current_clef = pair.as_str();
                        }
                        _ => unreachable!("impossible syllable sub-rule"),
                    }
                }
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

///Turns a syllable parse result into a Syllable. This relies on unchecked unwrap() calls that should not
///fail because of the characteristics of the pest PEG.
///This isn't used in the main parse pipeline because it can't update the current_clef tracker,
///but it is used in Syllable::new().
fn parsed_syllable_to_struct<'a>(parsed_syllable: pest::iterators::Pair<'a, Rule>, current_clef: &'a str) -> Syllable<'a> {
    let mut syllable_components = parsed_syllable.into_inner();
    let text = syllable_components.next().unwrap().as_str();
    let mut music: Vec<NoteElem> = Vec::new();
    while let Some(pair) = syllable_components.next() {
        match pair.as_rule() {
            Rule::note => {
                music.push(NoteElem::Note(parsed_note_to_struct(pair, current_clef)));
            }
            Rule::barline => {
                music.push(NoteElem::Barline(pair.as_str()));
            }
            Rule::spacer => {
                music.push(NoteElem::Spacer(pair.as_str()));
            }
            _ => unreachable!("impossible syllable sub-rule"),
        }
    }
    Syllable { text, music }
}

///Turns a note parse result into a Note. This relies on unchecked unwrap() calls that should not
///fail because of the characteristics of the pest PEG.
fn parsed_note_to_struct<'b>(parsed_note: pest::iterators::Pair<'b, Rule>, current_clef: &'b str) -> Note<'b> {
        let mut prefix = "";
        let mut position = 'z';
        let mut suffix = "";
        for p in parsed_note.into_inner() {
            match &p.as_rule() {
                Rule::prefix => prefix = p.as_str(),
                Rule::position => position = p.as_str().chars().next().unwrap(),
                Rule::suffix => suffix = p.as_str(),
                _ => unreachable!("impossible note sub-rule"),
            }
        }
        assert!(position != 'z'); //note rule MUST have a position sub-rule
        Note {
            prefix,
            position,
            suffix,
            current_clef,
        }
}

static LY_1: &'static str = r#"\include "gregorian.ly"

chant = \absolute { \transpose c c' {
  \set Score.timing = ##f
  "#;
// f4 a2 \divisioMinima
// g4 b a2 f2 \divisioMaior
// g4( f) f( g) a2 \finalis
static LY_2: &'static str = r#"
}}

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
