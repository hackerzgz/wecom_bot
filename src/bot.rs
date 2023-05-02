use std::any;
use std::fmt::Debug;
use std::io;
use std::path::Path;
use std::time::Duration;

use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::message::Message;
use crate::upload::{MediaType, UploadResp};

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum WeComError {
    #[error("wecom bot key not set")]
    KeyNotFound,
    #[error("network failed: {}", source)]
    Network {
        #[from]
        source: reqwest::Error,
    },
    #[error("wecom bot server error: {}", status)]
    Http { status: reqwest::StatusCode },
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        #[source]
        source: serde_json::Error,
        typename: &'static str,
    },
    #[error("failed to read image file: {}", source)]
    ImageRead {
        #[from]
        source: io::Error,
    },
    #[error("unknown upload media type: {0}")]
    MediaType(String),
    #[error("failed to load upload file: {}", source)]
    LoadUploadFile { source: io::Error },
}

impl WeComError {
    fn network(source: reqwest::Error) -> Self {
        WeComError::Network { source }
    }

    fn data_type<T>(source: serde_json::Error) -> Self {
        WeComError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }

    pub(crate) fn image(source: io::Error) -> Self {
        WeComError::ImageRead { source }
    }

    fn load_file(source: io::Error) -> Self {
        WeComError::LoadUploadFile { source }
    }
}

type WeComResult<T> = Result<T, WeComError>;

pub struct WeComBot {
    url: String,
    upload_base_url: String,

    client: reqwest::blocking::Client,
}

impl WeComBot {
    fn new<K>(key: K) -> WeComResult<WeComBot>
    where
        K: Into<String>,
    {
        WeComBotBuilder::new().key(key).build()
    }

    pub fn builder() -> WeComBotBuilder {
        WeComBotBuilder::new()
    }

    pub fn send<T>(&self, msg: Message<'_>) -> WeComResult<T>
    where
        T: DeserializeOwned,
    {
        let resp = self.client.post(&self.url).json(&msg).send()?;
        let status = resp.status();
        if status.is_server_error() {
            return Err(WeComError::Http { status });
        }

        serde_json::from_reader::<_, T>(resp).map_err(WeComError::data_type::<T>)
    }

    pub fn upload<P>(&self, media_type: MediaType, path: P) -> WeComResult<UploadResp>
    where
        P: AsRef<Path>,
    {
        let file = reqwest::blocking::multipart::Form::new()
            .file("filename", path)
            .map_err(WeComError::load_file)?;

        let upload_url = media_type.format_upload_url(&self.upload_base_url);
        let resp = self.client.post(upload_url).multipart(file).send()?;
        let status = resp.status();
        if status.is_server_error() {
            return Err(WeComError::Http { status });
        }

        let ret: UploadResp = resp.json()?;
        Ok(ret)
    }
}

impl Debug for WeComBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeComBot").field("url", &self.url).finish()
    }
}

const WECOM_SEND_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/webhook/send";
const WECOM_UPLOAD_URL: &str = "https://qyapi.weixin.qq.com/cgi-bin/webhook/upload_media";

macro_rules! format_wecom_url {
    ($key:expr) => {
        match $key {
            None => return Err(WeComError::KeyNotFound),
            Some(k) => {
                if k.trim().len() == 0 {
                    return Err(WeComError::KeyNotFound);
                }
                (
                    format!("{}?key={}", WECOM_SEND_URL, k),
                    format!("{}?key={}", WECOM_UPLOAD_URL, k),
                )
            }
        }
    };
}

#[derive(Debug, Default)]
pub struct WeComBotBuilder {
    key: Option<String>,
    client: Option<reqwest::blocking::Client>,
}

impl WeComBotBuilder {
    pub fn new() -> WeComBotBuilder {
        Self::default()
    }

    pub fn build(self) -> WeComResult<WeComBot> {
        let (url, upload_base_url) = format_wecom_url!(self.key);

        let client = self.client.unwrap_or(
            reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        );
        Ok(WeComBot {
            client,
            url,
            upload_base_url,
        })
    }

    pub fn key<K>(mut self, key: K) -> WeComBotBuilder
    where
        K: Into<String>,
    {
        self.key = Some(key.into());
        self
    }

    pub fn client(mut self, client: reqwest::blocking::Client) -> WeComBotBuilder {
        self.client = Some(client);
        self
    }
}

#[cfg(feature = "async_api")]
pub struct WeComBotAsync {
    url: String,
    upload_base_url: String,

    client: reqwest::Client,
}

#[cfg(feature = "async_api")]
impl WeComBotAsync {
    fn new<K>(key: K) -> WeComResult<WeComBotAsync>
    where
        K: Into<String>,
    {
        WeComBotAsyncBuilder::new().key(key).build()
    }

    pub fn builder() -> WeComBotAsyncBuilder {
        WeComBotAsyncBuilder::new()
    }

