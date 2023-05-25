use serde::Deserialize;

/// Represents the result of sending a group message to the WeCom bot server.
#[derive(Debug, Default, Deserialize)]
pub struct SendResp {
    /// The error code returned by the WeCom bot server.
    ///
    /// A value of 0 indicates success, while non-zero values indicate an error.
    #[serde(rename = "errcode")]
    pub err_code: i64,

    /// The error message returned by the WeCom bot server.
    ///
    /// This field provides additional information about any errors that
    /// occurred during message sending.
    #[serde(rename = "errmsg")]
    pub err_msg: String,
}

/// Represents the result of uploading media files to the WeCom bot server.
#[derive(Debug, Deserialize)]
pub struct UploadResp {
    /// The error code returned by the WeCom bot server.
    ///
    /// A value of 0 indicates success, while non-zero values indicate an error.
    #[serde(rename = "errcode")]
    pub err_code: i64,

    /// A hint or additional information about any errors that occurred during
    /// media file upload.
    ///
    /// This field provides extra context or suggestions for resolving any
    /// upload issues.
    #[serde(rename = "errmsg")]
    pub err_msg: String,

    /// The type of the uploaded media.
    ///
    /// This field specifies the MIME type or format of the uploaded file, e.g.,
    /// "image/jpeg", "audio/mp3", etc.
    #[serde(rename = "type", default)]
    pub media_type: String,

    /// The unique identifier assigned to the uploaded media file.
    ///
    /// This identifier can be used to send the uploaded file via a file type message.
    #[serde(rename = "media_id", default)]
    pub media_id: String,

    /// The timestamp of when the media file was created.
    ///
    /// This field represents the date and time when the media file was uploaded
    /// to the WeCom bot server.
    #[serde(rename = "created_at", default)]
    pub created_at: String,
}

impl Default for UploadResp {
    fn default() -> Self {
        UploadResp {
            err_code: 0,
            err_msg: String::from("success"),
            media_type: crate::MediaType::File.to_string(),
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
