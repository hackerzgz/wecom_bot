use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::WeComError;

pub enum MediaType {
    File,
    Image,
    Voice,
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
            _ => panic!("media type missing match"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UploadResp {
    #[serde(rename = "errcode")]
    pub err_code: i64,

    #[serde(rename = "errmsg")]
    pub err_msg: String,

    #[serde(rename = "type")]
    pub media_type: String,

    #[serde(rename = "media_id")]
    pub media_id: String,

    #[serde(rename = "created_at")]
    pub created_at: String,
}

impl Default for UploadResp {
    fn default() -> Self {
        UploadResp {
            err_code: 0,
            err_msg: String::from("success"),
            media_type: MediaType::File.to_string(),
            media_id: String::new(),
            created_at: String::new(),
        }
    }
}

impl UploadResp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_ok(&self) -> bool {
        self.err_code.eq(&0)
    }
}
