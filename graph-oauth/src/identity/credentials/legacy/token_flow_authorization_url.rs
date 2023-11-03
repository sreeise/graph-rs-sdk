use url::Url;

use graph_error::{AuthorizationFailure, IdentityResult, AF};

use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::oauth::ResponseType;

/// Legacy sign in for personal microsoft accounts to get access tokens for OneDrive
/// Not recommended - Instead use Microsoft Identity Platform OAuth 2.0 and OpenId Connect.
/// https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online#token-flow
#[derive(Clone)]
pub struct TokenFlowAuthorizationUrl {
    pub(crate) client_id: String,
    pub(crate) redirect_uri: String,
    pub(crate) response_type: ResponseType,
    pub(crate) scope: Vec<String>,
}

impl TokenFlowAuthorizationUrl {
    pub fn new<T: AsRef<str>, U: ToString, I: IntoIterator<Item = U>>(
        client_id: T,
        redirect_uri: T,
        scope: I,
    ) -> TokenFlowAuthorizationUrl {
        TokenFlowAuthorizationUrl {
            client_id: client_id.as_ref().to_owned(),
            redirect_uri: redirect_uri.as_ref().to_owned(),
            response_type: ResponseType::Token,
            scope: scope.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn builder() -> TokenFlowAuthorizationUrlBuilder {
        TokenFlowAuthorizationUrlBuilder::new()
    }

    pub fn url(&self) -> IdentityResult<Url> {
        let mut serializer = OAuthSerializer::new();
        if self.redirect_uri.trim().is_empty() {
            return AuthorizationFailure::result("redirect_uri");
        }

        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::result("client_id");
        }

        if self.scope.is_empty() {
            return AuthorizationFailure::result("scope");
        }

        serializer
            .client_id(self.client_id.as_str())
            .redirect_uri(self.redirect_uri.as_str())
            .extend_scopes(self.scope.clone())
            .legacy_authority()
            .response_type(self.response_type.clone());

        let query = serializer.encode_query(
            vec![],
            vec![
                OAuthParameter::ClientId,
                OAuthParameter::RedirectUri,
                OAuthParameter::Scope,
                OAuthParameter::ResponseType,
            ],
        )?;

        if let Some(authorization_url) = serializer.get(OAuthParameter::AuthorizationUrl) {
            let mut url = Url::parse(authorization_url.as_str())?;
            url.set_query(Some(query.as_str()));
            Ok(url)
        } else {
            AF::msg_internal_result("authorization_url")
        }
    }
}

#[derive(Clone)]
pub struct TokenFlowAuthorizationUrlBuilder {
    authorization_url: TokenFlowAuthorizationUrl,
}

impl TokenFlowAuthorizationUrlBuilder {
    fn new() -> TokenFlowAuthorizationUrlBuilder {
        TokenFlowAuthorizationUrlBuilder {
            authorization_url: TokenFlowAuthorizationUrl {
                client_id: String::new(),
                redirect_uri: String::new(),
                response_type: ResponseType::Token,
                scope: vec![],
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.authorization_url.client_id = client_id.as_ref().to_owned();
        self
    }

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.authorization_url.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_redirect_uri<T: AsRef<str>>(&mut self, redirect_uri: T) -> &mut Self {
        self.authorization_url.redirect_uri = redirect_uri.as_ref().to_owned();
        self
    }
}
