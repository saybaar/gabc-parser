# gabc-parser
gabc-parser is a Rust library to parse and analyze gabc, a typesetting language for Gregorian chant. It provides functions to parse a gabc file, represent and manipulate it as a Rust struct, and automatically convert to JSON and Lilypond.  

Documentation is available at [docs.rs](https://docs.rs/gabc-parser).

## Using this library
To use this library, you will need to include it in a Rust project's Cargo.toml:
```
[dependencies]
gabc-parser = "0.1.1"
```
If you're looking for a standalone program to convert gabc files to JSON or Lilypond, try [gabc-converter](https://github.com/saybaar/gabc-converter), a simple command-line program that uses this library. [gabctk](https://github.com/jperon/gabctk) is another good option for Lilypond conversion.

## Local development
To run the library locally, clone this repository with `git clone https://github.com/saybaar/gabc-parser.git`. To use the local version in another Rust project, use the following in Cargo.toml:
```
[dependencies]
gabc-parser = { path = "<path to local gabc-parser>" }
```
In the gabc-parser directory, run `cargo build` to build the current version (which will then be used by any Rust project that refers to the local library) and `cargo test` to run the tests.

## Example gabc files
The gabc files in /examples should all play nicely with this library. populus_sion.gabc is the canonical example in [the gabc documentation](http://gregorio-project.github.io/gabc/details.html), and the other examples are from [gregobase](https://gregobase.selapa.net/).

## Limitations
This library is under development and doesn't yet recognize all gabc syntax. Major gabc features not yet supported include:
* Accidentals and flat clefs (e.g. "cb2")
* gabc comments
* Text above or below the staff

Auto-generated Lilypond may require adjustments, especially to the transposition range (which is c -> c' by default) or to correct formatting and alignment of lyrics.  

## Resources
### Other gabc tools
* [gabctk](https://github.com/jperon/gabctk): A toolkit for gabc, including conversion to Lilypond, abc, midi, and others. Written in Python and documented (only) in French.
* [gabc2mid](https://github.com/jperon/gabc2mid): An earlier iteration of gabctk with midi conversion only. Written in Python with English documentation available.
* [gabc-to-ly](https://github.com/ahinkley/gabc-to-ly): Conversion from gabc to Lilypond via a .csv file, which can be manually edited to add organ accompaniment chords. Written in Python.
* [lygre](https://github.com/igneus/lygre): Conversion from gabc to Lilypond. Written in Ruby.

### Gregorio and gabc
gabc is part of [the Gregorio project](http://gregorio-project.github.io/index.html), which also includes the GregorioTeX tool for rendering gabc. GregorioTeX can be [installed locally](http://gregorio-project.github.io/installation.html), which may be a complicated process. Web renderers like [run.gregoriochant.org](http://run.gregoriochant.org) are a simpler option.  
* [Gregorio on GitHub](https://github.com/gregorio-project/gregorio)
* [gabc documentation](http://gregorio-project.github.io/gabc/index.html)

### Lilypond
Lilypond files can be rendered with `lilypond file.ly` on a [local installation](http://lilypond.org/download.html), or through a web renderer like [lilybin](http://lilybin.com/).
* [Lilypond documentation](http://lilypond.org/manuals.html)
_______________
Copyright (c) 2018 Lydia Simmons  
This software is licensed under the GNU General Public License v3.0. See the LICENSE file in this distribution for license terms.
