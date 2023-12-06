mod jwk;
mod pkce;

pub use jwk::*;
pub use pkce::*;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ring::rand::SecureRandom;

pub fn secure_random_32() -> String {
    let mut buf = [0; 32];

    let rng = ring::rand::SystemRandom::new();
    rng.fill(&mut buf).expect("ring::error::Unspecified");

    URL_SAFE_NO_PAD.encode(buf)
}
