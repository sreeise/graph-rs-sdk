use crate::auth::OAuthCredential;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FormCredential {
    Required(OAuthCredential),
    NotRequired(OAuthCredential),
}
