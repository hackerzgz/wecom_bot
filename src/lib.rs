#![allow(dead_code)]

//! A library for communicating with WeCom Bot instances.

mod bot;
mod message;

pub use bot::{WeComBot, WeComBotBuilder, WeComError};
pub use message::Message;

#[cfg(feature = "async_api")]
pub use bot::{WeComBotAsync, WeComBotAsyncBuilder};
