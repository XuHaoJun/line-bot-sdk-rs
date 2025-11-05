# OpenAPI Generator for Rust LINE Bot SDK

This directory contains the code generation scripts for the LINE Bot SDK Rust packages.

## Problem

The Rust OpenAPI generator **does not support `allOf`** (see [feature matrix](https://openapi-generator.tech/docs/generators/rust)). The LINE Messaging API spec uses `allOf` to compose message types:

```yaml
FlexMessage:
  allOf:
    - $ref: "#/components/schemas/Message"  # Base: type, quickReply, sender
    - properties:
        altText: string      # Specific to FlexMessage
        contents: FlexContainer
```

This causes the generator to create **incomplete enum variants**:
- ✅ `FlexMessage` struct has all fields (type, quickReply, sender, altText, contents)
- ❌ `Message::FlexMessage` enum variant only has base fields (quickReply, sender)

This forces users to do ugly double serde conversions to work around the limitation.

## Solution

The `genertate-all.js` script now **preprocesses the OpenAPI specs** to flatten `allOf` before generation:

1. Reads original specs from `line-openapi/`
2. Flattens all `allOf` compositions into single schemas
3. Writes processed specs to `tmp/` directory (gitignored)
4. Generates Rust code from the flattened specs

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

- `genertate-all.js` - Main generation script with `allOf` flattening
- `projects.json` - List of OpenAPI specs to generate
- `openapi-generator-config.yaml` - OpenAPI generator configuration
- `README.md` - This file

## How It Works

The `flattenAllOf()` function:
1. Detects schemas with `allOf`
2. Resolves `$ref` references to other schemas
3. Merges all properties and required fields
4. Preserves discriminators
5. Returns a single flattened schema

Example transformation:

**Before (original spec):**
```yaml
FlexMessage:
  allOf:
    - $ref: "#/components/schemas/Message"
    - properties:
        altText: string
        contents: FlexContainer
```

**After (flattened):**
```yaml
FlexMessage:
  type: object
  properties:
    type: string
    quickReply: QuickReply
    sender: Sender
    altText: string
    contents: FlexContainer
```

## Result

After regeneration, you can use messages directly without double serde conversion:

```rust
// Before: Required double conversion
let message_json = serde_json::to_value(&flex_message)?;
let message: Message = serde_json::from_value(message_json)?;

// After: Direct usage (once regenerated)
let message = Message::FlexMessage {
    quick_reply: None,
    sender: None,
    alt_text: flex_message.alt_text,
    contents: flex_message.contents,
};
```

## Notes

- Original specs in `line-openapi/` are **never modified**
- Processed specs in `tmp/` are **gitignored** and regenerated each time
- This workaround is necessary until the Rust OpenAPI generator adds `allOf` support

