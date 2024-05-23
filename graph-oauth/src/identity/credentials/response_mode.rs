/// Specifies how the identity platform should return the requested token to your app.
///
/// Supported values:
///
/// - **query**: Default when requesting an access token. Provides the code as a query string
///     parameter on your redirect URI. The query parameter is not supported when requesting an
///     ID token by using the implicit flow.
/// - fragment: Default when requesting an ID token by using the implicit flow.
///     Also supported if requesting only a code.
/// - form_post: Executes a POST containing the code to your redirect URI.
///     Supported when requesting a code.
#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ResponseMode {
    /// Default when requesting an access token. Provides the code as a query string
    /// parameter on your redirect URI. The query parameter is not supported when requesting an
    /// ID token by using the implicit flow.
    #[default]
    Query,
    /// Default when requesting an ID token by using the implicit flow. Also supported if requesting only a code.
    Fragment,
    /// Executes a POST containing the code to your redirect URI. Supported when requesting a code.
    FormPost,
}

impl AsRef<str> for ResponseMode {
    fn as_ref(&self) -> &'static str {
        match self {
            ResponseMode::Query => "query",
            ResponseMode::Fragment => "fragment",
            ResponseMode::FormPost => "form_post",
        }
    }
}
