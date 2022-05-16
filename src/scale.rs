use std::collections::HashSet;

use crate::note::Accidental::*;
use crate::note::Note;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum IntervalType {
    Major,
    Minor,
    Perfect,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Interval {
    Tonic,
    Third(IntervalType),
    Fourth(IntervalType),
    Fifth(IntervalType),
    Seventh(IntervalType),
}

pub struct Scale {
    tone: Note, 
    scale: Vec<Note>,
}

impl Scale {
    pub fn new(note: Note) -> Self {
        Scale {
            tone: note.clone(), 
            scale: Note::fill_scale(note) 
        }
    }

    pub fn get_intervals(&self, mut notes: Vec<Note>) -> HashSet<Interval> {

        let mut intervals = HashSet::new();

        while let Some(mut note) = notes.pop() {
            if note.accidental == Flat {
                note = Note::get_sharp_eq(note.clone());
            }
        
            let interval = match self.scale.iter().position(|i| i == &note).unwrap() {
                0 => Interval::Tonic,
                3 => Interval::Third(IntervalType::Minor),
                4 => Interval::Third(IntervalType::Major),
                5 => Interval::Fourth(IntervalType::Perfect),
                7 => Interval::Fifth(IntervalType::Perfect),
                10 => Interval::Seventh(IntervalType::Minor),
                11 => Interval::Seventh(IntervalType::Major),
                _ => panic!("{:?} not mapped", note)
            };
            intervals.insert(interval);
        }
        intervals
    }

    pub fn to_chord(&self, intervals: HashSet<Interval>) -> String {
        use Interval::*;
        use IntervalType::*;

        let mut chord_string = format!("{}", self.tone);
        
        let minor_triad: HashSet<Interval> = [Tonic, Third(Minor), Fifth(Perfect)].into_iter().collect();
        let major_tetrad: HashSet<Interval> = [Tonic, Third(Major), Fifth(Perfect), Seventh(Major)].into_iter().collect();
        let minor_tetrad: HashSet<Interval> = [Tonic, Third(Minor), Fifth(Perfect), Seventh(Minor)].into_iter().collect();
        let dom_tetrad: HashSet<Interval> = [Tonic, Third(Major), Fifth(Perfect), Seventh(Minor)].into_iter().collect();
        
        if intervals == minor_triad {
            chord_string.push_str("m");
        } 
        else if intervals == major_tetrad {
            chord_string.push_str("maj7");
        }
        else if intervals == minor_tetrad {
            chord_string.push_str("m7")
        }
        else if intervals == dom_tetrad {
            chord_string.push_str("7")
        }
        else {
            println!("Chord not mapped");
        } 
        chord_string
    }
    
}
