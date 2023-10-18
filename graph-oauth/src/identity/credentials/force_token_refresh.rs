#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForceTokenRefresh {
    /// Always use the token cache first to when returning tokens.
    /// Expired tokens will still cause an authorization request to
    /// be called.
    #[default]
    Never,
    /// ForceRefreshToken::Once will cause only the next authorization request
    /// to ignore any tokens in cache and request a new token. Authorization
    /// requests after this are treated as ForceRefreshToken::Never
    Once,
    /// Always make an authorization request regardless of any tokens in cache.
    Always,
}
