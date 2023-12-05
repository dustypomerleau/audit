/// A unique surgeon
pub struct Surgeon {
    pub email: String, // probably best to validate this as unique and email form at both the form and database levels - but pulling in the regex crate will probably make your wasm bundle huge
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub site: Option<String>,
}
