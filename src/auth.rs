use base64ct::{Base64UrlUnpadded, Encoding};
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha256};

const EDGEDB_AUTH_BASE_URL: &str = "edgedb://localhost:10702/branch/dev/ext/auth/";
const SERVER_PORT: u32 = 10702;

#[derive(Debug)]
pub struct Pkce {
    verifier: String,
    challenge: String,
}

pub fn generate_pkce() -> Pkce {
    // 1. generate 32 random bytes and URL-encode it:
    let input: [u8; 32] = thread_rng().gen();
    let verifier = Base64UrlUnpadded::encode_string(&input);
    // 2. SHA256 hash the result, then URL-encode again:
    let hash = Sha256::new().chain_update(&verifier).finalize();
    let challenge = Base64UrlUnpadded::encode_string(&hash);

    Pkce {
        verifier,
        challenge,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_pkce() {
        let pkce = generate_pkce();

        println!(
            "{pkce:?}, verifier length: {:?}, challenge length: {:?}",
            pkce.verifier.len(),
            pkce.challenge.len()
        );

        assert!(pkce.verifier.len() == pkce.challenge.len() && pkce.verifier.len() == 43);
    }
}
