use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::image::Image;

static GROUP_REBOT_MSG_TEXT: &str = "text";
static GROUP_REBOT_MSG_MARKDOWN: &str = "markdown";
static GROUP_REBOT_MSG_IMAGE: &str = "image";
static GROUP_REBOT_MSG_NEWS: &str = "news";

#[derive(Debug, Serialize)]
enum MessageBody<'a> {
    #[serde(rename = "text")]
    Text {
        /// Raw text content, up to 2048 bytes
        content: Cow<'a, str>,
        /// A list of userid.
        ///
        /// To remind the specified members in the group (@Member). Use `@all`
        /// means to remind everyone. Use `mentioned_mobile_list` instead if the
        /// developer cannot get the userid.
        #[serde(skip_serializing_if = "Option::is_none")]
        mentioned_list: Option<Vec<Cow<'a, str>>>,
        /// A list of mobile phone.
        ///
        /// To remind the group members corresponding to the mobile phone
        /// (@Member). Use `@all` means  to remind everyone in group.
        #[serde(skip_serializing_if = "Option::is_none")]
        mentioned_mobile_list: Option<Vec<Cow<'a, str>>>,
    },
    #[serde(rename = "markdown")]
    Markdown {
        /// markdown raw text content, up to 4096 bytes.
        content: Cow<'a, str>,
    },
    #[serde(rename = "image")]
    Image {
        /// base64 encoding of image content.
        base64: Cow<'a, str>,

        /// md5 encoding of image(before base64 encoding) content.
        md5: Cow<'a, str>,
    },
    #[serde(rename = "news")]
    News {
        /// Article content, each news supports 1 to 8 pieces of articles message.
        articles: Vec<Article<'a>>,
    },
}

#[derive(Debug, Serialize)]
pub struct Message<'a> {
    /// Type of message.
    #[serde(rename = "msgtype")]
    msg_type: &'static str,

    #[serde(flatten)]
    body: MessageBody<'a>,
}

macro_rules! iter_to_vec {
    ($values:tt) => {
        let $values = match $values {
            Some(list) => {
                let vs = list.into_iter().map(|v| v.into()).collect();
                Some(vs)
            }
            None => None,
        };
    };
}

impl<'a> Message<'a> {
    pub fn text<S, I>(
        content: S,
        mentioned_list: Option<I>,
        mentioned_mobile_list: Option<I>,
    ) -> Message<'a>
    where
        S: Into<Cow<'a, str>>,
        I: IntoIterator<Item = S>,
    {
        iter_to_vec!(mentioned_list);
        iter_to_vec!(mentioned_mobile_list);

        Self {
            msg_type: GROUP_REBOT_MSG_TEXT,
            body: MessageBody::Text {
                content: content.into(),
                mentioned_list,
                mentioned_mobile_list,
            },
        }
    }

    pub fn markdown<S>(content: S) -> Message<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            msg_type: GROUP_REBOT_MSG_MARKDOWN,
            body: MessageBody::Markdown {
                content: content.into(),
            },
        }
    }

    pub fn image(image: Image) -> Message<'a> {
        let (base64, md5) = image.encode();
        Self {
            msg_type: GROUP_REBOT_MSG_IMAGE,
            body: MessageBody::Image {
                base64: Cow::from(base64),
                md5: Cow::from(md5),
            },
        }
    }

    pub fn news(articles: Vec<Article<'a>>) -> Message {
        Self {
            msg_type: GROUP_REBOT_MSG_NEWS,
            body: MessageBody::News { articles },
        }
    }
}

/// elements of wecom bot message type news.
#[derive(Debug, Clone, Serialize)]
pub struct Article<'a> {
    /// No more than 128 bytes, it will be automatically truncated if exceeded.
    pub title: Cow<'a, str>,

    /// No more than 512 bytes, it will be automatically truncated if exceeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,

    /// The link to be redirected after clicking.
    pub url: Cow<'a, str>,

    /// The image link of the article message supports _JPG_ and _PNG_ formats.
    /// The optimal size for better effect is 1068*455 for large images and
    /// 150*150 for small images.
    #[serde(rename = "picurl", skip_serializing_if = "Option::is_none")]
    pub pic_url: Option<Cow<'a, str>>,
}

