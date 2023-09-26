#[derive(Clone, Default)]
pub struct TokenValidator {
    application_id: Option<String>,
}

impl TokenValidator {
    pub fn builder() -> TokenValidator {
        TokenValidator::default()
    }

    pub fn with_application_id(&mut self, aud: impl AsRef<str>) -> &mut Self {
        self.application_id = Some(aud.as_ref().to_owned());
        self
    }
}
