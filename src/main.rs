use std::env;

use get_the_chord::get_chord;

fn main() {
    let args = env::args(); 

    match get_chord(args) {
        Ok(chord) => println!("Chord: {}", chord),
        Err(error) => println!("{}", error)
    }
}