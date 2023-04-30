use crate::auth::{OAuth, OAuthCredential};
use crate::grants::GrantType;
use crate::identity::{Authority, AzureAuthorityHost, Prompt, ResponseMode};
use crate::oauth::form_credential::FormCredential;
use crate::oauth::{ProofKeyForCodeExchange, ResponseType};
use graph_error::{AuthorizationFailure, AuthorizationResult};
use url::form_urlencoded::Serializer;
use url::Url;

/// Get the authorization url required to perform the initial authorization and redirect in the
/// authorization code flow.
///
/// The authorization code flow begins with the client directing the user to the /authorize
/// endpoint.
///
/// The OAuth 2.0 authorization code grant type, or auth code flow, enables a client application
/// to obtain authorized access to protected resources like web APIs. The auth code flow requires
/// a user-agent that supports redirection from the authorization server (the Microsoft identity platform)
/// back to your application. For example, a web browser, desktop, or mobile application operated
/// by a user to sign in to your app and access their data.
///
/// Reference: https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow#request-an-authorization-code
#[derive(Clone, Debug)]
pub struct AuthCodeAuthorizationUrl {
    /// The client (application) ID of the service principal
    pub(crate) client_id: String,
    pub(crate) redirect_uri: String,
    pub(crate) authority: Authority,
    pub(crate) response_type: Vec<ResponseType>,
    pub(crate) response_mode: Option<ResponseMode>,
    pub(crate) nonce: Option<String>,
    pub(crate) state: Option<String>,
    pub(crate) scope: Vec<String>,
    pub(crate) prompt: Option<Prompt>,
    pub(crate) domain_hint: Option<String>,
    pub(crate) login_hint: Option<String>,
    pub(crate) code_challenge: Option<String>,
    pub(crate) code_challenge_method: Option<String>,
}

impl AuthCodeAuthorizationUrl {
    pub fn new<T: AsRef<str>>(client_id: T, redirect_uri: T) -> AuthCodeAuthorizationUrl {
        AuthCodeAuthorizationUrl {
            client_id: client_id.as_ref().to_owned(),
            redirect_uri: redirect_uri.as_ref().to_owned(),
            authority: Authority::default(),
            response_type: vec![ResponseType::Code],
            response_mode: None,
            nonce: None,
            state: None,
            scope: vec![],
            prompt: None,
            domain_hint: None,
            login_hint: None,
            code_challenge: None,
            code_challenge_method: None,
        }
    }

    pub fn grant_type(&self) -> GrantType {
        GrantType::AuthorizationCode
    }

    pub fn builder() -> AuthCodeAuthorizationUrlBuilder {
        AuthCodeAuthorizationUrlBuilder::new()
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
        self.url_with_host(&AzureAuthorityHost::default())
    }

    pub fn url_with_host(
        &self,
        azure_authority_host: &AzureAuthorityHost,
    ) -> AuthorizationResult<Url> {
        let mut serializer = OAuth::new();

        if self.redirect_uri.trim().is_empty() {
            return AuthorizationFailure::required_value_msg_result("redirect_uri", None);
        }

        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::required_value_msg_result("client_id", None);
        }

        if self.scope.is_empty() {
            return AuthorizationFailure::required_value_msg_result("scope", None);
        }

        serializer
            .client_id(self.client_id.as_str())
            .redirect_uri(self.redirect_uri.as_str())
            .extend_scopes(self.scope.clone())
            .authority(azure_authority_host, &self.authority);

        let response_types: Vec<String> =
            self.response_type.iter().map(|s| s.to_string()).collect();

        if response_types.is_empty() {
            serializer.response_type("code");
            if let Some(response_mode) = self.response_mode.as_ref() {
                serializer.response_mode(response_mode.as_ref());
            }
        } else {
            let response_type_string = response_types.join(" ");
            let mut response_type = response_type_string.trim();
            if response_type.is_empty() {
                serializer.response_type("code");
                response_type = "code";
            } else {
                serializer.response_type(response_type);
            }

            if response_type.contains("id_token") {
                serializer.response_mode(ResponseMode::Fragment.as_ref());
            } else if let Some(response_mode) = self.response_mode.as_ref() {
                serializer.response_mode(response_mode.as_ref());
            }
        }

