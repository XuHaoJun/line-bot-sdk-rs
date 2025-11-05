# Solution: Fixing the Double Serde Conversion Problem

## Root Cause

**The Rust OpenAPI generator does NOT support `allOf`.**

From the [official documentation](https://openapi-generator.tech/docs/generators/rust):

```
Schema Support Feature
allOf    ✗    OAS2,OAS3
```

### Why This Causes Problems

The LINE Messaging API spec uses `allOf` inheritance:

```yaml
Message:
  type: object
  discriminator:
    propertyName: type
  properties:
    type: string
    quickReply: QuickReply
    sender: Sender

FlexMessage:
  allOf:
    - $ref: "#/components/schemas/Message"  # Inherit base fields
    - type: object
      properties:
        altText: string       # Add specific fields
        contents: FlexContainer
```

The generator produces:
- ✅ **Complete struct**: `FlexMessage` has all fields (type, quickReply, sender, altText, contents)
- ❌ **Incomplete enum**: `Message::FlexMessage` only has inherited fields (quickReply, sender)

This forced ugly workarounds:

```rust
// Double serde conversion required!
let message_json = serde_json::to_value(&flex_message)?;
let message: Message = serde_json::from_value(message_json)?;
```

## The Solution

### Preprocess the OpenAPI Spec to Flatten `allOf`

Modified `genertate-all.js` to:

1. **Read** original specs from `line-openapi/`
2. **Flatten** all `allOf` compositions into single schemas
3. **Write** processed specs to `tmp/` directory (gitignored)
4. **Generate** Rust code from the flattened specs

### Key Features

✅ **Non-destructive**: Original specs remain unchanged  
✅ **Automatic**: Runs on every generation  
✅ **Complete**: Produces enum variants with all fields  
✅ **Clean**: No more double serde conversions needed

## Files Modified

1. **`openapi-generator/genertate-all.js`**
   - Added `flattenAllOf()` function to flatten `allOf` compositions
   - Added `processOpenAPISpec()` to process and save flattened specs
   - Modified generation to use processed specs from `tmp/`

2. **`openapi-generator/package.json`** (new)
   - Added `js-yaml` dependency for YAML parsing

3. **`openapi-generator/README.md`** (new)
   - Documentation explaining the problem and solution

4. **`examples/push-flex-message/src/main.rs`**
   - Kept temporary serde workaround until regeneration
   - Added comments explaining it will be removed after regeneration

## Usage

### Step 1: Install Dependencies

```bash
cd openapi-generator
npm install
```

### Step 2: Regenerate Packages

```bash
node genertate-all.js
```

This will:
- Flatten `allOf` in all OpenAPI specs
- Generate complete Rust code with all fields in enum variants
- Save to `packages/` directory

### Step 3: Update Example Code

After regeneration, you can remove the serde workaround:

```rust
// OLD (before regeneration):
let message_json = serde_json::to_value(&flex_message)?;
let message: Message = serde_json::from_value(message_json)?;

// NEW (after regeneration):
// Just use the FlexMessage directly, or construct the enum variant:
let message = Message::FlexMessage {
    quick_reply: flex_message.quick_reply,
    sender: flex_message.sender,
    alt_text: flex_message.alt_text,
    contents: flex_message.contents,
};
```

## How the Flattening Works

### Example Transformation

**Input (original spec with `allOf`):**
```yaml
Message:
  type: object
  properties:
    type: string
    quickReply: QuickReply
    sender: Sender
  discriminator:
    propertyName: type

FlexMessage:
  allOf:
    - $ref: "#/components/schemas/Message"
    - type: object
      required: [altText, contents]
      properties:
        altText: string
        contents: FlexContainer
```

**Output (flattened):**
```yaml
Message:
  type: object
  properties:
    type: string
    quickReply: QuickReply
    sender: Sender
  discriminator:
    propertyName: type

FlexMessage:
  type: object
  required: [type, altText, contents]
  properties:
    type: string          # From Message
    quickReply: QuickReply # From Message
    sender: Sender        # From Message
    altText: string       # From FlexMessage
    contents: FlexContainer # From FlexMessage
  discriminator:          # Preserved from Message
    propertyName: type
```

### Generated Rust Code

**Before (incomplete enum):**
```rust
pub enum Message {
    #[serde(rename="flex")]
    FlexMessage {
        quick_reply: Option<Box<QuickReply>>,
        sender: Option<Box<Sender>>,
        // Missing: altText and contents!
    },
}
```

**After (complete enum):**
```rust
pub enum Message {
    #[serde(rename="flex")]
    FlexMessage {
        quick_reply: Option<Box<QuickReply>>,
        sender: Option<Box<Sender>>,
        alt_text: String,              // Now included!
        contents: Box<FlexContainer>,  // Now included!
    },
}
```

## Benefits

1. ✅ **No more double serde conversion**
2. ✅ **Type-safe message construction**
3. ✅ **Better performance** (no JSON serialization/deserialization)
4. ✅ **Cleaner code**
5. ✅ **Automatic** - runs on every generation
6. ✅ **Safe** - original specs never modified

## Notes

- The `tmp/` directory is gitignored and regenerated each time
- Original OpenAPI specs in `line-openapi/` remain unchanged
- This workaround is necessary until the Rust generator adds native `allOf` support
- The solution works for all message types (Text, Image, Video, Flex, etc.)

