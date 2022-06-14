use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({},{},{})", self.red, self.green, self.blue)
    }
}

impl Rgb {
    pub fn from_tuple((red, green, blue): (u8, u8, u8)) -> Self {
        Rgb { red, green, blue }
    }
}
