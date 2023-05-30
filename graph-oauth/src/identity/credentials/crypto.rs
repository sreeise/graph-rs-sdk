use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use graph_error::{AuthorizationFailure, AuthorizationResult};
use ring::rand::SecureRandom;

pub struct Crypto;

impl Crypto {
    /// Generate a secure 43-octet URL safe string for use as a nonce
    /// parameter or in the proof key for code exchange (PKCE) flow.
    ///
    /// Internally this method uses the Rust ring cyrpto library to
    /// generate a secure random 32-octet sequence that is base64 URL
    /// encoded (no padding). This sequence is hashed using SHA256 and
    /// base64 URL encoded (no padding) resulting in a 43-octet URL safe string.
    ///
    /// For more info on PKCE and entropy see: <https://tools.ietf.org/html/rfc7519#section-7.2>
    pub fn sha256_secure_string() -> AuthorizationResult<(String, String)> {
        let mut buf = [0; 32];

        let rng = ring::rand::SystemRandom::new();
        rng.fill(&mut buf)
            .map_err(|_| AuthorizationFailure::unknown("ring::error::Unspecified"))?;

        // Known as code_verifier in proof key for code exchange
        let base_64_random_string = URL_SAFE_NO_PAD.encode(buf);

        let mut context = ring::digest::Context::new(&ring::digest::SHA256);
        context.update(base_64_random_string.as_bytes());

        // Known as code_challenge in proof key for code exchange
        let secure_string = URL_SAFE_NO_PAD.encode(context.finish().as_ref());

        // code verifier, code challenge
        Ok((base_64_random_string, secure_string))
    }
}
