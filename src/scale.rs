use std::collections::HashMap;

use crate::note::Accidental::*;
use crate::note::Note;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum IntervalType {
    Major,
    Minor,
    Perfect,
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Interval {
    Tonic,
    Third(IntervalType),
    Fourth(IntervalType),
    Fifth(IntervalType),
    Seventh(IntervalType),
}

fn get_chord_shapes() -> HashMap<Vec<Interval>, String> {
    use Interval::*;
    use IntervalType::*;

    vec![
        (vec![Tonic, Third(Major), Fifth(Perfect)], "".to_string()),
        (vec![Tonic, Third(Minor), Fifth(Perfect)], "m".to_string()),
        (vec![Tonic, Third(Major), Fifth(Perfect), Seventh(Major)], "maj7".to_string()),
        (vec![Tonic, Third(Minor), Fifth(Perfect), Seventh(Minor)], "m7".to_string()),
        (vec![Tonic, Third(Major), Fifth(Perfect), Seventh(Minor)], "7".to_string()),
    ].into_iter().collect()
}

pub struct Scale {
    tone: Note, 
    scale: Vec<Note>,
}

impl Scale {
    pub fn new(note: Note) -> Self {
        Scale {
            tone: note.clone(), 
            scale: Scale::fill_scale(note) 
        }
    }

    pub fn fill_scale(mut tonic: Note) -> Vec<Note> {
        let mut scale = vec![];
    
        if tonic.accidental == Flat {
            tonic = Note::get_sharp_eq(tonic);
        }
    
        let mut note = tonic.clone();
        loop {
            scale.push(note.clone());
            note = Note::get_next_note(note);
            if note == tonic {
                break;
            }
        }
        scale
    }

    pub fn get_intervals(&self, mut notes: Vec<Note>) -> Vec<Interval> {

        let mut intervals = vec![];

        while let Some(mut note) = notes.pop() {
            if note.accidental == Flat {
                note = Note::get_sharp_eq(note.clone());
            }
        
            let interval = self.map_to_interval(note);
            intervals.push(interval);
        }
        intervals.sort();
        intervals
    }

    fn map_to_interval(&self, note: Note) -> Interval {
        if let Some(p) = self.scale.iter().position(|i| i == &note) {
            match p {
                0 => Interval::Tonic,
                3 => Interval::Third(IntervalType::Minor),
                4 => Interval::Third(IntervalType::Major),
                5 => Interval::Fourth(IntervalType::Perfect),
                7 => Interval::Fifth(IntervalType::Perfect),
                10 => Interval::Seventh(IntervalType::Minor),
                11 => Interval::Seventh(IntervalType::Major),
                _ => panic!("{:?} not mapped", note)
            }
        }
        else {
            panic!("{:?} not on scale", note);
        }
    }

    pub fn to_chord(&self, intervals: Vec<Interval>) -> String {  
        let chord_shapes = get_chord_shapes();

        let mut chord_string = self.tone.to_string();
        if let Some(c) = chord_shapes.get(&intervals) {
            chord_string.push_str(c);
        }
        else {
            println!("Chord not mapped");
        } 
        chord_string
    }
    
}
