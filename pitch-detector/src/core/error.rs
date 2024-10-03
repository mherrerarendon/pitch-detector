use std::{error::Error, fmt::Display};

#[derive(Debug, Clone)]
pub enum PitchError {
    UnexpectedError(String),
    IncorrectParameters(String),
}

impl Display for PitchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PitchError::UnexpectedError(msg) => writeln!(f, "Unexpected Error: {}", msg),
            PitchError::IncorrectParameters(msg) => {
                writeln!(f, "Incorrect Parameters Error: {}", msg)
            }
        }
    }
}

impl Error for PitchError {}
