use std::fs::File;
use std::io::Read;
use std::path::Path;

use base64::{engine::general_purpose, Engine as _};
use md5;

use crate::bot::WeComError;

/// An wecom bot format Image loaded from or image data or a path.
///
/// This struct only supports PNG or JPG formats.
///
/// # Example
///
/// ```
/// use wecom_bot::{Image};
///
/// let raw_data = vec![0xff, 0x00, 0x00, 0xff, /* ... */];
/// let logo = Image::new(raw_data);
///
/// let logo = Image::from_file("src/tests/imgs/tiny-rust-logo.png").unwrap();
/// ```
pub struct Image {
    content: Vec<u8>,
}

impl Image {
    /// Creates a new [`Image`] instance from the given raw image data.
    pub fn new(data: Vec<u8>) -> Self {
        Self { content: data }
    }

    /// Loads the image data from a file located at the given path.
    ///
    /// # Errors
    ///
    /// Returns a `WeComError::Image` variant if the file cannot be opened or read.
    ///
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, WeComError> {
        let mut file = File::open(path).map_err(WeComError::image)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf).map_err(WeComError::image)?;
        Ok(Self { content: buf })
    }

    /// Encodes the image data as base64 and computes its MD5 hash.
    pub(crate) fn encode(&self) -> (String, String) {
        let content = self.content.clone();
        let b64 = general_purpose::STANDARD.encode(&content);
        let m5 = md5::compute(&content);

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
