# Version 0.2.0 (not-release)

## Features

- Message type `image` and `news` support.
- Use `MessageBuilder` to set `mentioned_list` and `mentioned_mobile_list`.

## Performance

- Use `Cow<str>` instead of `String` in Message body to avoid memory copy.

# Version 0.1.0 (2023-03-28)

## Features

- Message type `text` and `markdown` support.
- Wecom bot **synchronize** / **asynchronize** client support.
