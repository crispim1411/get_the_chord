use std::fmt;

use Symbol::*;
use Accidental::*;

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

    pub fn get_next_note(note: Note) -> Note {
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
            Flat => Note::new(note.symbol, Normal),
            Normal if [B, E].contains(&note.symbol) => Note::new(next_symbol, Normal),
            Normal => Note::new(note.symbol, Sharp),        
            Sharp => Note::new(next_symbol, Normal)        
        }
    }
    
    pub fn get_sharp_eq(note: Note) -> Note {
        
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