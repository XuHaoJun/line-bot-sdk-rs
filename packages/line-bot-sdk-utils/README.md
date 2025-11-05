# LINE Bot SDK Utils

Utility functions for the LINE Bot SDK for Rust, including webhook signature validation.

## Features

- **Webhook Signature Validation**: Validate LINE webhook request signatures using HMAC-SHA256
- **Constant-time Comparison**: Prevents timing attacks during signature verification

## Installation

Add this package to your `Cargo.toml`:

```toml
[dependencies]
line-bot-sdk-utils = { path = "../packages/line-bot-sdk-utils" }
```

## Usage

### Signature Validation

Validate webhook signatures to ensure requests are from LINE servers:

```rust
use line_bot_sdk_utils::signature::validate_signature;

// Get the raw request body and signature header
let body = /* raw request body as bytes */;
let channel_secret = std::env::var("CHANNEL_SECRET")?;
let signature = /* X-Line-Signature header value */;

match validate_signature(&body, &channel_secret, &signature) {
    Ok(true) => {
        // Signature is valid, process the webhook
    }
    Ok(false) => {
        // Signature is invalid, reject the request
        return Err("Invalid signature");
    }
    Err(e) => {
        // Error during validation
        return Err(format!("Validation error: {}", e));
    }
}
```

### Example with Axum

```rust
use axum::{
    body::Bytes,
    extract::HeaderMap,
    http::StatusCode,
    response::Response,
};
use line_bot_sdk_utils::signature::validate_signature;

async fn webhook_handler(
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, StatusCode> {
    let signature = headers
        .get("x-line-signature")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let channel_secret = std::env::var("CHANNEL_SECRET")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match validate_signature(&body, &channel_secret, signature) {
        Ok(true) => {
            // Process webhook...
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body("OK".into())
                .unwrap())
        }
        Ok(false) => Err(StatusCode::UNAUTHORIZED),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
```

## API Reference

### `validate_signature`

Validates a LINE webhook signature.

**Signature:**
```rust
pub fn validate_signature(
    body: &[u8],
    channel_secret: &str,
    signature: &str,
) -> Result<bool, SignatureValidationError>
```

**Parameters:**
- `body`: The raw request body as bytes
- `channel_secret`: Your channel secret from LINE Developers console
- `signature`: The signature from the `X-Line-Signature` header (base64 encoded)

**Returns:**
- `Ok(true)` if the signature is valid
- `Ok(false)` if the signature is invalid
- `Err(SignatureValidationError)` if signature decoding fails

## How It Works

The signature validation follows LINE's specification:

1. Compute HMAC-SHA256 hash of the request body using the channel secret as the key
2. Base64 encode the hash
3. Compare the computed signature with the provided signature using constant-time comparison

This ensures that only requests from LINE servers are processed, preventing unauthorized access to your webhook endpoint.

## License

Unlicense

