# Version 0.2.0 (not-release)

## Features

- Create WeComBot client Using static str, not `String`.
- Message type `image` and `news` support.
- For setting `mentioned_list` and `mentioned_mobile_list` as independent methods.
- Add `upload` API to send local media files to support the `file` message type.

## Performance

- Use `Cow<str>` instead of `String` in Message body to avoid memory copy.

# Version 0.1.0 (2023-03-28)

## Features

- Message type `text` and `markdown` support.
- Wecom bot **synchronize** / **asynchronize** client support.
