#![allow(dead_code)]

mod bot;
mod message;

pub use bot::{WeComBot, WeComBotBuilder};
pub use message::Message;

#[cfg(feature = "async_api")]
pub use bot::{WeComBotAsync, WeComBotAsyncBuilder};
