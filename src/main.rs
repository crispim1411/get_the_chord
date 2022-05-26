use std::env;
use std::env::Args;
use std::str::FromStr;

use get_the_chord::note::Note;
use get_the_chord::notes_to_chord;

fn main() {
    let args = env::args(); 

    match get_chord(args) {
        Ok(chord) => println!("Chord: {}", chord),
        Err(error) => println!("ERROR: {}", error)
    }
}

fn read_args(args: Args) -> Result<Vec<Note>, String> {
    args
    .skip(1) // skip filename 
    .into_iter()
    .map(|i| Note::from_str(&i))
    .collect()
}

fn get_chord(args: Args) -> Result<String, String> {
    let notes = read_args(args)?;
    let chord = notes_to_chord(notes)?;
    Ok(chord)
}
