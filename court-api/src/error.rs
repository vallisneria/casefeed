use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CourtApiError {
    UnexpectedValue,
}

impl Error for CourtApiError {}

impl Display for CourtApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::UnexpectedValue => "Unexpected Value",
        };

        write!(f, "{}", message)
    }
}
