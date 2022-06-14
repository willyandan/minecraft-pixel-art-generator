use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct WritePermission {}

impl fmt::Display for WritePermission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error trying to write. Try to run as Admin")
    }
}

impl Error for WritePermission {}
