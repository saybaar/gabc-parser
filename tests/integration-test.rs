//Copyright (c) 2018 Lydia Simmons
//This software is licensed under the GNU General Public License v3.0.
//See the LICENSE file in this distribution for license terms.

extern crate gabc_parser;
use gabc_parser::*;

static FILE: &'static str = "office-part:Tractus;
mode:8;
%%
(c3) Pó(eh/hi)pu(h)lus(h) Si(hi)on,(hgh.) *(;) ec(hihi)ce(e.) (::)";

#[test]
fn test_file() {
    let g = GabcFile::new(FILE);
    assert_eq!("office-part", g.attributes[0].0);
    assert_eq!(" Pó", g.syllables[1].text);
}

#[test]
fn test_absolute_pitch() {
    let note = Note::new("d", "c1");
    assert_eq!("c'", note.absolute_pitch());
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
