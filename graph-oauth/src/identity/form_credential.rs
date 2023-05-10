use crate::auth::OAuthParameter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SerializerField {
    Required(OAuthParameter),
    NotRequired(OAuthParameter),
}
