use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct ImageNotFound {
    image_path: String,
}

impl fmt::Display for ImageNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image Not Found:{}", self.image_path)
    }
}

impl Error for ImageNotFound {}

impl ImageNotFound {
    pub fn new(image_path: String) -> Self {
        ImageNotFound {
            image_path: image_path,
        }
    }
}