        if let Some(response_mode) = self.response_mode.as_ref() {
            serializer.response_mode(response_mode.as_ref());
        }

        if let Some(state) = self.state.as_ref() {
            serializer.state(state.as_str());
        }

        if let Some(prompt) = self.prompt.as_ref() {
            serializer.prompt(prompt.as_ref());
        }

        if let Some(domain_hint) = self.domain_hint.as_ref() {
            serializer.domain_hint(domain_hint.as_str());
        }

        if let Some(login_hint) = self.login_hint.as_ref() {
            serializer.login_hint(login_hint.as_str());
        }

        if let Some(code_challenge) = self.code_challenge.as_ref() {
            serializer.code_challenge(code_challenge.as_str());
        }

        if let Some(code_challenge_method) = self.code_challenge_method.as_ref() {
            serializer.code_challenge_method(code_challenge_method.as_str());
        }

        let authorization_credentials = vec![
            FormCredential::Required(OAuthCredential::ClientId),
            FormCredential::Required(OAuthCredential::ResponseType),
            FormCredential::Required(OAuthCredential::RedirectUri),
            FormCredential::Required(OAuthCredential::Scope),
            FormCredential::NotRequired(OAuthCredential::ResponseMode),
            FormCredential::NotRequired(OAuthCredential::State),
            FormCredential::NotRequired(OAuthCredential::Prompt),
            FormCredential::NotRequired(OAuthCredential::LoginHint),
            FormCredential::NotRequired(OAuthCredential::DomainHint),
            FormCredential::NotRequired(OAuthCredential::CodeChallenge),
            FormCredential::NotRequired(OAuthCredential::CodeChallengeMethod),
        ];

        let mut encoder = Serializer::new(String::new());
        serializer.url_query_encode(authorization_credentials, &mut encoder)?;

        if let Some(authorization_url) = serializer.get(OAuthCredential::AuthorizationUrl) {
            let mut url = Url::parse(authorization_url.as_str())?;
            url.set_query(Some(encoder.finish().as_str()));
            Ok(url)
        } else {
            AuthorizationFailure::required_value_msg_result(
                "authorization_url",
                Some("Internal Error"),
            )
        }
    }
}

#[derive(Clone)]
pub struct AuthCodeAuthorizationUrlBuilder {
    authorization_code_authorize_url: AuthCodeAuthorizationUrl,
}

impl Default for AuthCodeAuthorizationUrlBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthCodeAuthorizationUrlBuilder {
    pub fn new() -> AuthCodeAuthorizationUrlBuilder {
        AuthCodeAuthorizationUrlBuilder {
            authorization_code_authorize_url: AuthCodeAuthorizationUrl {
                client_id: String::new(),
                redirect_uri: String::new(),
                authority: Authority::default(),
                response_mode: None,
                response_type: vec![ResponseType::Code],
                nonce: None,
                state: None,
                scope: vec![],
                prompt: None,
                domain_hint: None,
                login_hint: None,
                code_challenge: None,
                code_challenge_method: None,
            },
        }
    }

