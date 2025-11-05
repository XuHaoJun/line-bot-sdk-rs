# Push Flex Message Example

A LINE bot example written in Rust that demonstrates how to send a flex message using the `push_message` API.

This example demonstrates:
- Building a complex flex message structure with hero image, body content, and footer buttons
- Converting Rust structs to internally tagged enum variants via JSON serialization
- Sending push messages to users using the LINE Messaging API

## Prerequisites

- Rust 1.70 or later
- A LINE channel with Messaging API enabled
- Channel Access Token from LINE Developers console
- A user ID to send the message to (you can get this from webhook events or use your own LINE user ID)

## Setup

### 1. Configure Environment Variables

Set the following environment variables:

```bash
export CHANNEL_ACCESS_TOKEN=your_channel_access_token
export USER_ID=your_user_id
```

To get your user ID:
- Add your bot as a friend
- Send a message to your bot
- Check the webhook event logs or use the [LINE Developers console](https://developers.line.biz/console/)

### 2. Build and Run

```bash
cd examples/push-flex-message
cargo run
```

The program will send a flex message to the specified user ID and print a success message if successful.

## Flex Message Structure

This example creates a flex message that includes:

- **Hero Section**: An image with a URI action
- **Body Section**: 
  - Title text ("Brown Cafe")
  - Rating display with gold and gray star icons
  - Place and Time information rows
- **Footer Section**: 
  - Two link buttons (CALL and WEBSITE)
  - Spacer box

## How It Works

1. **Configuration**: Reads `CHANNEL_ACCESS_TOKEN` and `USER_ID` from environment variables
2. **Flex Message Construction**: Builds the complete flex message structure using Rust structs:
   - `FlexMessage` for the message container
   - `FlexBubble` for the bubble container
   - `FlexBox` for layout containers
   - `FlexText` for text elements
   - `FlexImage` for the hero image
   - `FlexIcon` for star icons
   - `FlexButton` for action buttons
   - `UriAction` for URI actions
3. **Enum Conversion**: Converts structs to internally tagged enum variants (`Message`, `FlexContainer`, `FlexComponent`, `Action`) via JSON serialization/deserialization
4. **Message Sending**: Creates a `PushMessageRequest` and calls `push_message` API to send the message

## Code Structure

- `src/main.rs`: Main application entry point
  - `main()`: Sets up configuration and sends the message
  - `build_flex_message()`: Constructs the flex message structure matching the provided JSON

## Dependencies

- `tokio`: Async runtime
- `serde_json`: JSON serialization/deserialization
- `line-bot-sdk-messaging-api`: Messaging API client and models

## Converting JSON to Rust Structs

This example demonstrates how to convert JSON flex message structures to Rust code. The key pattern used is:

1. Create struct instances (e.g., `FlexText`, `FlexImage`, `FlexBox`)
2. Serialize to JSON using `serde_json::to_value()`
3. Deserialize to enum variants using `serde_json::from_value()`

This works because the SDK uses internally tagged enum serialization (`#[serde(tag = "type")]`), which requires the type field to be present in the JSON.

## Testing

1. Set the required environment variables
2. Run `cargo run`
3. Check your LINE app - you should receive a flex message with the cafe information

## Error Handling

The example includes basic error handling:
- Missing environment variables will cause a panic with a clear error message
- API errors will be returned as `Result` errors
- JSON serialization/deserialization errors are propagated up

## Further Reading

- [LINE Developers Documentation](https://developers.line.biz/)
- [LINE Messaging API Reference](https://developers.line.biz/en/reference/messaging-api/)
- [Flex Message Overview](https://developers.line.biz/en/docs/messaging-api/using-flex-messages/)
- [Flex Message JSON Reference](https://developers.line.biz/en/reference/messaging-api/#flex-message)

