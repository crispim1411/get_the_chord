use note::Note;
use scale::Scale;

pub mod note;
pub mod scale;

pub fn notes_to_chord(notes: Vec<Note>) -> Result<String, String> {
    let scale = Scale::new(notes.clone());
    let intervals = scale.get_intervals(notes)?;
    let chord = scale.to_chord(intervals)?;
    Ok(chord)
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
        
        assert_eq!("G#", chord.unwrap());
    }

    #[test]
    fn get_triad_minor_chord_test() {
        let notes = vec![
            Note::new(E, Flat),
            Note::new(G, Flat),
            Note::new(B, Flat)
        ];

        let chord = notes_to_chord(notes);
        
        assert_eq!("Ebm", chord.unwrap());
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
        
        assert_eq!("A#maj7", chord.unwrap());
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
        
        assert_eq!("Bbm7", chord.unwrap());
    }

    #[test]
    fn get_inverted_major_chord_test() {
        // first inversion
        let notes = vec![
            Note::new(G, Normal),
            Note::new(B, Flat),
            Note::new(E, Flat)
        ];

        let chord = notes_to_chord(notes);
        
        assert_eq!("Eb/G", chord.unwrap());

        // second inversion
        let notes = vec![
            Note::new(B, Flat),
            Note::new(E, Flat),
            Note::new(G, Normal)
        ];

        let chord = notes_to_chord(notes);
        
        assert_eq!("Eb/Bb", chord.unwrap());
    }

    #[test]
    fn get_inverted_minor_chord_test() {
        // first inversion
        let notes = vec![
            Note::new(A, Flat),
            Note::new(C, Normal),
            Note::new(F, Normal)
        ];

        let chord = notes_to_chord(notes);
        
        assert_eq!("Fm/Ab", chord.unwrap());

        // second inversion
        let notes = vec![
            Note::new(C, Normal),
            Note::new(F, Normal),
            Note::new(A, Flat)
        ];

        let chord = notes_to_chord(notes);
        
        assert_eq!("Fm/C", chord.unwrap());
    }
}