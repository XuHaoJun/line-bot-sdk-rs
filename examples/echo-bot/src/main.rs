use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use line_bot_sdk_messaging_api::{
    apis::{configuration::Configuration, messaging_api_api::reply_message},
    models::{Message, ReplyMessageRequest, TextMessage},
};
use line_bot_sdk_utils::signature::validate_signature;
use line_bot_sdk_webhook::models::{CallbackRequest, Event};
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() {
    // Get configuration from environment variables
    let channel_secret = env::var("CHANNEL_SECRET").expect("CHANNEL_SECRET must be set");
    let channel_access_token =
        env::var("CHANNEL_ACCESS_TOKEN").expect("CHANNEL_ACCESS_TOKEN must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // Create LINE Messaging API configuration
    let messaging_config = Configuration {
        bearer_access_token: Some(channel_access_token),
        ..Default::default()
    };

    // Build the application
    let app = Router::new()
        .route("/callback", post(webhook_handler))
        .with_state((channel_secret.clone(), messaging_config.clone()));

    // Start the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to address");

    println!("Echo bot listening on port {}", port);
    println!("Webhook URL: https://your.base.url/callback");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn webhook_handler(
    State((channel_secret, messaging_config)): State<(String, Configuration)>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    // Get signature from header
    let signature = match headers.get("x-line-signature") {
        Some(h) => match h.to_str() {
            Ok(s) => s,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid signature header").into_response();
            }
        },
        None => {
            return (StatusCode::BAD_REQUEST, "Missing x-line-signature header").into_response();
        }
    };

    // Validate signature
    match validate_signature(&body, &channel_secret, signature) {
        Ok(true) => {
            // Signature is valid, proceed
        }
        Ok(false) => {
            return (StatusCode::UNAUTHORIZED, "Invalid signature").into_response();
        }
        Err(e) => {
            eprintln!("Signature validation error: {}", e);
            return (StatusCode::BAD_REQUEST, "Signature validation failed").into_response();
        }
    }

    // Parse the webhook request
    let callback_request: CallbackRequest = match serde_json::from_slice(&body) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Failed to parse webhook request: {}", e);
            return (StatusCode::BAD_REQUEST, "Invalid request body").into_response();
        }
    };

    // Process each event
    let mut results = Vec::new();
    for event in callback_request.events {
        match handle_event(&event, &messaging_config).await {
            Ok(_) => results.push("ok".to_string()),
            Err(e) => {
                eprintln!("Error handling event: {}", e);
                results.push(format!("error: {}", e));
            }
        }
    }

    // Return success response
    (StatusCode::OK, "OK").into_response()
}

async fn handle_event(
    event: &Event,
    config: &Configuration,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if this is a message event with text content
    // The Event enum uses internally tagged enum, so we need to deserialize to get full MessageEvent
    let event_json = serde_json::to_value(event)?;
    
    // Check if it's a message event
    if event_json.get("type") != Some(&json!("message")) {
        return Ok(());
    }

    // Deserialize to get the full MessageEvent structure
    // We'll parse it manually from the JSON
    let reply_token = event_json
        .get("replyToken")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let reply_token = match reply_token {
        Some(token) => token,
        None => {
            // No reply token, ignore this event
            return Ok(());
        }
    };

    // Get the message content
    let message_obj = event_json
        .get("message")
        .ok_or("Missing message field")?;

    // Check if it's a text message
    if message_obj.get("type") != Some(&json!("text")) {
        return Ok(());
    }

    let text = message_obj
        .get("text")
        .and_then(|v| v.as_str())
        .ok_or("Missing text field")?
        .to_string();

    // Create TextMessage struct
    let text_message = TextMessage {
        r#type: Some("text".to_string()),
        quick_reply: None,
        sender: None,
        text: text.clone(),
        emojis: None,
        quote_token: None,
    };

    // Convert TextMessage struct to Message enum
    let message: Message = text_message.into();

    // Create reply request
    let reply_request = ReplyMessageRequest::new(reply_token, vec![message]);

    // Send reply
    reply_message(config, reply_request).await?;
    Ok(())
}