impl<'a> Article<'a> {
    pub fn new<S>(title: S, url: S) -> Article<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            title: title.into(),
            url: url.into(),
            description: None,
            pic_url: None,
        }
    }

    pub fn desc<D>(&mut self, desc: D) -> &mut Article<'a>
    where
        D: Into<Cow<'a, str>>,
    {
        self.description = Some(desc.into());
        self
    }

    pub fn pic<P>(&mut self, pic: P) -> &mut Article<'a>
    where
        P: Into<Cow<'a, str>>,
    {
        self.pic_url = Some(pic.into());
        self
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
        serialize_text();
        serialize_markdown();
        serialize_image();
        serialize_article();
    }

    fn serialize_text() {
        let text = Message::text("Text-Only",None::<Vec<_>>, None);
        assert_eq!(
            "{\"msgtype\":\"text\",\"text\":{\"content\":\"Text-Only\"}}",
            serde_json::to_string(&text).unwrap()
        );

        let text = Message::text(
            "Title".to_string(),
            Some(vec![String::new(), "uid2".to_string()]),
            None,
        );
        assert_eq!(
            "{\"msgtype\":\"text\",\"text\":{\"content\":\"Title\",\"mentioned_list\":[\"\",\"uid2\"]}}",
            serde_json::to_string(&text).unwrap()
        );

        let user_iter = vec!["uid1", "uid2"].into_iter();
        let text = Message::text("User-Iter", Some(user_iter), None);
        assert_eq!(
            "{\"msgtype\":\"text\",\"text\":{\"content\":\"User-Iter\",\"mentioned_list\":[\"uid1\",\"uid2\"]}}",
            serde_json::to_string(&text).unwrap()
        );

        let text = Message::text(
            "Title-2".to_string(),
            None,
            Some(vec![String::new(), "1234567890".to_string()]),
        );
        assert_eq!(
            "{\"msgtype\":\"text\",\"text\":{\"content\":\"Title-2\",\"mentioned_mobile_list\":[\"\",\"1234567890\"]}}",
            serde_json::to_string(&text).unwrap()
        );
    }

    fn serialize_markdown() {
        let md = Message::markdown(r"# Markdown".to_string());
        assert_eq!(
            "{\"msgtype\":\"markdown\",\"markdown\":{\"content\":\"# Markdown\"}}",
            serde_json::to_string(&md).unwrap()
        );
    }

    fn serialize_image() {
        let img = Message::image(Image::new(b"image".to_vec()));
        assert_eq!(
            "{\"msgtype\":\"image\",\"image\":{\"base64\":\"aW1hZ2U=\",\"md5\":\"78805a221a988e79ef3f42d7c5bfd418\"}}",
            serde_json::to_string(&img).unwrap()
        );
    }

    fn serialize_article() {
        let mut air = Article::new("中秋节礼品领取".to_string(), "www.qq.com".to_string());
        air.desc("今年中秋节公司有豪礼相送".to_string()).pic(
            "http://res.mail.qq.com/node/ww/wwopenmng/images/independent/doc/test_pic_msg1.png"
                .to_string(),
        );
        let news = Message::news(vec![Article::new("".to_string(), "".to_string()), air]);
        assert_eq!(
            r#"{"msgtype":"news","news":{"articles":[{"title":"","url":""},{"title":"中秋节礼品领取","description":"今年中秋节公司有豪礼相送","url":"www.qq.com","picurl":"http://res.mail.qq.com/node/ww/wwopenmng/images/independent/doc/test_pic_msg1.png"}]}}"#,
            serde_json::to_string(&news).unwrap()
        );
    }
}
