# gabc-parser
This is a Rust library to parse and analyze gabc, a typesetting language for Gregorian chant. It provides functions to parse a gabc file, represent and manipulate it as a Rust struct, and automatically convert to JSON and Lilypond.

## Using this library
To use this library, you will need to include it in a Rust project: \
(todo) \
If you're looking for a standalone program to convert gabc files to JSON or Lilypond, try gabc-converter, a simple command-line program that uses this library. (TODO) may be more mature options for Lilypond conversion.
use std::fs::File;
use std::io::Read;

## Limitations
This library is in a prototype stage and doesn't correctly process all gabc syntax. Common gabc features not yet supported include:
* Flat clefs (e.g. "cb2")
*
Auto-generated Lilypond output may require adjustments, especially to correct the transposition range. Lilypond conversion doesn't yet take gabc note shapes or rhythmic signs into account; this may be implemented in future versions. 

## Related work
* gabctk: A toolkit for gabc, including conversion to Lilypond, abc, midi, and others. Written in Python and documented only in French. <https://github.com/jperon/gabctk>
* gabc2mid: An earlier iteration of gabctk with midi conversion only. Written in Python and documented in French, with English documentation also available. <https://github.com/jperon/gabc2mid>
* gabc-to-ly: Conversion from gabc to Lilypond via a .csv file, which can be manually edited to add organ accompaniment chords. Written in Python. <https://github.com/ahinkley/gabc-to-ly>
* lygre: Conversion from gabc to Lilypond. Written in Ruby. <https://github.com/igneus/lygre>

## Gregorio and gabc
* <https://github.com/gregorio-project/gregorio>
* <http://gregorio-project.github.io/gabc/index.html>

## Lilypond
* <http://lilypond.org>
