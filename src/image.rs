use std::fs::File;
use std::io::Read;
use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use md5;

use crate::bot::WeComError;

pub struct Image {
    content: Vec<u8>,
}

impl Image {
    pub fn new(data: Vec<u8>) -> Self {
        Self { content: data }
    }

    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, WeComError> {
        let mut file = File::open(path).map_err(WeComError::image)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).map_err(WeComError::image)?;
        Ok(Self { content: buf })
    }

    /// return encoded base64 and md5 of image data
    pub fn encode(&self) -> (String, String) {
        let b64 = general_purpose::STANDARD.encode(self.content.clone());
        let m5 = md5::compute(self.content.clone());

        (b64, format!("{:x}", m5))
    }
}

#[cfg(test)]
mod image_test {
    use super::Image;

    #[test]
    pub fn encode() {
        let img = Image::from_file("src/tests/imgs/tiny-rust-logo.png").unwrap();

        assert_eq!(img.encode().0, "iVBORw0KGgoAAAANSUhEUgAAAAoAAAAKCAYAAACNMs+9AAAAAXNSR0IArs4c6QAAAJFJREFUKFON0DEOgUEUBODvl6CQqCV6Bc6hopdotO7gHHqNC3ANrcQBJFqNRIO8ZFf+bEJs87Iz82ZepvLnqwpdAy0cMapzdeErERmL/xbLwDMY81m4X9DPmixs4oAJrujhjAHinI/jCUNEXCzf0E0JHdzLG2eJDPcp9mV08BusMMcYayywq0eXbbbx+FbPz+rfkJoUC+KW8YsAAAAASUVORK5CYII=");
        assert_eq!(img.encode().1, "4d1b24690a324e7ac911c3c721982951");
    }
}