    pub async fn send<T>(&self, msg: Message<'_>) -> WeComResult<T>
    where
        T: DeserializeOwned,
    {
        let resp = self
            .client
            .post(&self.url)
            .json(&msg)
            .send()
            .await
            .map_err(WeComError::network)?;
        let status = resp.status();
        if status.is_server_error() {
            return Err(WeComError::Http { status });
        }

        serde_json::from_slice::<T>(&resp.bytes().await?).map_err(WeComError::data_type::<T>)
    }

    pub async fn upload<P>(&self, media_type: MediaType, path: P) -> WeComResult<UploadResp>
    where
        P: AsRef<Path> + Sized,
    {
        let content = tokio::fs::read(&path)
            .await
            .map_err(WeComError::load_file)?;

        let filename = self.get_filename(path.as_ref());
        let part = reqwest::multipart::Part::bytes(content).file_name(filename);
        let form = reqwest::multipart::Form::new().part("filename", part);
        let upload_url = media_type.format_upload_url(&self.upload_base_url);

        let resp = self
            .client
            .post(upload_url)
            .multipart(form)
            .send()
            .await
            .map_err(WeComError::network)?;
        let status = resp.status();
        if status.is_server_error() {
            return Err(WeComError::Http { status });
        }

        serde_json::from_slice::<UploadResp>(&resp.bytes().await?)
            .map_err(WeComError::data_type::<UploadResp>)
    }

    fn get_filename(&self, p: &Path) -> String {
        let name = match p.file_name() {
            None => "",
            Some(f) => f.to_str().unwrap(),
        };
        String::from(name)
    }
}

#[cfg(feature = "async_api")]
impl Debug for WeComBotAsync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeComBot").field("url", &self.url).finish()
    }
}

#[cfg(feature = "async_api")]
#[derive(Debug, Default)]
pub struct WeComBotAsyncBuilder {
    key: Option<String>,
    client: Option<reqwest::Client>,
}

#[cfg(feature = "async_api")]
impl WeComBotAsyncBuilder {
    pub fn new() -> WeComBotAsyncBuilder {
        Self::default()
    }

    pub fn build(self) -> WeComResult<WeComBotAsync> {
        let (url, upload_base_url) = format_wecom_url!(self.key);

        let client = self.client.unwrap_or(
            reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        );

        Ok(WeComBotAsync {
            client,
            url,
            upload_base_url,
        })
    }

    pub fn key<K>(mut self, key: K) -> WeComBotAsyncBuilder
    where
        K: Into<String>,
    {
        self.key = Some(key.into());
        self
    }

    pub fn client(mut self, client: reqwest::Client) -> WeComBotAsyncBuilder {
        self.client = Some(client);
        self
    }
}

#[cfg(test)]
mod botest {
    use crate::message::{Message, SendResp};

    use super::WeComBot;

    #[test]
    fn send_msg() {
        let bot = WeComBot::new("693a91f6-7xxx-4bc4-97a0-0ec2sifa5aaa").unwrap();
        let resp: SendResp = bot
            .send(Message::text(
                "say hi to wecom bot power by rust".to_string(),
            ))
            .unwrap();

        assert_eq!(resp.err_code, 93000);
    }

    #[test]
    fn upload_media() {
        let bot = WeComBot::new("693a91f6-7xxx-4bc4-97a0-0ec2sifa5aaa").unwrap();
        let resp = bot
            .upload(
                crate::MediaType::File,
                "./src/tests/imgs/tiny-rust-logo.png",
            )
            .unwrap();

        assert_eq!(resp.err_code, 0);
        assert_ne!(resp.media_id, "");

        let resp: SendResp = bot.send(Message::file(resp.media_id)).unwrap();
        assert_eq!(resp.err_code, 0);
    }

    #[tokio::test]
    #[cfg(feature = "async_api")]
    async fn send_msg_async() {
        let bot = super::WeComBotAsync::new("693a91f6-7xxx-4bc4-97a0-0ec2sifa5aaa").unwrap();
        let resp: SendResp = bot
            .send(Message::markdown(
                "> say hi to wecom bot power by rust".to_string(),
            ))
            .await
            .unwrap();
        assert_eq!(resp.err_code, 93000);
    }

    #[tokio::test]
    #[cfg(feature = "async_api")]
    async fn upload_media_async() {
        let bot = super::WeComBotAsync::new("693a91f6-7xxx-4bc4-97a0-0ec2sifa5aaa").unwrap();
        let resp = bot
            .upload(
                crate::MediaType::File,
                "./src/tests/imgs/tiny-rust-logo.png",
            )
            .await
            .unwrap();

        assert_eq!(resp.err_code, 0);

        let resp: SendResp = bot.send(Message::file(resp.media_id)).await.unwrap();
        assert_eq!(resp.err_code, 0);
    }
}
