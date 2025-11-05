//! Webhook signature validation for LINE Messaging API
//!
//! This module provides functions to validate webhook request signatures
//! to ensure requests are actually sent from LINE servers.

use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Validates a LINE webhook signature.
///
/// This function computes an HMAC-SHA256 hash of the request body using
/// the channel secret as the key, then compares it with the provided signature
/// using constant-time comparison to prevent timing attacks.
///
/// # Arguments
///
/// * `body` - The raw request body as bytes
/// * `channel_secret` - Your channel secret from LINE Developers console
/// * `signature` - The signature from the `X-Line-Signature` header (base64 encoded)
///
/// # Returns
///
/// Returns `Ok(true)` if the signature is valid, `Ok(false)` if invalid,
/// or an error if signature decoding fails.
///
/// # Example
///
/// ```no_run
/// use line_bot_sdk_utils::signature::validate_signature;
///
/// let body = b"{\"events\":[]}";
/// let channel_secret = "your_channel_secret";
/// let signature = "base64_encoded_signature";
///
/// match validate_signature(body, channel_secret, signature) {
///     Ok(true) => println!("Signature is valid"),
///     Ok(false) => println!("Signature is invalid"),
///     Err(e) => eprintln!("Error validating signature: {}", e),
/// }
/// ```
pub fn validate_signature(
    body: &[u8],
    channel_secret: &str,
    signature: &str,
) -> Result<bool, SignatureValidationError> {
    // Decode the base64 signature
    let expected_signature = general_purpose::STANDARD
        .decode(signature)
        .map_err(|_| SignatureValidationError::InvalidSignatureFormat)?;

    // Create HMAC-SHA256 hasher with channel secret as key
    let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes())
        .map_err(|_| SignatureValidationError::InvalidKey)?;

    // Update with request body
    mac.update(body);

    // Get the computed signature
    let computed_signature = mac.finalize().into_bytes();

    // Constant-time comparison to prevent timing attacks
    if expected_signature.len() != computed_signature.len() {
        return Ok(false);
    }

    // Use constant-time comparison
    let mut result = 0u8;
    for (a, b) in expected_signature.iter().zip(computed_signature.iter()) {
        result |= a ^ b;
    }

    Ok(result == 0)
}

/// Errors that can occur during signature validation
#[derive(Debug)]
pub enum SignatureValidationError {
    /// The signature format is invalid (not valid base64)
    InvalidSignatureFormat,
    /// The channel secret key is invalid
    InvalidKey,
}

impl std::fmt::Display for SignatureValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignatureValidationError::InvalidSignatureFormat => {
                write!(f, "Invalid signature format: signature must be base64 encoded")
            }
            SignatureValidationError::InvalidKey => {
                write!(f, "Invalid channel secret key")
            }
        }
    }
}

impl std::error::Error for SignatureValidationError {}

