use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ring::rand::SecureRandom;

pub struct Crypto;

impl Crypto {
    pub fn secure_random_string() -> anyhow::Result<String> {
        let mut buf = [0; 32];

        let rng = ring::rand::SystemRandom::new();
        rng.fill(&mut buf)
            .map_err(|_| anyhow::Error::msg("ring::error::Unspecified"))?;

        let base_64_random_string = URL_SAFE_NO_PAD.encode(buf);

        let mut context = ring::digest::Context::new(&ring::digest::SHA256);
        context.update(base_64_random_string.as_bytes());

        let secure_random_string = URL_SAFE_NO_PAD.encode(context.finish().as_ref());
        Ok(secure_random_string)
    }
}
