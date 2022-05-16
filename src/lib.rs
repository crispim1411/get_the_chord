use note::Note;
use scale::Scale;

pub mod note;
pub mod scale;

pub fn notes_to_chord(notes: Vec<Note>) -> String {
    let scale = Scale::new(notes[0].clone());
    let intervals = scale.get_intervals(notes);
    
    scale.to_chord(intervals)
}