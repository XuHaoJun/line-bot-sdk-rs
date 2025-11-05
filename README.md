# LINE Bot SDK for Rust

A Rust SDK for the LINE Messaging API and related LINE platform services. This project provides type-safe, asynchronous Rust bindings for building LINE bots and integrations.

**Note:** This SDK is automatically generated from the official [LINE OpenAPI specifications](https://github.com/line/line-openapi) using [openapi-generator](https://github.com/OpenAPITools/openapi-generator), ensuring accurate and up-to-date API bindings.

## 📦 Packages

This repository is organized as a Cargo workspace containing the following packages:

| Package                                                                         | Description                                                          |
| ------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| [line-bot-sdk-messaging-api](packages/line-bot-sdk-messaging-api)               | LINE Messaging API for sending messages and managing rich content    |
| [line-bot-sdk-webhook](packages/line-bot-sdk-webhook)                           | Webhook event handling and models for receiving LINE platform events |
| [line-bot-sdk-channel-access-token](packages/line-bot-sdk-channel-access-token) | Channel Access Token management and authentication                   |
| [line-bot-sdk-insight](packages/line-bot-sdk-insight)                           | Analytics and insights API for bot statistics                        |
| [line-bot-sdk-liff](packages/line-bot-sdk-liff)                                 | LINE Front-end Framework (LIFF) app management                       |
| [line-bot-sdk-manage-audience](packages/line-bot-sdk-manage-audience)           | Audience management for targeted messaging                           |
| [line-bot-sdk-module](packages/line-bot-sdk-module)                             | LINE module management and chat control                              |
| [line-bot-sdk-module-attach](packages/line-bot-sdk-module-attach)               | Module attachment functionality                                      |
| [line-bot-sdk-shop](packages/line-bot-sdk-shop)                                 | LINE Shop API integration                                            |

## ✨ Features

- 🦀 **Fully asynchronous** - Built on `tokio` and `reqwest` for async/await support
- 🔒 **Type-safe** - Generated from OpenAPI specifications with strong typing
- 🔧 **Flexible TLS** - Support for both `native-tls` and `rustls-tls`
- 📝 **Well-documented** - Comprehensive API documentation for all endpoints
- 🎯 **Modular** - Use only the packages you need

## 🚀 Getting Started

### Installation

Add the packages you need to your `Cargo.toml`:

```toml
[dependencies]
# For sending messages
line-bot-sdk-messaging-api = "0.1.0"

# For handling webhooks
line-bot-sdk-webhook = "0.1.0"

# For managing access tokens
line-bot-sdk-channel-access-token = "0.1.0"
```

### Basic Usage

#### Sending a Text Message

```rust
use line_bot_sdk_messaging_api::{
    apis::configuration::Configuration,
    apis::messaging_api_api::messaging_api_push_message,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure the API client
    let config = Configuration {
        bearer_access_token: Some("YOUR_CHANNEL_ACCESS_TOKEN".to_string()),
        ..Default::default()
    };

    // Send a push message
    messaging_api_push_message(
        &config,
        push_message_request,
        None,
    ).await?;

    Ok(())
}
```

#### Managing Channel Access Tokens

```rust
use line_bot_sdk_channel_access_token::{
    apis::configuration::Configuration,
    apis::channel_access_token_api::issue_channel_access_token_v3,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();

    let response = issue_channel_access_token_v3(
        &config,
        Some("grant_type"),
        Some("client_id"),
        Some("client_secret"),
    ).await?;

    println!("Access Token: {}", response.access_token);
    Ok(())
}
```

## 🏗️ Project Structure

```
line-bot-sdk-rs/
├── line-openapi/          # OpenAPI specifications for LINE APIs
│   ├── messaging-api.yml
│   ├── webhook.yml
│   └── ...
├── openapi-generator/     # Generator configuration
├── packages/              # Individual SDK packages
│   ├── line-bot-sdk-messaging-api/
│   ├── line-bot-sdk-webhook/
│   └── ...
└── Cargo.toml            # Workspace configuration
```

## 📋 OpenAPI Specifications

This SDK is generated from the official [LINE OpenAPI specifications](https://github.com/line/line-openapi) maintained by LINE Corporation. The specifications are located in the `line-openapi/` directory and correspond to the following APIs:

| Specification File         | Package                           | API Documentation                                                                                         |
| -------------------------- | --------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `messaging-api.yml`        | line-bot-sdk-messaging-api        | [Messaging API](https://developers.line.biz/en/reference/messaging-api/)                                  |
| `webhook.yml`              | line-bot-sdk-webhook              | [Webhook Events](https://developers.line.biz/en/reference/messaging-api/#webhook-event-objects)           |
| `channel-access-token.yml` | line-bot-sdk-channel-access-token | [Channel Access Token API](https://developers.line.biz/en/reference/messaging-api/#channel-access-token)  |
| `insight.yml`              | line-bot-sdk-insight              | [Insight API](https://developers.line.biz/en/reference/messaging-api/#get-insight)                        |
| `liff.yml`                 | line-bot-sdk-liff                 | [LIFF API](https://developers.line.biz/en/reference/liff/)                                                |
| `manage-audience.yml`      | line-bot-sdk-manage-audience      | [Audience Group API](https://developers.line.biz/en/reference/messaging-api/#manage-audience)             |
| `module.yml`               | line-bot-sdk-module               | [Module API](https://developers.line.biz/en/reference/messaging-api/#acquire-control-api)                 |
| `module-attach.yml`        | line-bot-sdk-module-attach        | [Module Attach API](https://developers.line.biz/en/reference/partner-docs/#attach-by-operation-api)       |
| `shop.yml`                 | line-bot-sdk-shop                 | [Mission Stickers API](https://developers.line.biz/en/reference/messaging-api/#send-mission-stickers-api) |

The OpenAPI specifications are synchronized with LINE's public APIs and updated regularly to reflect the latest features and changes.

## 🛠️ Development

### Prerequisites

- Rust 1.70 or later
- Node.js (for OpenAPI generator tools)

### Building

Build all packages in the workspace:

```bash
cargo build
```

Build a specific package:

```bash
cargo build -p line-bot-sdk-messaging-api
```

### Testing

Run tests for all packages:

```bash
cargo test
```

### Generating from OpenAPI Specs

The SDK packages are generated from OpenAPI specifications located in the `line-openapi/` directory. To regenerate the SDK:

```bash
cd openapi-generator
node genertate-all.js
```

## 🔑 TLS Feature Flags

Each package supports two TLS backends:

- `native-tls` (default) - Uses the system's native TLS library
- `rustls-tls` - Uses the pure Rust `rustls` implementation

To use `rustls` instead:

```toml
[dependencies]
line-bot-sdk-messaging-api = { version = "0.1.0", default-features = false, features = ["rustls-tls"] }
```

## 📚 Documentation

Each package contains detailed API documentation in its `docs/` directory. You can also generate and view the Rust documentation:

```bash
cargo doc --open
```

## 🤝 Contributing

Contributions are welcome! Please note that the SDK code in the `packages/*/src/` directories is auto-generated from OpenAPI specifications. To contribute:

1. For SDK changes, modify the OpenAPI specifications in `line-openapi/`
2. For tooling/generation improvements, update the generator configuration in `openapi-generator/`
3. For documentation or examples, update the README files

## 📄 License

This project is licensed under the Unlicense - see individual package licenses for details.

## 🔗 Links

- [LINE Developers Documentation](https://developers.line.biz/)
- [LINE Messaging API Reference](https://developers.line.biz/en/reference/messaging-api/)
- [LINE Official Account Manager](https://manager.line.biz/)

## ⚠️ Disclaimer

This is an unofficial SDK. For the official LINE Bot SDK, please visit the [LINE Developers](https://developers.line.biz/) website.
