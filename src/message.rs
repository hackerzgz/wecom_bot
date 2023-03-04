use serde::Serialize;

static GROUP_REBOT_MSG_TEXT: &str = "text";
static GROUP_REBOT_MSG_MARKDOWN: &str = "markdown";

#[derive(Debug, Serialize)]
enum MessageBody {
    Text {
        /// Raw text content, up to 2048 bytes
        content: String,
        /// A list of userid.
        ///
        /// To remind the specified members in the group (@Member). Use `@all`
        /// means to remind everyone. Use `mentioned_mobile_list` instead if the
        /// developer cannot get the userid.
        mentioned_list: Option<Vec<String>>,
        /// A list of mobile phone.
        ///
        /// To remind the group members corresponding to the mobile phone
        /// (@Member). Use `@all` means  to remind everyone in group.
        mentioned_mobile_list: Option<Vec<String>>,
    },
    Markdown {
        /// markdown raw text content, up to 4096 bytes
        content: String,
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
