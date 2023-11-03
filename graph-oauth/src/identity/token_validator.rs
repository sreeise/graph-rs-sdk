#[derive(Clone, Default)]
pub struct TokenValidator {
    application_id: Option<String>,
}

impl TokenValidator {
    pub fn builder() -> TokenValidator {
        TokenValidator::default()
    }

    // Validate the audience
    pub fn with_aud(&mut self, aud: impl AsRef<str>) -> &mut Self {
        self.application_id = Some(aud.as_ref().to_owned());
        self
    }
}
