use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};
use chrono::Utc;
use rand::{rngs::StdRng, Rng, SeedableRng};
use sha2::{Digest, Sha512};

const TOKEN_LENGTH: usize = 32;
const SHORT_TOKEN_LENGTH: usize = 4;

pub fn generate_token() -> String {
    let secret = std::env::var("SECRET").ok();
    let seed = {
        let mut seed = [0u8; TOKEN_LENGTH];

        if let Some(s) = secret {
            let secret = s.as_bytes();
            for (i, &byte) in secret.iter().enumerate() {
                seed[i % seed.len()] ^= byte;
            }
        }

        let mut entropy = [0u8; TOKEN_LENGTH];
        StdRng::from_entropy().fill(&mut entropy);
        for (i, &byte) in entropy.iter().enumerate() {
            seed[i % seed.len()] ^= byte;
        }

        let ts = Utc::now().timestamp().to_ne_bytes();
        for (i, &byte) in ts.iter().enumerate() {
            for offset in 0..(seed.len() / ts.len()) {
                seed[offset * ts.len() + (i % seed.len())] ^= byte;
            }
        }

        seed
    };
    let mut rng = StdRng::from_seed(seed);
    let mut bytes = [0u8; TOKEN_LENGTH];
    rng.fill(&mut bytes);

    BASE64_URL_SAFE_NO_PAD.encode(&bytes)
}

pub fn hash_token(token: &str) -> String {
    let hashed_token = Sha512::digest(token.as_bytes());
    BASE64_URL_SAFE_NO_PAD.encode(hashed_token)
}

/// This isn't really safe, it's only 4 bytes; should only be used
/// in low risk environments, such as the initial setup of the infradmin account.
pub fn generate_short_token() -> String {
    let mut bytes = [0u8; SHORT_TOKEN_LENGTH];
    StdRng::from_entropy().fill(&mut bytes);
    base32::encode(base32::Alphabet::Crockford, &bytes)
}
