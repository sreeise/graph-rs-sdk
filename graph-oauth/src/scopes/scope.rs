/// Trait for specifying that a type is used for scopes.
pub trait Scope: AsRef<str> + ToString {
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl Scope for &str {}
