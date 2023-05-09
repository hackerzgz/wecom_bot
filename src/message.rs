use std::borrow::Cow;

use serde::Serialize;

use crate::image::Image;

static GROUP_REBOT_MSG_TEXT: &str = "text";
static GROUP_REBOT_MSG_MARKDOWN: &str = "markdown";
static GROUP_REBOT_MSG_IMAGE: &str = "image";
static GROUP_REBOT_MSG_NEWS: &str = "news";
static GROUP_REBOT_MSG_FILE: &str = "file";

#[derive(Debug, Clone, Serialize)]
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
    #[serde(rename = "file")]
    File {
        /// File id, obtained through the wecom bot upload interface mentioned.
        media_id: Cow<'a, str>,
    },
}

macro_rules! inject_iter_fields {
    ($field_name:tt, $matched_type:path) => {
        pub fn $field_name<S, I>(mut self, iter: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: Into<Cow<'a, str>>,
        {
            match &mut self.body {
                $matched_type { $field_name, .. } => {
                    let vs: Vec<Cow<'a, str>> = iter.into_iter().map(Into::into).collect();
                    *$field_name = Some(vs);
                    self
                }
                _ => self,
            }
        }
    };
}

#[derive(Debug, Clone, Serialize)]
pub struct Message<'a> {
    /// Type of message.
    #[serde(rename = "msgtype")]
    msg_type: &'static str,

    #[serde(flatten)]
    body: MessageBody<'a>,
}

impl<'a> Message<'a> {
    /// Returns a raw text wecom `Message` without mentioned anyone.
    ///
    /// The maxium length of content up to 2048 bytes.
    ///
    /// <https://developer.work.weixin.qq.com/document/path/91770#%E6%96%87%E6%9C%AC%E7%B1%BB%E5%9E%8B>
    ///
    /// Use `mentioned_list` or `mentioned_mobile_list` to add someone you want
    /// to mentioned.
    ///
    /// ```
    /// # use wecom_bot::{Message, SendResp, WeComBot, WeComError};
    ///
    /// # fn main() -> Result<(), WeComError> {
    ///     let msg = Message::text("hello world!");
    ///     let _rsp: SendResp = WeComBot::builder().key("xxx").build()?.send(msg)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn text<S>(content: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            msg_type: GROUP_REBOT_MSG_TEXT,
            body: MessageBody::Text {
                content: content.into(),
                mentioned_list: None,
                mentioned_mobile_list: None,
            },
        }
    }

    /// Returns a markdown syntax wecom `Message` that display rendered format.
    ///
    /// The maxium length of content up to 4096 bytes.
    ///
    /// <https://developer.work.weixin.qq.com/document/path/91770#markdown%E7%B1%BB%E5%9E%8B>
    ///
    /// Use the `<@userid>` extend syntax in content to mention someone in the group.
    ///
    /// ```
    /// # use wecom_bot::{Message, SendResp, WeComBot, WeComError};
    ///
    /// # fn main() -> Result<(), WeComError> {
    ///     let msg = Message::markdown(r#"# hello world! <@1001>"#);
    ///     let _rsp: SendResp = WeComBot::builder().key("xxx").build()?.send(msg)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn markdown<S>(content: S) -> Self
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

    /// Returns a wecom `Message` that displays an image.
    ///
    /// The image format only supports JPG or PNG, with maximum size up to 2Mb.
    ///
    /// <https://developer.work.weixin.qq.com/document/path/91770#%E5%9B%BE%E7%89%87%E7%B1%BB%E5%9E%8B>
    ///
    /// ```
    /// # use wecom_bot::{Image, Message, SendResp, WeComBot, WeComError};
    ///
    /// # fn main() -> Result<(), WeComError> {
    ///     let msg = Message::image(Image::from_file("src/tests/imgs/profile.png")?);
    ///     let _rsp: SendResp = WeComBot::builder().key("xxx").build()?.send(msg)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn image(image: Image) -> Self {
        let (base64, md5) = image.encode();
        Self {
            msg_type: GROUP_REBOT_MSG_IMAGE,
            body: MessageBody::Image {
                base64: Cow::from(base64),
                md5: Cow::from(md5),
            },
        }
    }

    /// Returns an article wecom `Message` that can click then redirect to a new
    /// url in internal web brower.
    ///
    /// The length of `articles` can be up to 8, with each `title` being a
    /// maximum of 128 bytes and each `description` being a maximum of 512 bytes.
    ///
    /// <https://developer.work.weixin.qq.com/document/path/91770#%E5%9B%BE%E6%96%87%E7%B1%BB%E5%9E%8B>
    ///
    /// ```
    /// # use wecom_bot::{Article, Message, SendResp, WeComBot, WeComError};
    ///
    /// # fn main() -> Result<(), WeComError> {
    ///     let msg = Message::news(vec![Article::new("title", "url")]);
    ///     let _rsp: SendResp = WeComBot::builder().key("xxx").build()?.send(msg)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn news(articles: Vec<Article<'a>>) -> Self {
        Self {
            msg_type: GROUP_REBOT_MSG_NEWS,
            body: MessageBody::News { articles },
        }
    }

    /// Returns a file wecom `Message` that displays with its name and can be
    /// download.
    ///
    /// The required field `media_id` are obtained through the `upload` API returned.
    ///
    /// <https://developer.work.weixin.qq.com/document/path/91770#%E6%96%87%E4%BB%B6%E7%B1%BB%E5%9E%8B>
    ///
    /// ```
    /// # use wecom_bot::{Article, Message, SendResp, WeComBot, WeComError};
    ///
    /// # fn main() -> Result<(), WeComError> {
    ///     let msg = Message::file("3a8asd892asd8asd");
    ///     let _rsp: SendResp = WeComBot::builder().key("xxx").build()?.send(msg)?;
    /// #   Ok(())
    /// # }
    /// ```
    pub fn file<S>(media_id: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            msg_type: GROUP_REBOT_MSG_FILE,
            body: MessageBody::File {
                media_id: media_id.into(),
            },
        }
    }

    inject_iter_fields!(mentioned_list, MessageBody::Text);

    inject_iter_fields!(mentioned_mobile_list, MessageBody::Text);
}

