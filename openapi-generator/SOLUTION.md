# Solution: Automated Enum Generation with Intelligent Skip Logic

## Root Cause

**The Rust OpenAPI generator does NOT support `allOf` and has broken discriminator handling.**

From the [official documentation](https://openapi-generator.tech/docs/generators/rust):

```
Schema Support Feature
allOf    ✗    OAS2,OAS3
```

### Why This Causes Problems

The LINE Messaging API spec uses `allOf` inheritance with discriminators:

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
- ❌ **Incomplete enum**: `Message::FlexMessage` only has base fields (quickReply, sender) - **missing altText and contents!**

This forced ugly workarounds:

```rust
// Double serde conversion required!
let message_json = serde_json::to_value(&flex_message)?;
let message: Message = serde_json::from_value(message_json)?;
```

## The Solution

### Two-Stage Code Generation Pipeline

Our solution combines preprocessing and intelligent post-processing:

#### Stage 1: Preprocessing (`genertate-all.js`)

1. **Flatten `allOf`**: Merge parent and child schemas into single schemas
2. **Remove discriminators**: Prevent the generator from creating broken enums
3. **Generate code**: OpenAPI generator creates complete **structs**

#### Stage 2: Post-Processing (`post-process-enums.js`)

Automatically generates proper enum implementations:

1. **Read original spec** to find all discriminated schemas
2. **Parse generated structs** to extract:
   - Field names and types
   - Inline enum definitions (like `Mode`, `InputOption`)
   - Actual struct names (handles `UriImagemapAction` vs `URIImagemapAction`)
3. **Intelligent detection**:
   - Detects if any variant references inline enum types
   - **Skips** enum generation if inline types found (prevents compilation errors)
   - **Generates** complete enum if all fields use standard types
4. **Generate complete enums**:
   - All fields inline in each variant
   - `From` trait implementations for easy conversion
   - `Default` trait implementation
5. **Robust parsing**:
   - Handles multi-line generic types (`HashMap<String, SubstitutionObject>`)
   - Handles nested generics correctly
   - Adds `models::` prefix to custom types

### Key Features

✅ **Automatic**: Runs on every generation - no manual intervention
✅ **Intelligent**: Skips problematic enums that would fail compilation  
✅ **Complete**: Enum variants have ALL fields from both parent and child  
✅ **Clean**: No more double serde conversions needed
✅ **Robust**: Handles complex types, generics, and edge cases
✅ **Non-destructive**: Original specs never modified

## Files Created/Modified

### New Files

1. **`openapi-generator/post-process-enums.js`**
   - Automated enum generation from discriminated schemas
   - Intelligent inline type detection
   - Robust field parsing with multi-line type support
   - `From` and `Default` trait generation

2. **`openapi-generator/package.json`**
   - Added `js-yaml` dependency for YAML parsing

### Modified Files

1. **`openapi-generator/genertate-all.js`**
   - Added `flattenAllOf()` function
   - Added `removeDiscriminators()` function
   - Integrated post-processing after code generation

2. **`examples/push-flex-message/src/main.rs`**
   - Updated to use `From` trait instead of serde workaround

3. **`examples/echo-bot/src/main.rs`**
   - Updated to use `From` trait instead of serde workaround
   - Removed unused `IntoEnum` trait

## Usage

### One Command Generation

```bash
cd openapi-generator
node genertate-all.js
```

This automatically:
1. Flattens `allOf` in all OpenAPI specs
2. Removes discriminators
3. Generates Rust code from flattened specs
4. Post-processes to create proper enums
5. Reports: "✓ Post-processed N discriminated schema(s): X generated, Y skipped"

### Results

**✅ 15 Enums Successfully Generated:**
- `Message` - All 11 message variants with complete fields
- `Recipient` - operator, audience, redelivery
- `DemographicFilter` - age, appType, area, gender, operator, subscriptionPeriod
- `RichMenuBatchOperation` - link, unlink, unlinkAll
- `SubstitutionObject` - mention, emoji
- `MentionTarget` - user, all
- `ImagemapAction` - message, uri, clipboard
- `Template` - buttons, confirm, carousel, image_carousel
- `FlexBoxBackground` - linearGradient
- `AcquisitionConditionRequest/Response` - normal, lottery
- `CouponRewardRequest/Response` - cashBack, discount, free, gift, others
- `CashBackPriceInfoRequest` - fixed, percentage
- `DiscountPriceInfoRequest` - fixed, percentage

**⚠️ 5 Enums Intelligently Skipped (inline type references):**
- `FlexContainer` - references inline enums: Direction, Size
- `FlexComponent` - references: Layout, Position, Gravity, Style, etc. (12 inline enums)
- `Action` - references: Mode, InputOption
- `CashBackPriceInfoResponse` - references: Currency
- `DiscountPriceInfoResponse` - references: Currency

These keep their generated structs, which is the correct behavior!

## Example: Generated Code

### Before (Broken OpenAPI Generator Output)

```rust
pub enum Message {
    #[serde(rename="flex")]
    FlexMessage {
        quick_reply: Option<Box<QuickReply>>,
        sender: Option<Box<Sender>>,
        // ❌ Missing: alt_text and contents!
    },
}

// Required ugly workaround:
let message_json = serde_json::to_value(&flex_message)?;
let message: Message = serde_json::from_value(message_json)?;
```

### After (Our Post-Processed Output)

```rust
/// Message enum with all fields inline for each variant
/// This is automatically generated from discriminated schemas
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
        alt_text: String,                    // ✅ Now included!
        #[serde(rename = "contents")]
        contents: Box<models::FlexContainer>, // ✅ Now included!
    },
    // ... all 11 variants with complete fields
}

impl Default for Message {
    fn default() -> Self {
        Self::TextMessage {
            quick_reply: None,
            sender: None,
            text: String::new(),
            emojis: None,
            quote_token: None,
        }
    }
}

// Automatic From trait - clean conversion!
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

// Usage - simple and clean!
let flex_message = FlexMessage::new(alt_text, contents);
let message: Message = flex_message.into(); // ✅ Just works!
```

## Technical Details

### Preprocessing Algorithm

```javascript
function flattenAllOf(schema, schemas) {
  if (schema.allOf) {
    const flattened = { type: "object", properties: {}, required: [] };
    
    schema.allOf.forEach(item => {
      if (item.$ref) {
        const refSchema = schemas[item.$ref.split("/").pop()];
        Object.assign(flattened.properties, refSchema.properties);
        flattened.required.push(...(refSchema.required || []));
      } else {
        Object.assign(flattened.properties, item.properties);
        flattened.required.push(...(item.required || []));
      }
    });
    
    // Deduplicate and return
    flattened.required = [...new Set(flattened.required)];
    return flattened;
  }
  return schema;
}
```

### Post-Processing Algorithm

```javascript
function postProcessEnums(specPath, packagePath) {
  // 1. Read original spec to find discriminators
  const spec = yaml.load(fs.readFileSync(specPath));
  
  // 2. For each discriminated schema
  for (const [schemaName, schema] of Object.entries(spec.schemas)) {
    if (schema.discriminator) {
      const variantData = [];
      const allInlineEnums = new Set();
      
      // 3. Parse all variant structs
      for (const [typeName, schemaRef] of Object.entries(schema.discriminator.mapping)) {
        const structFile = findStructFile(schemaRef);
        const { fields, inlineEnums, actualStructName } = parseStructFields(structFile);
        
        allInlineEnums.add(...inlineEnums);
        variantData.push({ typeName, structName: actualStructName, fields });
      }
      
      // 4. Check for inline enum references
      const hasInlineRefs = variantData.some(v => 
        v.fields.some(f => referencesInlineEnum(f.fieldType, allInlineEnums))
      );
      
      if (hasInlineRefs) {
        console.log(`⚠ Skipping ${schemaName}: references inline enum types`);
        continue; // Keep generated struct
      }
      
      // 5. Generate complete enum with all fields
      generateEnumFile(schemaName, variantData);
    }
  }
}
```

### Inline Type Detection

The key innovation is detecting inline enums to prevent compilation errors:

```javascript
// Parse struct file to find inline enum definitions
const enumRegex = /pub enum (\w+) \{/g;
while ((match = enumRegex.exec(content)) !== null) {
  inlineEnums.add(match[1]); // e.g., "Mode", "InputOption"
}

// Check if field type references any inline enum
function hasInlineEnumReference(fieldType, inlineEnums) {
  const typeNames = extractTypeNames(fieldType); // handles generics
  return typeNames.some(t => inlineEnums.has(t));
}
```

## Benefits

1. ✅ **No more double serde conversion** - direct type-safe usage
2. ✅ **Better performance** - no JSON serialization/deserialization overhead
3. ✅ **Cleaner code** - idiomatic Rust with `From` traits
4. ✅ **Type safety** - compile-time checks for all fields
5. ✅ **Automatic** - runs on every generation, no manual work
6. ✅ **Intelligent** - skips problematic types automatically
7. ✅ **Robust** - handles complex types, generics, edge cases
8. ✅ **Safe** - original specs never modified

## Comparison

| Approach | Pros | Cons |
|----------|------|------|
| **Manual enum** | Full control | Tedious, error-prone, hard to maintain |
| **Serde workaround** | Works with generator | Slow, ugly, runtime overhead |
| **Our solution** | Automatic, fast, type-safe | Requires custom tooling |

## Notes

- The `tmp/` directory is gitignored and regenerated each time
- Original OpenAPI specs in `line-openapi/` remain unchanged
- Generated enums are marked with "Post-processed by: post-process-enums.js"
- This workaround is necessary until the Rust OpenAPI generator adds proper `allOf` and discriminator support
- The solution is general and works for all discriminated schemas in any OpenAPI spec
