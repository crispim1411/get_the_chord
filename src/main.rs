use std::env;
use std::str::FromStr;

use get_the_chord::note::Note;
use get_the_chord::notes_to_chord;

fn main() {
    let notes: Vec<Note> = env::args().into_iter().skip(1)
                            .map(|i| Note::from_str(&i)
                            .unwrap())
                            .collect();

    let chord = notes_to_chord(notes);
    
    println!("Chord: {}", chord);
}
