use std::any;
use std::fmt::Debug;
use std::time::Duration;

use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::message::Message;

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
}

type WeComResult<T> = Result<T, WeComError>;

pub struct WeComBot {
    url: String,
    client: reqwest::blocking::Client,
}

impl WeComBot {
    fn new(key: String) -> WeComResult<WeComBot> {
        WeComBotBuilder::new().key(key).build()
    }

    pub fn builder() -> WeComBotBuilder {
        WeComBotBuilder::new()
    }

    pub fn send<T>(&self, msg: Message) -> WeComResult<T>
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
}

impl Debug for WeComBot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeComBot").field("url", &self.url).finish()
    }
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
        if self.key.is_none() {
            return Err(WeComError::KeyNotFound);
        }

        let client = self.client.unwrap_or(
            reqwest::blocking::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        );

        Ok(WeComBot {
            client,
            url: format!(
                "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key={}",
                self.key.unwrap(),
            ),
        })
    }

    pub fn key(mut self, key: String) -> WeComBotBuilder {
        self.key = Some(key);
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
    client: reqwest::Client,
}

#[cfg(feature = "async_api")]
impl WeComBotAsync {
    fn new(key: String) -> WeComResult<WeComBotAsync> {
        WeComBotAsyncBuilder::new().key(key).build()
    }

    pub fn builder() -> WeComBotAsyncBuilder {
        WeComBotAsyncBuilder::new()
    }

    pub async fn send<T>(&self, msg: Message) -> WeComResult<T>
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
        if self.key.is_none() {
            return Err(WeComError::KeyNotFound);
        }

        let client = self.client.unwrap_or(
            reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
        );

        Ok(WeComBotAsync {
            client,
            url: format!(
                "https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key={}",
                self.key.unwrap(),
            ),
        })
    }

    pub fn key(mut self, key: String) -> WeComBotAsyncBuilder {
        self.key = Some(key);
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
        let bot = WeComBot::new("693a91f6-7xxx-4bc4-97a0-0ec2sifa5aaa".to_string()).unwrap();
        let resp: SendResp = bot
            .send(Message::text(
                "say hi to wecom bot power by rust".to_string(),
                None,
                None,
            ))
            .unwrap();

        assert_eq!(resp.err_code, 93000);
    }

    #[tokio::test]
    #[cfg(feature = "async_api")]
    async fn send_msg_async() {
        let bot =
            super::WeComBotAsync::new("693a91f6-7xxx-4bc4-97a0-0ec2sifa5aaa".to_string()).unwrap();
        let resp: SendResp = bot
            .send(Message::markdown(
                "> say hi to wecom bot power by rust".to_string(),
            ))
            .await
            .unwrap();
        assert_eq!(resp.err_code, 93000);
    }
}
