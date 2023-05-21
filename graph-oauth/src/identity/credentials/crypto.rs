use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ring::rand::SecureRandom;

pub struct Crypto;

impl Crypto {
    pub fn sha256_secure_string() -> anyhow::Result<(String, String)> {
        let mut buf = [0; 32];

        let rng = ring::rand::SystemRandom::new();
        rng.fill(&mut buf)
            .map_err(|_| anyhow::Error::msg("ring::error::Unspecified"))?;

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
