# Version 0.2.0 (2023-05-04)

## Features

- Creating `WeComBot` client through static str is not limited to just the `String` type.
- WeCom bot message type `image`, `news` and `file` support.
- For setting `mentioned_list` and `mentioned_mobile_list` as independent methods.
- Add `upload` API to send local media files to support the wecom bot message type `file`.

## Performance

- Use `Cow<str>` instead of `String` in message body to avoid memory copy.

# Version 0.1.0 (2023-03-28)

## Features

- Message type `text` and `markdown` support.
- Wecom bot **synchronize** / **asynchronize** client support.
