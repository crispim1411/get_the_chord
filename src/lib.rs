use note::Note;
use scale::Scale;

pub mod note;
pub mod scale;

pub fn notes_to_chord(notes: Vec<Note>) -> String {
    let scale = Scale::new(notes[0].clone());
    let intervals = scale.get_intervals(notes);
    
    scale.to_chord(intervals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::{Symbol::*, Accidental::*};

    #[test]
    fn get_triad_major_chord_test() {
        let notes = vec![
            Note::new(G, Sharp),
            Note::new(C, Normal),
            Note::new(D, Sharp)
        ];

        let chord = notes_to_chord(notes);
        
        assert_eq!("G#", chord);
    }

    #[test]
    fn get_triad_minor_chord_test() {
        let notes = vec![
            Note::new(E, Flat),
            Note::new(G, Flat),
            Note::new(B, Flat)
        ];

        let chord = notes_to_chord(notes);
        
        assert_eq!("Ebm", chord);
    }

    #[test]
    fn get_tetrad_major_chord_test() {
        let notes = vec![
            Note::new(A, Sharp),
            Note::new(D, Normal),
            Note::new(F, Normal),
            Note::new(A, Normal)
        ];

        let chord = notes_to_chord(notes);
        
        assert_eq!("A#maj7", chord);
    }

    #[test]
    fn get_tetrad_minor_chord_test() {
        let notes = vec![
            Note::new(B, Flat),
            Note::new(F, Normal),
            Note::new(D, Flat),
            Note::new(A, Flat)
        ];

        let chord = notes_to_chord(notes);
        
        assert_eq!("Bbm7", chord);
    }
}