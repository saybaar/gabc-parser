//Copyright (c) 2018 Lydia Simmons
//This software is licensed under the GNU General Public License v3.0.
//See the LICENSE file in this distribution for license terms.

extern crate gabc_parser;
extern crate pest;
use gabc_parser::*;
use pest::Parser;

static FILE: &'static str = "office-part:Tractus;
mode:8;
%%
(c3) Pó(eh/hi)pu(h)lus(h) Si(hi)on,(hgh.) *(;) ec(hihi)ce(e.) (::)";

static LYRICS: &'static str = " Pó -- pu -- lus Si -- on, \\set stanza = \" *\"  ec -- ce  ";

static NOTES: &'static str = "
g(c' c' d')
c'
c'
c'(d')
c'(b c')
\\divisioMaior
c'(d' c' d')
g
\\finalis
";

#[test]
fn new_file_works() {
    let g = GabcFile::new(FILE);
    assert_eq!("office-part", g.attributes[0].0);
    assert_eq!(2, g.attributes.len());
    assert_eq!(" Pó", g.syllables[1].text);
}

#[test]
fn test_file_text_and_lyrics() {
    let g = GabcFile::new(FILE);
    assert_eq!(g.ly_lyrics(), LYRICS);
    assert_eq!(g.ly_notes(), NOTES);
}

#[test]
fn test_absolute_pitch() {
    let note = Note::new("d", "c1");
    assert_eq!("c'", note.absolute_pitch());
}

#[test]
fn new_syllable_works() {
    let g = Syllable::new("Pó(eh/hi)", "c3");
    assert_eq!(g.text, "Pó");
    assert_eq!(g.music.len(), 5);
}

#[test]
fn test_syllable_to_ly() {
    let break_syllable = Syllable::new("*(;)", "c1");
    assert_eq!("\\divisioMaior", break_syllable.ly_notes());
    assert_eq!(" \\set stanza = \"*\" ", break_syllable.ly_text());
    let num_syllable = Syllable::new(" 3. Po(cde)", "c3");
    assert_eq!("e(f g)", num_syllable.ly_notes());
    assert_eq!(" \"3._Po\"", num_syllable.ly_text());
}

#[test]
fn test_raw_parsing() {
    let good_file = GABCParser::parse(Rule::file, FILE);
    assert!(good_file.is_ok());
    let bad_file = GABCParser::parse(Rule::file, "this is not a gabc file");
    assert!(bad_file.is_err());
    let good_syll = GABCParser::parse(Rule::syllable, "Pó(eh/hi)");
    assert!(good_syll.is_ok());
    let bad_syll = GABCParser::parse(Rule::syllable, "this is not a syllable");
    assert!(bad_syll.is_err());
    let good_note = GABCParser::parse(Rule::note, "e..");
    assert!(good_note.is_ok());
    let bad_note = GABCParser::parse(Rule::note, "this is not a note");
    assert!(bad_note.is_err());
}