/// Elements of wecom bot message type news.
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
    /// Returns a article that can be clicked and displays with text and image
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

    /// Set the description on the article to show more detail about itself.
    pub fn desc<D>(&mut self, desc: D) -> &mut Article<'a>
    where
        D: Into<Cow<'a, str>>,
    {
        self.description = Some(desc.into());
        self
    }

    /// Set the image on the article card to make it more beautiful.
    pub fn pic<P>(&mut self, pic: P) -> &mut Article<'a>
    where
        P: Into<Cow<'a, str>>,
    {
        self.pic_url = Some(pic.into());
        self
    }
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
        serialize_file();
    }

    fn serialize_text() {
        let text = Message::text("Text-Only");
        assert_eq!(
            "{\"msgtype\":\"text\",\"text\":{\"content\":\"Text-Only\"}}",
            serde_json::to_string(&text).unwrap()
        );

        let text = Message::text("Title").mentioned_list(vec!["", "uid2"]);
        assert_eq!(
            "{\"msgtype\":\"text\",\"text\":{\"content\":\"Title\",\"mentioned_list\":[\"\",\"uid2\"]}}",
            serde_json::to_string(&text).unwrap()
        );

        let user_iter = vec!["uid1", "uid2"].into_iter();
        let text = Message::text("User-Iter").mentioned_list(user_iter);
        assert_eq!(
            "{\"msgtype\":\"text\",\"text\":{\"content\":\"User-Iter\",\"mentioned_list\":[\"uid1\",\"uid2\"]}}",
            serde_json::to_string(&text).unwrap()
        );

        let text = Message::text("Title-2").mentioned_mobile_list(vec!["", "1234567890"]);
        assert_eq!(
            "{\"msgtype\":\"text\",\"text\":{\"content\":\"Title-2\",\"mentioned_mobile_list\":[\"\",\"1234567890\"]}}",
            serde_json::to_string(&text).unwrap()
        );
    }

    fn serialize_markdown() {
        let md = Message::markdown(r"# Markdown");
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
        let mut air = Article::new("中秋节礼品领取", "www.qq.com");
        air.desc("今年中秋节公司有豪礼相送").pic(
            "http://res.mail.qq.com/node/ww/wwopenmng/images/independent/doc/test_pic_msg1.png",
        );
        let news = Message::news(vec![Article::new("", ""), air]);
        assert_eq!(
            r#"{"msgtype":"news","news":{"articles":[{"title":"","url":""},{"title":"中秋节礼品领取","description":"今年中秋节公司有豪礼相送","url":"www.qq.com","picurl":"http://res.mail.qq.com/node/ww/wwopenmng/images/independent/doc/test_pic_msg1.png"}]}}"#,
            serde_json::to_string(&news).unwrap()
        );
    }

    fn serialize_file() {
        let file = Message::file("3a8asd892asd8asd");

        assert_eq!(
            r#"{"msgtype":"file","file":{"media_id":"3a8asd892asd8asd"}}"#,
            serde_json::to_string(&file).unwrap(),
        );
    }
}
