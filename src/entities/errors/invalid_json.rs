use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct InvalidJson;

impl fmt::Display for InvalidJson {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid json")
    }
}

impl Error for InvalidJson {}
