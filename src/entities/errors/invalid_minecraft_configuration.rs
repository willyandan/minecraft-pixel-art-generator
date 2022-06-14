use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct InvalidMinecraftConfiguration;

impl fmt::Display for InvalidMinecraftConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Invalid minecraft configuration. \nCheck if minecrat-pixel-art-generator is inside the .minecraft. \nCheck if the world name is correct")
    }
}

impl Error for InvalidMinecraftConfiguration {}
