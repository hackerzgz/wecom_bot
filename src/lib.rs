#![allow(dead_code)]

//! A library for communicating with WeCom Bot instances.

mod bot;
mod image;
mod message;
mod upload;

pub use bot::{WeComBot, WeComBotBuilder, WeComError};
pub use image::Image;
pub use message::{Message, SendResp};
pub use upload::{MediaType, UploadResp};

#[cfg(feature = "async_api")]
pub use bot::{WeComBotAsync, WeComBotAsyncBuilder};
