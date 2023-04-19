use base64::Engine;
use ring::rand::SecureRandom;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ProofKeyForCodeExchange {
    /// The code verifier is not included in the authorization URL.
    /// You can set the code verifier here and then use the From trait
    /// for [AuthorizationCodeCredential] which does use the code verifier.
    pub code_verifier: String,
    /// Used to secure authorization code grants by using Proof Key for Code Exchange (PKCE).
    /// Required if code_challenge_method is included. For more information, see the PKCE RFC.
    /// This parameter is now recommended for all application types, both public and confidential
    /// clients, and required by the Microsoft identity platform for single page apps using the
    /// authorization code flow.
    pub code_challenge: String,
    /// The method used to encode the code_verifier for the code_challenge parameter.
    /// This SHOULD be S256, but the spec allows the use of plain if the client can't support SHA256.
    ///
    /// If excluded, code_challenge is assumed to be plaintext if code_challenge is included.
    /// The Microsoft identity platform supports both plain and S256.
    /// For more information, see the PKCE RFC. This parameter is required for single page
    /// apps using the authorization code flow.
    pub code_challenge_method: String,
}

impl ProofKeyForCodeExchange {
    pub fn new<T: AsRef<str>>(
        code_verifier: T,
        code_challenge: T,
        code_challenge_method: T,
    ) -> ProofKeyForCodeExchange {
        ProofKeyForCodeExchange {
            code_verifier: code_verifier.as_ref().to_owned(),
            code_challenge: code_challenge.as_ref().to_owned(),
            code_challenge_method: code_challenge_method.as_ref().to_owned(),
        }
    }

    /// Generate a code challenge and code verifier for the
    /// authorization code grant flow using proof key for
    /// code exchange (PKCE) and SHA256.
    ///
    /// [ProofKeyForCodeExchange] contains a code_verifier,
    /// code_challenge, and code_challenge_method for use in the authorization code grant.
    ///
    /// For authorization, the code_challenge_method parameter in the request body
    /// is automatically set to 'S256'.
    ///
    /// Internally this method uses the Rust ring cyrpto library to
    /// generate a secure random 32-octet sequence that is base64 URL
    /// encoded (no padding). This sequence is hashed using SHA256 and
    /// base64 URL encoded (no padding) resulting in a 43-octet URL safe string.
    pub fn generate() -> anyhow::Result<ProofKeyForCodeExchange> {
        let mut buf = [0; 32];
        let rng = ring::rand::SystemRandom::new();
        rng.fill(&mut buf)
            .map_err(|_| anyhow::Error::msg("ring::error::Unspecified"))?;
        let code_verifier = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(buf);
        let mut context = ring::digest::Context::new(&ring::digest::SHA256);
        context.update(code_verifier.as_bytes());
        let code_challenge =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(context.finish().as_ref());

        Ok(ProofKeyForCodeExchange {
            code_verifier,
            code_challenge,
            code_challenge_method: "S256".to_owned(),
        })
    }
}
