#![allow(dead_code)]

//! A library for communicating with WeCom Bot instances.
//!
//! Example:
//!
//! ```rust
//! use wecom_bot::{WeComBot, Message, SendResp, WeComError};
//!
//! fn main() -> Result<(), WeComError> {
//!     // create a wecom bot with webhook key
//!     let bot = WeComBot::builder().key("693a91f6-7xxx-4bc4-97a0-0ec2sifa5aaa").build()?;
//!     let _rsp: SendResp = bot.send(Message::text("hello world!").mentioned_list(vec!["1001"]))?;
//!
//!     Ok(())
//! }
//! ```

mod bot;
mod image;
mod media;
mod message;
mod response;

pub use bot::{WeComBot, WeComBotBuilder, WeComError};
pub use image::Image;
pub use media::MediaType;
pub use message::{Article, Message};
pub use response::{SendResp, UploadResp};

#[cfg(feature = "async_api")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_api")))]
pub use bot::{WeComBotAsync, WeComBotAsyncBuilder};
