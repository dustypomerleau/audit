/// A unique surgeon
pub struct Surgeon {
    email: String, // probably best to validate this as unique and email form at both the form and database levels - but pulling in the regex crate will probably make your wasm bundle huge
    first_name: String,
    last_name: String,
    site: Option<String>,
}
