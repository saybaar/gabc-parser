# gabc-parser
gabc-parser is a Rust library to parse and analyze gabc, a typesetting language for Gregorian chant. It provides functions to parse a gabc file, represent and manipulate it as a Rust struct, and automatically convert to JSON and Lilypond.

## Using this library
To use this library, you will need to include it in a Rust project's cargo.toml:
```
[dependencies]
gabc-parser = "0.1.0"
```
If you're looking for a standalone program to convert gabc files to JSON or Lilypond, try [gabc-converter](https://github.com/saybaar/gabc-converter), a simple command-line program that uses this library. [gabctk](https://github.com/jperon/gabctk) is another good option for Lilypond conversion.

## Examples
The gabc files in /examples are from [gregobase](https://gregobase.selapa.net/), except for populus_sion.gabc, which is the canonical example from [the gabc documentation](http://gregorio-project.github.io/gabc/details.html).

## Limitations
This library is under development and doesn't yet recognize all gabc syntax. Major gabc features not yet supported include:
* Accidentals and flat clefs (e.g. "cb2")
* gabc comments
* Text above or below the staff
Auto-generated Lilypond output may require adjustments, especially to the transposition range (which is c -> c' by default) or to correct formatting and alignment of lyrics.  

## Related work
* [gabctk](https://github.com/jperon/gabctk): A toolkit for gabc, including conversion to Lilypond, abc, midi, and others. Written in Python and documented (only) in French.
* [gabc2mid](https://github.com/jperon/gabc2mid): An earlier iteration of gabctk with midi conversion only. Written in Python with English documentation available.
* [gabc-to-ly](https://github.com/ahinkley/gabc-to-ly): Conversion from gabc to Lilypond via a .csv file, which can be manually edited to add organ accompaniment chords. Written in Python.
* [lygre](https://github.com/igneus/lygre): Conversion from gabc to Lilypond. Written in Ruby.

## Gregorio and gabc
* [Gregorio on GitHub](https://github.com/gregorio-project/gregorio)
* [gabc documentation](http://gregorio-project.github.io/gabc/index.html)

## Lilypond
* [Lilypond website](http://lilypond.org)
_______________
Copyright (c) 2018 Lydia Simmons  
This software is licensed under the GNU General Public License v3.0. See the LICENSE file in this distribution for license terms.
