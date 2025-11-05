# Echo Bot Example

A simple LINE bot example written in Rust that echoes back text messages sent by users.

This example demonstrates:
- Setting up an HTTP webhook server using Axum
- Validating LINE webhook signatures using `line-bot-sdk-utils`
- Parsing and handling webhook events
- Replying to text messages using the LINE Messaging API

## Prerequisites

- Rust 1.70 or later
- A LINE channel with webhook configured
- Channel Secret and Channel Access Token from LINE Developers console

## Setup

### 1. Configure Environment Variables

Set the following environment variables:

```bash
export CHANNEL_SECRET=your_channel_secret
export CHANNEL_ACCESS_TOKEN=your_channel_access_token
export PORT=3000  # Optional, defaults to 3000
```

### 2. Build and Run

```bash
cd examples/echo-bot
cargo run
```

The server will start listening on the configured port (default: 3000).

## Webhook Configuration

Configure your LINE webhook URL to point to:

```
https://your.base.url/callback
```

Replace `your.base.url` with your actual server URL. For local development, you can use a tunneling service like ngrok:

```bash
ngrok http 3000
```

Then use the ngrok URL (e.g., `https://abc123.ngrok.io/callback`) as your webhook URL in the LINE Developers console.

## How It Works

1. **Webhook Reception**: The server listens for POST requests on `/callback`
2. **Signature Validation**: Each request is validated using HMAC-SHA256 to ensure it's from LINE servers
3. **Event Processing**: The webhook payload is parsed into `CallbackRequest` containing an array of events
4. **Message Handling**: For each message event:
   - Checks if it's a text message
   - Extracts the message text
   - Creates a reply message with the same text
   - Sends the reply using the LINE Messaging API

## Code Structure

- `src/main.rs`: Main application entry point
  - Sets up Axum HTTP server
  - Configures webhook handler route
  - Implements event handling logic

## Dependencies

- `axum`: Modern async web framework
- `tokio`: Async runtime
- `serde_json`: JSON parsing
- `line-bot-sdk-webhook`: Webhook event models
- `line-bot-sdk-messaging-api`: Messaging API client
- `line-bot-sdk-utils`: Signature validation utilities

## Testing

1. Start the bot server
2. Send a text message to your LINE bot
3. The bot should echo back the same message

## Error Handling

The example includes basic error handling:
- Invalid or missing signatures return 401 Unauthorized
- Invalid request bodies return 400 Bad Request
- Event processing errors are logged but don't stop other events from being processed

## Further Reading

- [LINE Developers Documentation](https://developers.line.biz/)
- [LINE Messaging API Reference](https://developers.line.biz/en/reference/messaging-api/)
- [Axum Documentation](https://docs.rs/axum/)

