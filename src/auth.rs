pub enum AuthError {
    Hash(),
}

pub struct Pkce {
    verifier: String,
    challenge: String,
}

pub fn generate_pkce() -> Result<Pkce, AuthError> {
    todo!()
}
