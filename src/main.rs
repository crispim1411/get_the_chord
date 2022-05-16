use get_the_chord::note::{Symbol::*, Accidental::*};
use get_the_chord::note::Note;
use get_the_chord::notes_to_chord;

fn main() {
    let notes = vec![
        Note::new(C, Normal),
        Note::new(E, Normal),
        Note::new(G, Normal),
        Note::new(B, Flat),
    ];

    let chord = notes_to_chord(notes);
    
    println!("Chord: {}", chord);
}
