use crate::auth::OAuthParameter;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ParameterIs {
    Required(OAuthParameter),
    Optional(OAuthParameter),
}
