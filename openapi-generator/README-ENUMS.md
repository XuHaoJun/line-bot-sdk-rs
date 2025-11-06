# Enum Post-Processing: v1 vs v2

## Overview

Both scripts solve the same problem: the OpenAPI Rust generator creates broken enum implementations for discriminated schemas. However, they use different approaches.

## The Problem

The Rust OpenAPI generator doesn't properly handle `allOf` with discriminators. When you have:

```yaml
Message:
  discriminator:
    propertyName: type
  properties:
    type: string
    quickReply: QuickReply

FlexMessage:
  allOf:
    - $ref: "#/components/schemas/Message"
    - properties:
        altText: string
        contents: FlexContainer
```

The generator creates an incomplete enum that's missing child-specific fields.

## Version 1: Field Duplication Approach (`post-process-enums.js`)

### How It Works
- Parses all fields from each struct
- Duplicates all fields inline in enum variants
- Uses `#[serde(tag = "type")]` for discrimination

### Generated Code
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
    // ... more variants
}

impl From<models::FlexMessage> for Message {
    fn from(value: models::FlexMessage) -> Self {
        Message::FlexMessage {
            quick_reply: value.quick_reply,
            sender: value.sender,
            alt_text: value.alt_text,
            contents: value.contents,
        }
    }
}
```

### Pros
- Explicit field mapping
- Uses Rust's tagged enum serde pattern

### Cons
- **Duplicates all struct fields in enum** (violates DRY)
- **Complex field parsing logic** (handles generics, multi-line types, etc.)
- **Requires inline enum detection** (must skip some enums to avoid compile errors)
- **Field-by-field From traits** (error-prone to maintain)
- **Generates 15 enums, skips 5** (FlexContainer, FlexComponent, Action, etc.)

## Version 2: Newtype Pattern Approach (`post-process-enums-v2.js`) ✨

### How It Works
- Wraps existing structs in `Box<models::Struct>`
- Uses `#[serde(untagged)]` for discrimination
- No field parsing needed!

### Generated Code
```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    TextMessage(Box<models::TextMessage>),
    FlexMessage(Box<models::FlexMessage>),
    // ... more variants
}

impl From<models::FlexMessage> for Message {
    fn from(value: models::FlexMessage) -> Self {
        Message::FlexMessage(Box::new(value))
    }
}
```

### Pros
- ✅ **No field duplication** - enum just wraps the struct
- ✅ **Much simpler code** - no field parsing needed
- ✅ **No inline enum detection needed** - always works if struct compiles
- ✅ **Cleaner From traits** - just `Box::new(value)`
- ✅ **Generates all 20 enums** - no need to skip any
- ✅ **Easier to maintain** - less code, less complexity

### Cons
- Uses `#[serde(untagged)]` which relies on trying variants in order during deserialization
- Slightly different API (but still supports `.into()`)

## Usage Comparison

Both versions support the same clean usage:

```rust
let flex_message = FlexMessage::new(alt_text, contents);
let message: Message = flex_message.into(); // Works with both v1 and v2!
```

Or explicitly:

```rust
// v1 and v2 both support this
let message = Message::from(flex_message);

// v2 also allows direct construction (if you prefer)
let message = Message::FlexMessage(Box::new(flex_message));
```

## Performance

Both versions have similar runtime performance:

- **v1**: Copies fields from struct to enum variant
- **v2**: Wraps struct in Box (one allocation)

The performance difference is negligible in practice.

## Recommendation

**Use v2 (`post-process-enums-v2.js`)** for new code:
- Simpler implementation
- Generates more enums (no skipping needed)
- Easier to maintain
- Same ergonomics for users

**v1 is kept** for:
- Backward compatibility
- Projects that specifically need `#[serde(tag = "type")]` behavior
- Reference implementation

## Running the Scripts

### Version 1
```bash
node post-process-enums.js <spec-path> <package-path>
```

### Version 2
```bash
node post-process-enums-v2.js <spec-path> <package-path>
```

### From generate-all.js

You can integrate either version in `genertate-all.js`:

```javascript
// Use v1
const { postProcessEnums } = require("./post-process-enums");
postProcessEnums(inputSpec, `./packages/${packageName}`);

// Or use v2
const { postProcessEnumsV2 } = require("./post-process-enums-v2");
postProcessEnumsV2(inputSpec, `./packages/${packageName}`);
```

## Statistics

| Metric | v1 | v2 |
|--------|----|----|
| Lines of code | 422 | 200 |
| Enums generated | 15 | 20 |
| Enums skipped | 5 | 0 |
| Complexity | High | Low |
| Field parsing | Yes | No |
| Inline enum detection | Yes | No |

## Conclusion

**v2 is the better approach** - it's simpler, generates more enums, and is easier to maintain. The newtype pattern is a cleaner solution that avoids all the complexity of v1 while providing the same user experience.

