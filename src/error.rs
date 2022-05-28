use std::fmt;

use crate::scale::Interval;

#[derive(Debug)]
pub enum CustomError {
    ParseNoteError,
    MapIntervalError,
    MapChordError(Vec<Interval>),
    MapInversionError,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CustomError::*;

        let err = match self {
            ParseNoteError => "Failed to parse due to invalid symbol error".to_string(),
            MapIntervalError => "Failed to map interval on scale".to_string(), 
            MapChordError(intervals) => format!("Failed to map chord with intervals `{:?}`", intervals),
            MapInversionError => "Failed to map chord inversion".to_string()
        };
        write!(f, "ERROR: {}", err)
    }
}