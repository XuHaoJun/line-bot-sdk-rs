use line_bot_sdk_messaging_api::{
    apis::{configuration::Configuration, messaging_api_api::push_message},
    models::{
        Action, FlexBubble, FlexBox, FlexButton, FlexComponent, FlexContainer, FlexIcon,
        FlexImage, FlexMessage, FlexText, Message, PushMessageRequest, UriAction,
    },
};
use line_bot_sdk_messaging_api::models::flex_box::Layout;
use line_bot_sdk_messaging_api::models::flex_image::AspectMode;
use line_bot_sdk_messaging_api::models::flex_text::Weight;
use line_bot_sdk_messaging_api::models::flex_button::{Style, Height};
use std::env;

/// Helper trait to convert structs to their enum wrappers
/// Needed because OpenAPI generator doesn't support allOf in enum variants
trait IntoEnum<T> {
    fn into_enum(self) -> Result<T, serde_json::Error>;
}

impl<S: serde::Serialize, T: serde::de::DeserializeOwned> IntoEnum<T> for S {
    fn into_enum(self) -> Result<T, serde_json::Error> {
        serde_json::from_value(serde_json::to_value(self)?)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get configuration from environment variables
    let channel_access_token =
        env::var("CHANNEL_ACCESS_TOKEN").expect("CHANNEL_ACCESS_TOKEN must be set");
    let user_id = env::var("USER_ID").expect("USER_ID must be set");

    // Create LINE Messaging API configuration
    let messaging_config = Configuration {
        bearer_access_token: Some(channel_access_token),
        ..Default::default()
    };

    // Build the flex message structure
    let flex_message = build_flex_message()?;

    // Convert FlexMessage struct to Message enum
    let message: Message = flex_message.into_enum()?;

    // Create push message request
    let push_request = PushMessageRequest::new(user_id, vec![message]);

    // Send push message using the messaging API
    let response = push_message(&messaging_config, push_request, None).await?;

    println!("Successfully sent flex message!");
    println!("Response: {:?}", response);

    Ok(())
}

fn build_flex_message() -> Result<FlexMessage, Box<dyn std::error::Error>> {
    // Build hero image with URI action
    let hero_uri_action = UriAction {
        r#type: Some("uri".to_string()),
        label: None,
        uri: Some("https://line.me/".to_string()),
        alt_uri: None,
    };
    let hero_action: Action = hero_uri_action.into_enum()?;

    let hero_image = FlexImage {
        r#type: "image".to_string(),
        url: "https://developers-resource.landpress.line.me/fx/img/01_1_cafe.png".to_string(),
        flex: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        align: None,
        gravity: None,
        size: Some("full".to_string()),
        aspect_ratio: Some("20:13".to_string()),
        aspect_mode: Some(AspectMode::Cover),
        background_color: None,
        action: Some(Box::new(hero_action)),
        animated: None,
    };
    let hero_component: FlexComponent = hero_image.into_enum()?;

    // Build body content
    // Title text: "Brown Cafe"
    let title_text = FlexText {
        r#type: "text".to_string(),
        flex: None,
        text: Some("Brown Cafe".to_string()),
        size: Some("xl".to_string()),
        align: None,
        gravity: None,
        color: None,
        weight: Some(Weight::Bold),
        style: None,
        decoration: None,
        wrap: None,
        line_spacing: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        action: None,
        max_lines: None,
        contents: None,
        adjust_mode: None,
        scaling: None,
    };
    let title_component: FlexComponent = title_text.into_enum()?;

    // Rating stars and text
    let gold_star_icon = FlexIcon {
        r#type: Some("icon".to_string()),
        url: "https://developers-resource.landpress.line.me/fx/img/review_gold_star_28.png"
            .to_string(),
        size: Some("sm".to_string()),
        aspect_ratio: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        scaling: None,
    };
    let gold_star_component: FlexComponent = gold_star_icon.into_enum()?;

    let gray_star_icon = FlexIcon {
        r#type: Some("icon".to_string()),
        url: "https://developers-resource.landpress.line.me/fx/img/review_gray_star_28.png"
            .to_string(),
        size: Some("sm".to_string()),
        aspect_ratio: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        scaling: None,
    };
    let gray_star_component: FlexComponent = gray_star_icon.into_enum()?;

    let rating_text = FlexText {
        r#type: "text".to_string(),
        flex: Some(0),
        text: Some("4.0".to_string()),
        size: Some("sm".to_string()),
        align: None,
        gravity: None,
        color: Some("#999999".to_string()),
        weight: None,
        style: None,
        decoration: None,
        wrap: None,
        line_spacing: None,
        margin: Some("md".to_string()),
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        action: None,
        max_lines: None,
        contents: None,
        adjust_mode: None,
        scaling: None,
    };
    let rating_text_component: FlexComponent = rating_text.into_enum()?;

    // Rating box (baseline layout with stars and rating)
    let rating_box_contents = vec![
        gold_star_component.clone(),
        gold_star_component.clone(),
        gold_star_component.clone(),
        gold_star_component.clone(),
        gray_star_component,
        rating_text_component,
    ];
    let rating_box = FlexBox {
        r#type: Some("box".to_string()),
        layout: Layout::Baseline,
        flex: None,
        contents: rating_box_contents,
        spacing: None,
        margin: Some("md".to_string()),
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        background_color: None,
        border_color: None,
        border_width: None,
        corner_radius: None,
        width: None,
        max_width: None,
        height: None,
        max_height: None,
        padding_all: None,
        padding_top: None,
        padding_bottom: None,
        padding_start: None,
        padding_end: None,
        action: None,
        justify_content: None,
        align_items: None,
        background: None,
    };
    let rating_box_component: FlexComponent = rating_box.into_enum()?;

    // Place row
    let place_label = FlexText {
        r#type: "text".to_string(),
        flex: Some(1),
        text: Some("Place".to_string()),
        size: Some("sm".to_string()),
        align: None,
        gravity: None,
        color: Some("#aaaaaa".to_string()),
        weight: None,
        style: None,
        decoration: None,
        wrap: None,
        line_spacing: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        action: None,
        max_lines: None,
        contents: None,
        adjust_mode: None,
        scaling: None,
    };
    let place_label_component: FlexComponent = place_label.into_enum()?;

    let place_value = FlexText {
        r#type: "text".to_string(),
        flex: Some(5),
        text: Some("Flex Tower, 7-7-4 Midori-ku, Tokyo".to_string()),
        size: Some("sm".to_string()),
        align: None,
        gravity: None,
        color: Some("#666666".to_string()),
        weight: None,
        style: None,
        decoration: None,
        wrap: Some(true),
        line_spacing: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        action: None,
        max_lines: None,
        contents: None,
        adjust_mode: None,
        scaling: None,
    };
    let place_value_component: FlexComponent = place_value.into_enum()?;

    let place_row = FlexBox {
        r#type: Some("box".to_string()),
        layout: Layout::Baseline,
        flex: None,
        contents: vec![place_label_component, place_value_component],
        spacing: Some("sm".to_string()),
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        background_color: None,
        border_color: None,
        border_width: None,
        corner_radius: None,
        width: None,
        max_width: None,
        height: None,
        max_height: None,
        padding_all: None,
        padding_top: None,
        padding_bottom: None,
        padding_start: None,
        padding_end: None,
        action: None,
        justify_content: None,
        align_items: None,
        background: None,
    };
    let place_row_component: FlexComponent = place_row.into_enum()?;

    // Time row
    let time_label = FlexText {
        r#type: "text".to_string(),
        flex: Some(1),
        text: Some("Time".to_string()),
        size: Some("sm".to_string()),
        align: None,
        gravity: None,
        color: Some("#aaaaaa".to_string()),
        weight: None,
        style: None,
        decoration: None,
        wrap: None,
        line_spacing: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        action: None,
        max_lines: None,
        contents: None,
        adjust_mode: None,
        scaling: None,
    };
    let time_label_component: FlexComponent = time_label.into_enum()?;

    let time_value = FlexText {
        r#type: "text".to_string(),
        flex: Some(5),
        text: Some("10:00 - 23:00".to_string()),
        size: Some("sm".to_string()),
        align: None,
        gravity: None,
        color: Some("#666666".to_string()),
        weight: None,
        style: None,
        decoration: None,
        wrap: Some(true),
        line_spacing: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        action: None,
        max_lines: None,
        contents: None,
        adjust_mode: None,
        scaling: None,
    };
    let time_value_component: FlexComponent = time_value.into_enum()?;

    let time_row = FlexBox {
        r#type: Some("box".to_string()),
        layout: Layout::Baseline,
        flex: None,
        contents: vec![time_label_component, time_value_component],
        spacing: Some("sm".to_string()),
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        background_color: None,
        border_color: None,
        border_width: None,
        corner_radius: None,
        width: None,
        max_width: None,
        height: None,
        max_height: None,
        padding_all: None,
        padding_top: None,
        padding_bottom: None,
        padding_start: None,
        padding_end: None,
        action: None,
        justify_content: None,
        align_items: None,
        background: None,
    };
    let time_row_component: FlexComponent = time_row.into_enum()?;

    // Info box (vertical layout with place and time rows)
    let info_box = FlexBox {
        r#type: Some("box".to_string()),
        layout: Layout::Vertical,
        flex: None,
        contents: vec![place_row_component, time_row_component],
        spacing: Some("sm".to_string()),
        margin: Some("lg".to_string()),
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        background_color: None,
        border_color: None,
        border_width: None,
        corner_radius: None,
        width: None,
        max_width: None,
        height: None,
        max_height: None,
        padding_all: None,
        padding_top: None,
        padding_bottom: None,
        padding_start: None,
        padding_end: None,
        action: None,
        justify_content: None,
        align_items: None,
        background: None,
    };
    let info_box_component: FlexComponent = info_box.into_enum()?;

    // Body box (vertical layout with title, rating, and info)
    let body_box = FlexBox {
        r#type: Some("box".to_string()),
        layout: Layout::Vertical,
        flex: None,
        contents: vec![title_component, rating_box_component, info_box_component],
        spacing: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        background_color: None,
        border_color: None,
        border_width: None,
        corner_radius: None,
        width: None,
        max_width: None,
        height: None,
        max_height: None,
        padding_all: None,
        padding_top: None,
        padding_bottom: None,
        padding_start: None,
        padding_end: None,
        action: None,
        justify_content: None,
        align_items: None,
        background: None,
    };

    // Footer buttons
    let call_button_action = UriAction {
        r#type: Some("uri".to_string()),
        label: Some("CALL".to_string()),
        uri: Some("https://line.me/".to_string()),
        alt_uri: None,
    };
    let call_button_action_enum: Action = call_button_action.into_enum()?;

    let call_button = FlexButton {
        r#type: Some("button".to_string()),
        flex: None,
        color: None,
        style: Some(Style::Link),
        action: Box::new(call_button_action_enum),
        gravity: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        height: Some(Height::Sm),
        adjust_mode: None,
        scaling: None,
    };
    let call_button_component: FlexComponent = call_button.into_enum()?;

    let website_button_action = UriAction {
        r#type: Some("uri".to_string()),
        label: Some("WEBSITE".to_string()),
        uri: Some("https://line.me/".to_string()),
        alt_uri: None,
    };
    let website_button_action_enum: Action = website_button_action.into_enum()?;

    let website_button = FlexButton {
        r#type: Some("button".to_string()),
        flex: None,
        color: None,
        style: Some(Style::Link),
        action: Box::new(website_button_action_enum),
        gravity: None,
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        height: Some(Height::Sm),
        adjust_mode: None,
        scaling: None,
    };
    let website_button_component: FlexComponent = website_button.into_enum()?;

    // Empty spacer box
    let spacer_box = FlexBox {
        r#type: Some("box".to_string()),
        layout: Layout::Vertical,
        flex: None,
        contents: vec![],
        spacing: None,
        margin: Some("sm".to_string()),
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        background_color: None,
        border_color: None,
        border_width: None,
        corner_radius: None,
        width: None,
        max_width: None,
        height: None,
        max_height: None,
        padding_all: None,
        padding_top: None,
        padding_bottom: None,
        padding_start: None,
        padding_end: None,
        action: None,
        justify_content: None,
        align_items: None,
        background: None,
    };
    let spacer_box_component: FlexComponent = spacer_box.into_enum()?;

    // Footer box
    let footer_box = FlexBox {
        r#type: Some("box".to_string()),
        layout: Layout::Vertical,
        flex: Some(0),
        contents: vec![
            call_button_component,
            website_button_component,
            spacer_box_component,
        ],
        spacing: Some("sm".to_string()),
        margin: None,
        position: None,
        offset_top: None,
        offset_bottom: None,
        offset_start: None,
        offset_end: None,
        background_color: None,
        border_color: None,
        border_width: None,
        corner_radius: None,
        width: None,
        max_width: None,
        height: None,
        max_height: None,
        padding_all: None,
        padding_top: None,
        padding_bottom: None,
        padding_start: None,
        padding_end: None,
        action: None,
        justify_content: None,
        align_items: None,
        background: None,
    };

    // Build FlexBubble
    let bubble = FlexBubble {
        r#type: "bubble".to_string(),
        direction: None,
        styles: None,
        header: None,
        hero: Some(Box::new(hero_component)),
        body: Some(Box::new(body_box)),
        footer: Some(Box::new(footer_box)),
        size: None,
        action: None,
    };
    let flex_container: FlexContainer = bubble.into_enum()?;

    // Build FlexMessage
    let flex_message = FlexMessage {
        r#type: Some("flex".to_string()),
        quick_reply: None,
        sender: None,
        alt_text: "Flex Message".to_string(),
        contents: Box::new(flex_container),
    };

    Ok(flex_message)
}

