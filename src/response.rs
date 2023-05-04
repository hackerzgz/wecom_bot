use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct SendResp {
    #[serde(rename = "errcode")]
    pub err_code: i64,

    #[serde(rename = "errmsg")]
    pub err_msg: String,
}

#[derive(Debug, Deserialize)]
pub struct UploadResp {
    #[serde(rename = "errcode")]
    pub err_code: i64,

    #[serde(rename = "errmsg")]
    pub err_msg: String,

    #[serde(rename = "type", default)]
    pub media_type: String,

    #[serde(rename = "media_id", default)]
    pub media_id: String,

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
