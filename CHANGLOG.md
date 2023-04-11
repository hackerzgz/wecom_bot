# Version 0.1.1 (not-release)

## Features

- Message type `image` and `news` support.

## Performance

- Use `Cow<str>` instead of `String` in Message body to avoid memory copy.

# Version 0.1.0 (2023-03-28)

## Features

- Message type `text` and `markdown` support.
- Wecom bot **synchronize** / **asynchronize** client support.
