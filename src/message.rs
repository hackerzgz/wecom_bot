use serde::{Deserialize, Serialize};

use crate::image::Image;

static GROUP_REBOT_MSG_TEXT: &str = "text";
static GROUP_REBOT_MSG_MARKDOWN: &str = "markdown";
static GROUP_REBOT_MSG_IMAGE: &str = "image";

#[derive(Debug, Serialize)]
enum MessageBody {
    #[serde(rename = "text")]
    Text {
        /// Raw text content, up to 2048 bytes
        content: String,
        /// A list of userid.
        ///
        /// To remind the specified members in the group (@Member). Use `@all`
        /// means to remind everyone. Use `mentioned_mobile_list` instead if the
        /// developer cannot get the userid.
        #[serde(skip_serializing_if = "Option::is_none")]
        mentioned_list: Option<Vec<String>>,
        /// A list of mobile phone.
        ///
        /// To remind the group members corresponding to the mobile phone
        /// (@Member). Use `@all` means  to remind everyone in group.
        #[serde(skip_serializing_if = "Option::is_none")]
        mentioned_mobile_list: Option<Vec<String>>,
    },
    #[serde(rename = "markdown")]
    Markdown {
        /// markdown raw text content, up to 4096 bytes
        content: String,
    },
    #[serde(rename = "markdown")]
    Image {
        /// base64 encoding of image content
        base64: String,
        /// md5 encoding of image(before base64 encoding) content
        md5: String,
    },
}

#[derive(Debug, Serialize)]
pub struct Message {
    /// Type of message.
    #[serde(rename = "msgtype")]
    msg_type: String,

    #[serde(flatten)]
    body: MessageBody,
}

impl Message {
    pub fn text(
        content: String,
        mentioned_list: Option<Vec<String>>,
        mentioned_mobile_list: Option<Vec<String>>,
    ) -> Message {
        Self {
            msg_type: GROUP_REBOT_MSG_TEXT.to_string(),
            body: MessageBody::Text {
                content,
                mentioned_list,
                mentioned_mobile_list,
            },
        }
    }

    pub fn markdown(content: String) -> Message {
        Self {
            msg_type: GROUP_REBOT_MSG_MARKDOWN.to_string(),
            body: MessageBody::Markdown { content },
        }
    }

    pub fn image(image: Image) -> Message {
        let (base64, md5) = image.encode();
        Self {
            msg_type: GROUP_REBOT_MSG_IMAGE.to_string(),
            body: MessageBody::Image { base64, md5 },
        }
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct SendResp {
    #[serde(rename = "errcode")]
    pub err_code: i64,

    #[serde(rename = "errmsg")]
    pub err_msg: String,
}

#[cfg(test)]
mod message_tests {
    use super::*;

    #[test]
    fn serialize_request() {
        let text = Message::text(
            "Title".to_string(),
            Some(vec![String::new(), "uid2".to_string()]),
            None,
        );
        assert_eq!(
            "{\"msgtype\":\"text\",\"text\":{\"content\":\"Title\",\"mentioned_list\":[\"\",\"uid2\"]}}",
            serde_json::to_string(&text).unwrap()
        );

        let md = Message::markdown(r"# Markdown".to_string());
        assert_eq!(
            "{\"msgtype\":\"markdown\",\"markdown\":{\"content\":\"# Markdown\"}}",
            serde_json::to_string(&md).unwrap()
        );
    }
}