    pub fn with_redirect_uri<T: AsRef<str>>(&mut self, redirect_uri: T) -> &mut Self {
        self.authorization_code_authorize_url.redirect_uri = redirect_uri.as_ref().to_owned();
        self
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.authorization_code_authorize_url.client_id = client_id.as_ref().to_owned();
        self
    }

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.authorization_code_authorize_url.authority =
            Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.authorization_code_authorize_url.authority = authority.into();
        self
    }

    /// Default is code. Must include code for the authorization code flow.
    /// Can also include id_token or token if using the hybrid flow.
    pub fn with_response_type<I: IntoIterator<Item = ResponseType>>(
        &mut self,
        response_type: I,
    ) -> &mut Self {
        self.authorization_code_authorize_url.response_type = response_type.into_iter().collect();
        self
    }

    /// Specifies how the identity platform should return the requested token to your app.
    ///
    /// Supported values:
    ///
    /// - **query**: Default when requesting an access token. Provides the code as a query string
    ///     parameter on your redirect URI. The query parameter is not supported when requesting an
    ///     ID token by using the implicit flow.
    /// - **fragment**: Default when requesting an ID token by using the implicit flow.
    ///     Also supported if requesting only a code.
    /// - **form_post**: Executes a POST containing the code to your redirect URI.
    ///     Supported when requesting a code.
    pub fn with_response_mode(&mut self, response_mode: ResponseMode) -> &mut Self {
        self.authorization_code_authorize_url.response_mode = Some(response_mode);
        self
    }

    /// A value included in the request, generated by the app, that is included in the
    /// resulting id_token as a claim. The app can then verify this value to mitigate token
    /// replay attacks. The value is typically a randomized, unique string that can be used
    /// to identify the origin of the request.
    pub fn with_nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
        self.authorization_code_authorize_url.nonce = Some(nonce.as_ref().to_owned());
        self
    }

    pub fn with_state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
        self.authorization_code_authorize_url.state = Some(state.as_ref().to_owned());
        self
    }

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scopes: I) -> &mut Self {
        self.authorization_code_authorize_url.scope =
            scopes.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Automatically adds profile, id_token, and offline_access to the scope parameter.
    pub fn with_default_scope(&mut self) -> &mut Self {
        self.with_scope(vec!["profile", "id_token", "offline_access"])
    }

    /// Indicates the type of user interaction that is required. Valid values are login, none,
    /// consent, and select_account.
    ///
    /// - **prompt=login** forces the user to enter their credentials on that request, negating single-sign on.
    /// - **prompt=none** is the opposite. It ensures that the user isn't presented with any interactive prompt.
    ///     If the request can't be completed silently by using single-sign on, the Microsoft identity platform returns an interaction_required error.
    /// - **prompt=consent** triggers the OAuth consent dialog after the user signs in, asking the user to
    ///     grant permissions to the app.
    /// - **prompt=select_account** interrupts single sign-on providing account selection experience
    ///     listing all the accounts either in session or any remembered account or an option to choose to use a different account altogether.
    pub fn with_prompt(&mut self, prompt: Prompt) -> &mut Self {
        self.authorization_code_authorize_url.prompt = Some(prompt);
        self
    }

    pub fn with_domain_hint<T: AsRef<str>>(&mut self, domain_hint: T) -> &mut Self {
        self.authorization_code_authorize_url.domain_hint = Some(domain_hint.as_ref().to_owned());
        self
    }

    pub fn with_login_hint<T: AsRef<str>>(&mut self, login_hint: T) -> &mut Self {
        self.authorization_code_authorize_url.login_hint = Some(login_hint.as_ref().to_owned());
        self
    }

    /// Used to secure authorization code grants by using Proof Key for Code Exchange (PKCE).
    /// Required if code_challenge_method is included.
    pub fn with_code_challenge<T: AsRef<str>>(&mut self, code_challenge: T) -> &mut Self {
        self.authorization_code_authorize_url.code_challenge =
            Some(code_challenge.as_ref().to_owned());
        self
    }

    /// The method used to encode the code_verifier for the code_challenge parameter.
    /// This SHOULD be S256, but the spec allows the use of plain if the client can't support SHA256.
    ///
    /// If excluded, code_challenge is assumed to be plaintext if code_challenge is included.
    /// The Microsoft identity platform supports both plain and S256.
    pub fn with_code_challenge_method<T: AsRef<str>>(
        &mut self,
        code_challenge_method: T,
    ) -> &mut Self {
        self.authorization_code_authorize_url.code_challenge_method =
            Some(code_challenge_method.as_ref().to_owned());
        self
    }

    pub fn with_proof_key_for_code_exchange(
        &mut self,
        proof_key_for_code_exchange: &ProofKeyForCodeExchange,
    ) -> &mut Self {
        self.with_code_challenge(proof_key_for_code_exchange.code_challenge.as_str());
        self.with_code_challenge_method(proof_key_for_code_exchange.code_challenge_method.as_str());
        self
    }

    pub fn build(&self) -> AuthCodeAuthorizationUrl {
        self.authorization_code_authorize_url.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_uri() {
        let authorizer = AuthCodeAuthorizationUrl::builder()
            .with_redirect_uri("https::/localhost:8080")
            .with_client_id("client_id")
            .with_scope(["read", "write"])
            .build();

        let url_result = authorizer.url();
        assert!(url_result.is_ok());
    }
}
