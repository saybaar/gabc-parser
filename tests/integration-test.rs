//Copyright (c) 2018 Lydia Simmons
//This software is licensed under the GNU General Public License v3.0.
//See the LICENSE file in this distribution for license terms.

extern crate gabc_parser;
use gabc_parser::*;
use std::fs::File;
use std::io::Read;

#[test]
fn test_absolute_pitch() {
    let note = Note {
        prefix: "",
        position: 'd',
        suffix: "",
        current_clef: "c1",
    };
    assert_eq!("c'", note.absolute_pitch());
}

#[test]
fn test_syllable_to_ly() {
    let syllable = Syllable {
        text: "*",
        music: vec![NoteElem::Barline(";")],
    };
    assert_eq!("\\divisioMaior", syllable.ly_notes());
}

#[test]
fn test_bar() {
    let mut f = File::open("./tests/bar.gabc").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).expect("error reading bar");
    let g = GabcFile::new(&s);
    assert_eq!("test", g.as_lilypond());
}
