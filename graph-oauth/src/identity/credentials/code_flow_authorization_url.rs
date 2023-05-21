use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::oauth::ResponseType;
use graph_error::{AuthorizationFailure, AuthorizationResult};
use url::form_urlencoded::Serializer;
use url::Url;

/// Legacy sign in for personal microsoft accounts to get access tokens for OneDrive
/// Not recommended - Instead use Microsoft Identity Platform OAuth 2.0 and OpenId Connect.
/// https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online#code-flow
#[derive(Clone)]
pub struct CodeFlowAuthorizationUrl {
    /// Required.
    /// The Application (client) ID that the Azure portal - App registrations page assigned
    /// to your app
    pub(crate) client_id: String,
    /// Required
    /// The same redirect_uri value that was used to acquire the authorization_code.
    pub(crate) redirect_uri: String,
    /// Required
    /// Must be code for code flow.
    pub(crate) response_type: ResponseType,
    /// Required
    /// A space-separated list of scopes. The scopes must all be from a single resource,
    /// along with OIDC scopes (profile, openid, email). For more information, see Permissions
    /// and consent in the Microsoft identity platform. This parameter is a Microsoft extension
    /// to the authorization code flow, intended to allow apps to declare the resource they want
    /// the token for during token redemption.
    pub(crate) scope: Vec<String>,
}

impl CodeFlowAuthorizationUrl {
    pub fn new<T: AsRef<str>, U: ToString, I: IntoIterator<Item = U>>(
        client_id: T,
        redirect_uri: T,
        scope: I,
    ) -> CodeFlowAuthorizationUrl {
        CodeFlowAuthorizationUrl {
            client_id: client_id.as_ref().to_owned(),
            redirect_uri: redirect_uri.as_ref().to_owned(),
            response_type: ResponseType::Code,
            scope: scope.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn builder() -> CodeFlowAuthorizationUrlBuilder {
        CodeFlowAuthorizationUrlBuilder::new()
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
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

        let mut encoder = Serializer::new(String::new());

        serializer.encode_query(
            vec![],
            vec![
                OAuthParameter::ClientId,
                OAuthParameter::RedirectUri,
                OAuthParameter::Scope,
                OAuthParameter::ResponseType,
            ],
            &mut encoder,
        )?;

        if let Some(authorization_url) = serializer.get(OAuthParameter::AuthorizationUrl) {
            let mut url = Url::parse(authorization_url.as_str())?;
            url.set_query(Some(encoder.finish().as_str()));
            Ok(url)
        } else {
            AuthorizationFailure::msg_result("authorization_url", "Internal Error")
        }
    }
}

#[derive(Clone)]
pub struct CodeFlowAuthorizationUrlBuilder {
    token_flow_authorization_url: CodeFlowAuthorizationUrl,
}

impl CodeFlowAuthorizationUrlBuilder {
    fn new() -> CodeFlowAuthorizationUrlBuilder {
        CodeFlowAuthorizationUrlBuilder {
            token_flow_authorization_url: CodeFlowAuthorizationUrl {
                client_id: String::new(),
                redirect_uri: String::new(),
                response_type: ResponseType::Code,
                scope: vec![],
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.token_flow_authorization_url.client_id = client_id.as_ref().to_owned();
        self
    }

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.token_flow_authorization_url.scope =
            scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_redirect_uri<T: AsRef<str>>(&mut self, redirect_uri: T) -> &mut Self {
        self.token_flow_authorization_url.redirect_uri = redirect_uri.as_ref().to_owned();
        self
    }
}
