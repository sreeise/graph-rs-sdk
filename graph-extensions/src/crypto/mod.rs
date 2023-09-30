mod pkce;

pub use pkce::*;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use graph_error::{IdentityResult, AF};
use ring::rand::SecureRandom;

pub fn secure_random_32() -> IdentityResult<String> {
    let mut buf = [0; 32];

    let rng = ring::rand::SystemRandom::new();
    rng.fill(&mut buf)
        .map_err(|_| AF::unknown("ring::error::Unspecified"))?;

    Ok(URL_SAFE_NO_PAD.encode(buf))
}
