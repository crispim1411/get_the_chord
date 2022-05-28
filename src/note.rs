 
use std::{fmt, str::FromStr};

use Symbol::*;
use Accidental::*;

use crate::CustomError;

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    C,
    D,
    E,
    F,
    G,
    A,
    B
}

#[derive(Debug, Clone, PartialEq)]
pub enum Accidental {
    Flat,
    Normal,
    Sharp
}

#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    symbol: Symbol,
    pub accidental: Accidental,
}

impl Note {
    pub fn new(symbol: Symbol, accidental: Accidental) -> Self {
        Note { symbol, accidental }
    }

    pub fn get_next_note(note: &Note) -> Note {
        let next_symbol = match note.symbol {
            C => D,
            D => E,
            E => F,
            F => G,
            G => A,
            A => B,
            B => C
        };
    
        match note.accidental {
            Flat => Note::new(note.symbol.clone(), Normal),
            Normal if [B, E].contains(&note.symbol) => Note::new(next_symbol, Normal),
            Normal => Note::new(note.symbol.clone(), Sharp),        
            Sharp => Note::new(next_symbol, Normal)        
        }
    }
    
    pub fn get_sharp_eq(note: &Note) -> Note {
        if note == &Note::new(F, Flat) {
            return Note::new(E, Normal);
        } 
        else if note == &Note::new(C, Flat) {
            return Note::new(B, Normal);
        }

        let previous_symbol = match note.symbol {
            C => B,
            D => C,
            E => D,
            F => E,
            G => F,
            A => G,
            B => A
        };
        Note::new(previous_symbol, Sharp) 
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut signal = "";
        if self.accidental == Flat {
            signal = "b";
        } 
        else if self.accidental == Sharp {
            signal = "#";
        }
        write!(f, "{:?}{}", self.symbol, signal)
    }
}

impl FromStr for Note {
    type Err = CustomError;

    fn from_str(text: &str) -> Result<Note, Self::Err> {
        let chars: Vec<char> = text
            .to_uppercase()
            .chars()
            .collect();

        if chars.len() > 2 {
            return Err(CustomError::ParseNoteError);
        }

        let symbol = 
            if let Some(note) = chars.get(0) {
                match note {
                    'C' => C,
                    'D' => D,
                    'E' => E,
                    'F' => F,
                    'G' => G,
                    'A' => A,
                    'B' => B,
                    _ => return Err(CustomError::ParseNoteError)
                }
            } else {
                return Err(CustomError::ParseNoteError);
            };

        let accidental = 
            if let Some(signal) = chars.get(1) {
                match signal {
                    'B' => Flat,
                    '#' => Sharp,
                    _ => return Err(CustomError::ParseNoteError)
                }
            } else {
                Normal
            };

        let mut note = Note::new(symbol, accidental);

        let enarmonics = vec![
            Note::new(E, Sharp),
            Note::new(B, Sharp)
        ];
        if enarmonics.contains(&note) {
            note = Note::get_next_note(&note);
        }

        Ok(note)
    }
}