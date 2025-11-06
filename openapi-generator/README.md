# OpenAPI Generator for Rust LINE Bot SDK

This directory contains the code generation scripts for the LINE Bot SDK Rust packages.

## Problem

The Rust OpenAPI generator **does not support `allOf`** (see [feature matrix](https://openapi-generator.tech/docs/generators/rust)). The LINE Messaging API spec uses `allOf` to compose message types with discriminators:

```yaml
Message:
  discriminator:
    propertyName: type
  properties:
    type: string
    quickReply: QuickReply
    sender: Sender

FlexMessage:
  allOf:
    - $ref: "#/components/schemas/Message"  # Base: type, quickReply, sender
    - properties:
        altText: string      # Specific to FlexMessage
        contents: FlexContainer
```

This causes the generator to create **incomplete enum variants**:
- ✅ `FlexMessage` struct has all fields (type, quickReply, sender, altText, contents)
- ❌ `Message::FlexMessage` enum variant only has base fields (quickReply, sender) - **missing altText and contents!**

This forces users to do ugly double serde conversions to work around the limitation.

## Solution

Our generation pipeline has **two stages** to solve this problem:

### Stage 1: Preprocessing (`genertate-all.js`)

1. Reads original specs from `line-openapi/`
2. **Flattens all `allOf` compositions** into single schemas
3. **Removes discriminators** (they cause broken enum generation)
4. Writes processed specs to `tmp/` directory (gitignored)
5. Generates Rust code from the flattened specs

This produces complete **structs** with all fields.

### Stage 2: Post-processing (`post-process-enums.js`)

After code generation, automatically:

1. Finds all discriminated schemas in the original spec
2. Parses the generated struct files to extract field information
3. **Automatically generates proper enum implementations** with all fields inline
4. **Intelligently skips** enums that reference inline types (would cause compilation errors)
5. Generates `From` trait implementations for easy conversion
6. Adds `Default` trait implementations

This produces **complete enum variants** with all fields, eliminating the need for serde workarounds.

## Usage

### Prerequisites

Install the `js-yaml` dependency:

```bash
npm install js-yaml
```

### Generate All Packages

```bash
node openapi-generator/genertate-all.js
```

This will:
1. Process each spec to flatten `allOf`
2. Save processed specs to `tmp/`
3. Generate Rust packages to `packages/`

### Files

- `genertate-all.js` - Main generation script with `allOf` flattening and post-processing integration
- `post-process-enums.js` - Automated enum generation from discriminated schemas
- `projects.json` - List of OpenAPI specs to generate
- `openapi-generator-config.yaml` - OpenAPI generator configuration
- `README.md` - This file
- `SOLUTION.md` - Detailed technical documentation

## How It Works

### Preprocessing (`flattenAllOf()`)

1. Detects schemas with `allOf`
2. Resolves `$ref` references to other schemas
3. Merges all properties and required fields
4. Removes discriminators (to prevent broken enum generation)
5. Returns a single flattened schema

### Post-processing (`postProcessEnums()`)

1. Reads original spec to find discriminated schemas
2. For each discriminated schema:
   - Parses all variant struct files
   - Extracts field types and inline enum definitions
   - Checks if any field references inline enums
   - **Skips** generation if inline enum references found (prevents compilation errors)
   - **Generates** complete enum with all fields inline if safe
3. Creates `From` trait implementations for struct → enum conversion
4. Adds `Default` trait implementation

### Example: Generated Enum

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "flex")]
    FlexMessage {
        #[serde(rename = "quickReply", skip_serializing_if = "Option::is_none")]
        quick_reply: Option<Box<models::QuickReply>>,
        #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
        sender: Option<Box<models::Sender>>,
        #[serde(rename = "altText")]
        alt_text: String,
        #[serde(rename = "contents")]
        contents: Box<models::FlexContainer>,
    },
    // ... other variants
}

// Automatic From trait implementation
impl From<models::FlexMessage> for Message {
    fn from(msg: models::FlexMessage) -> Self {
        Message::FlexMessage {
            quick_reply: msg.quick_reply,
            sender: msg.sender,
            alt_text: msg.alt_text,
            contents: msg.contents,
        }
    }
}
```

## Result

✅ **15 enums auto-generated** with all fields:
- Message, Recipient, DemographicFilter, RichMenuBatchOperation
- SubstitutionObject, MentionTarget, ImagemapAction, Template
- FlexBoxBackground, AcquisitionConditionRequest/Response
- CouponRewardRequest/Response, and more

⚠️ **5 enums intelligently skipped** (reference inline enum types):
- FlexContainer, FlexComponent, Action
- CashBackPriceInfoResponse, DiscountPriceInfoResponse

These skipped types keep their generated structs - perfect for cases where enum generation would cause compilation errors.

### Usage in Code

```rust
// Simple and clean - no double conversion!
let flex_message = FlexMessage::new(alt_text, contents);
let message: Message = flex_message.into();

// Or construct directly:
let message = Message::FlexMessage {
    quick_reply: None,
    sender: None,
    alt_text: "Hello".to_string(),
    contents: Box::new(flex_container),
};
```

## Notes

- Original specs in `line-openapi/` are **never modified**
- Processed specs in `tmp/` are **gitignored** and regenerated each time
- This workaround is necessary until the Rust OpenAPI generator adds `allOf` support

