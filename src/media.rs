use std::str::FromStr;

use crate::WeComError;

/// Builder for creating a validated instance of `WeComBot`.
pub enum MediaType {
    /// File media type.
    ///
    /// This media type is used for generic files.
    File,

    /// Image media type.
    ///
    /// This media type is used for image files.
    Image,

    /// Voice media type.
    ///
    /// This media type is used for voice or audio files.
    Voice,

    /// Video media type.
    ///
    /// This media type is used for video files.
    Video,
}

impl FromStr for MediaType {
    type Err = WeComError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(MediaType::File),
            "image" => Ok(MediaType::Image),
            "voice" => Ok(MediaType::Voice),
            "video" => Ok(MediaType::Video),
            _ => Err(WeComError::MediaType(s.to_string())),
        }
    }
}

impl ToString for MediaType {
    fn to_string(&self) -> String {
        match *self {
            MediaType::File => String::from("file"),
            MediaType::Image => String::from("image"),
            MediaType::Voice => String::from("voice"),
            MediaType::Video => String::from("video"),
        }
    }
}

impl MediaType {
    pub(crate) fn format_upload_url<U>(&self, base: U) -> String
    where
        U: Into<String>,
    {
        format!("{}&type={}", base.into(), self.to_string())
    }
}
