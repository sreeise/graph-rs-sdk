use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use graph_error::{IdentityResult, AF};
use ring::rand::SecureRandom;

/*
pub(crate) fn sha256_secure_string() -> IdentityResult<(String, String)> {
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
 */

pub trait GenPkce {
    fn code_challenge_method() -> String {
        "S256".into()
    }

    /// Known as code_verifier in proof key for code exchange
    /// Uses the Rust ring crypto library to generate a secure random
    /// 32-octet sequence that is base64 URL encoded (no padding)
    fn code_verifier() -> String {
        let mut buf = [0; 32];

        let rng = ring::rand::SystemRandom::new();
        rng.fill(&mut buf).expect("ring::error::Unspecified");

        URL_SAFE_NO_PAD.encode(buf)
    }

    fn code_challenge(code_verifier: &String) -> String {
        let mut context = ring::digest::Context::new(&ring::digest::SHA256);
        context.update(code_verifier.as_bytes());

        // Known as code_challenge in proof key for code exchange
        let code_challenge = URL_SAFE_NO_PAD.encode(context.finish().as_ref());

        // code verifier, code challenge
        code_challenge
    }

    /// Generate a code challenge and code verifier for the
    /// authorization code grant flow using proof key for
    /// code exchange (PKCE) and SHA256.
    ///
    /// [ProofKeyCodeExchange] contains a code_verifier,
    /// code_challenge, and code_challenge_method for use in the authorization code grant.
    ///
    /// For authorization, the code_challenge_method parameter in the request body
    /// is automatically set to 'S256'.
    ///
    /// Internally this method uses the Rust ring cyrpto library to generate a secure random
    /// 32-octet sequence that is base64 URL encoded (no padding) and known as the code verifier.
    /// This sequence is hashed using SHA256 and base64 URL encoded (no padding) resulting in a
    /// 43-octet URL safe string which is known as the code challenge.
    fn oneshot() -> IdentityResult<ProofKeyCodeExchange> {
        let code_verifier = ProofKeyCodeExchange::code_verifier();
        let code_challenge = ProofKeyCodeExchange::code_challenge(&code_verifier);
        ProofKeyCodeExchange::new(
            code_verifier,
            code_challenge,
            ProofKeyCodeExchange::code_challenge_method(),
        )
    }
}

impl GenPkce for ProofKeyCodeExchange {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ProofKeyCodeExchange {
    /// Used to verify the the
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

impl ProofKeyCodeExchange {
    pub fn new<T: AsRef<str>>(
        code_verifier: T,
        code_challenge: T,
        code_challenge_method: T,
    ) -> IdentityResult<ProofKeyCodeExchange> {
        let code_challenge = code_challenge.as_ref().to_owned();
        if code_challenge.len() != 43 {
            return Err(AF::msg_err("code_challenge", "Must be 43-octet sequence"));
        }
        Ok(ProofKeyCodeExchange {
            code_verifier: code_verifier.as_ref().to_owned(),
            code_challenge,
            code_challenge_method: code_challenge_method.as_ref().to_owned(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pkce_generate() {
        let pkce = ProofKeyCodeExchange::oneshot().unwrap();
        assert_eq!(pkce.code_challenge.len(), 43);
    }

    #[test]
    fn validate_pkce_challenge_and_verifier() {
        let pkce = ProofKeyCodeExchange::oneshot().unwrap();
        let mut context = ring::digest::Context::new(&ring::digest::SHA256);
        context.update(pkce.code_verifier.as_bytes());
        let verifier = URL_SAFE_NO_PAD.encode(context.finish().as_ref());
        assert_eq!(verifier, pkce.code_challenge);
    }
}
