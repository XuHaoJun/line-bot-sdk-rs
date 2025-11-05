# Quick Start: Fixing the Double Serde Issue

## TL;DR

The double serde conversion exists because **Rust OpenAPI generator doesn't support `allOf`**.

**Solution**: The script now automatically flattens `allOf` before generation.

## Usage

```bash
# 1. Install dependencies (first time only)
cd openapi-generator
npm install

# 2. Regenerate all packages
node genertate-all.js
```

## What Happens

1. ✅ Reads OpenAPI specs from `line-openapi/`
2. ✅ Flattens `allOf` compositions
3. ✅ Saves to `tmp/` (gitignored, temporary)
4. ✅ Generates complete Rust code to `packages/`

## Result

### Before Regeneration
```rust
// Ugly double serde workaround needed
let message_json = serde_json::to_value(&flex_message)?;
let message: Message = serde_json::from_value(message_json)?;
```

### After Regeneration
```rust
// Clean, direct usage - no workaround needed!
let message = Message::FlexMessage {
    quick_reply: None,
    sender: None,
    alt_text: "Restaurant Finder".to_string(),
    contents: Box::new(flex_container),
};
```

## Files Changed

- ✅ `genertate-all.js` - Added `allOf` flattening logic
- ✅ `package.json` - Added `js-yaml` dependency
- ✅ `README.md`, `SOLUTION.md`, `QUICKSTART.md` - Documentation

## Next Steps

After regeneration, update your example code to remove the serde workaround in:
- `examples/push-flex-message/src/main.rs`
- `examples/echo-bot/src/main.rs`

See `SOLUTION.md` for detailed explanation of the root cause and fix.

