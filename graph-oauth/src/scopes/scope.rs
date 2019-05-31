pub trait Scope: AsRef<str> + ToString {
    fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl Scope for &str {}
impl Scope for String {}
impl Scope for &String {}
