WeComBot
=====

This library implements an interface to communicate with a WeCom Bot instance. Not
all Message Type are implemented, but patches are welcome.

[![Build status](https://github.com/hackerzgz/wecom_bot/workflows/Rust/badge.svg)](https://github.com/hackerzgz/wecom_bot/actions)

### Usage

Add this to your `Cargo.toml` or run `cargo add wecom_bot`:

```toml
[dependencies]
wecom_bot = "0.0.1"
```

If you need to use __async client__:

```bash
$ cargo add wecom_bot --features=async_api
```

Here's a simple example that send a markdown message:

```rust
use wecom_bot::{WeComBot, Message, SendResp};

fn main() {
    let bot = WeComBot::new("YOUR-BOT-KEY".to_string());

    let rsp: SendResp = bot.send(Message::markdown("> hello world"));
    assert_eq!(rsp.err_code, 0);
}
```

### License

This project is licensed under Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
