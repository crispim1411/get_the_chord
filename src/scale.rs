use std::collections::HashMap;

use crate::Chord;
use crate::note::Accidental::*;
use crate::note::Note;
use crate::CustomError;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntervalType {
    Major,
    Minor,
    Perfect,
    Augmented,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Interval {
    Tonic,
    Second(IntervalType),
    Third(IntervalType),
    Fourth(IntervalType),
    Fifth(IntervalType),
    Sixth(IntervalType),
    Seventh(IntervalType),
}

fn get_chord_shapes() -> HashMap<Vec<Interval>, Chord> {
    use Interval::*;
    use IntervalType::*;

    vec![
        // acordes maiores
        (vec![Tonic, Third(Major), Fifth(Perfect)], "".to_string()),
        (vec![Tonic, Third(Major), Fifth(Augmented)], "aug".to_string()),
        (vec![Tonic, Third(Major), Fifth(Perfect), Sixth(Major)], "6".to_string()),
        (vec![Tonic, Third(Major), Fifth(Perfect), Seventh(Major)], "maj7".to_string()),
        (vec![Tonic, Third(Major), Fifth(Augmented), Seventh(Major)], "augM7".to_string()),
        (vec![Tonic, Third(Major), Fifth(Perfect), Seventh(Minor)], "7".to_string()),
        (vec![Tonic, Third(Major), Fifth(Augmented), Seventh(Minor)], "aug7".to_string()),
        // acordes menores
        (vec![Tonic, Third(Minor), Fifth(Perfect)], "m".to_string()),
        (vec![Tonic, Third(Minor), Sixth(Major)], "m6".to_string()),
        (vec![Tonic, Third(Minor), Fifth(Perfect), Sixth(Major)], "m6".to_string()),
        (vec![Tonic, Third(Minor), Fifth(Perfect), Seventh(Minor)], "m7".to_string()),
        (vec![Tonic, Third(Minor), Fifth(Perfect), Seventh(Major)], "m(maj7)".to_string()),
        (vec![Tonic, Third(Minor), Fourth(Augmented), Seventh(Minor)], "m7(b5)".to_string()),
        (vec![Tonic, Third(Minor), Fourth(Augmented)], "dim".to_string()),
        (vec![Tonic, Third(Minor), Fourth(Augmented), Sixth(Major)], "dim7".to_string()),
        // acordes suspensos
        (vec![Tonic, Second(Major), Fifth(Perfect)], "sus2".to_string()),
        (vec![Tonic, Fourth(Perfect), Fifth(Perfect)], "sus4".to_string()),
        // inversoes
        (vec![Tonic, Third(Minor), Fifth(Augmented)], "major 1st inversion".to_string()),
        (vec![Tonic, Fourth(Perfect), Sixth(Major)], "major 2st inversion".to_string()),
        (vec![Tonic, Third(Major), Sixth(Major)], "minor 1st inversion".to_string()),
        (vec![Tonic, Fourth(Perfect), Fifth(Augmented)], "minor 2st inversion".to_string()),
    ].into_iter().collect()
}

pub struct Scale {
    notes: Vec<Note>, 
    scale: Vec<Note>,
}

impl Scale {
    pub fn new() -> Self {
        Scale {
            scale: vec![],
            notes: vec![]
        }
    }

    pub fn fill_scale(&mut self, notes: Vec<Note>) {
        let tonic = notes[0].clone();
        let mut scale = vec![];
    
        let tonic = 
            if tonic.accidental == Flat {
                Note::get_sharp_eq(&tonic)
            } else { tonic };
    
        let mut cursor = tonic.clone();
        loop {
            let next_note = Note::get_next_note(&cursor);
            scale.push(cursor);            
            cursor = next_note;
            if cursor == tonic {
                break;
            }
        }
        self.notes = notes;
        self.scale = scale;
    }

    pub fn get_intervals(&self) -> Result<Vec<Interval>, CustomError> {
        let mut notes = self.notes.clone();
        let mut intervals = vec![];

        while let Some(mut note) = notes.pop() {
            if note.accidental == Flat {
                note = Note::get_sharp_eq(&note);
            }
        
            let interval = self.map_to_interval(&note)?;
            intervals.push(interval);
        }
        intervals.sort();
        Ok(intervals)
    }

    fn map_to_interval(&self, note: &Note) -> Result<Interval, CustomError> {
        use IntervalType::*;

        if let Some(p) = self.scale.iter().position(|i| i == note) {
            let interval = match p {
                0 => Interval::Tonic,
                1 => Interval::Second(Minor),
                2 => Interval::Second(Major),
                3 => Interval::Third(Minor),
                4 => Interval::Third(Major),
                5 => Interval::Fourth(Perfect),
                6 => Interval::Fourth(Augmented),
                7 => Interval::Fifth(Perfect),
                8 => Interval::Fifth(Augmented),
                9 => Interval::Sixth(Major),
                10 => Interval::Seventh(Minor),
                11 => Interval::Seventh(Major),
                _ => return Err(CustomError::MapIntervalError)
            };
            Ok(interval)
        }
        else {
            return Err(CustomError::MapIntervalError);
        }
    }

    fn get_inversion_string(&self, c: &str) -> Result<Chord, CustomError> {
        let chord = match c {
            "major 1st inversion" => format!("{}/{}", self.notes[2], self.notes[0]),
            "major 2st inversion" => format!("{}/{}", self.notes[1], self.notes[0]),
            "minor 1st inversion" => format!("{}m/{}", self.notes[2], self.notes[0]),
            "minor 2st inversion" => format!("{}m/{}", self.notes[1], self.notes[0]),
            _ => return Err(CustomError::MapInversionError)
        };
        Ok(chord)
    }

    pub fn to_chord(&self, intervals: Vec<Interval>) -> Result<Chord, CustomError> {  
        let chord_shapes = get_chord_shapes();

        let chord_string: String;
        if let Some(c) = chord_shapes.get(&intervals) {
            if c.contains("inversion") {
                chord_string = self.get_inversion_string(c)?;
            } 
            else {
                chord_string = format!("{}{}", self.notes[0], c);
            }
        }
        else {
            return Err(CustomError::MapChordError(intervals));
        } 
        Ok(chord_string)
    }
    
}
