use crate::accesstoken::AccessToken;
use crate::grants::{Grant, GrantRequest, GrantType};
use crate::idtoken::IdToken;
use crate::oautherror::OAuthError;
use crate::oauthtools::OAuthTooling;
use std::collections::btree_map::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Output;
use transform_request::prelude::*;
use url::form_urlencoded::Serializer;

pub type OAuthReq<T> = Result<T, OAuthError>;

/// OAuthCredential list fields representing common OAuth credentials needed
/// to perform authentication in various formats such as token flow and
/// client credentials flow.

#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum OAuthCredential {
    ClientId,
    ClientSecret,
    AuthorizeURL,
    AccessTokenURL,
    RefreshTokenURL,
    RedirectURI,
    AccessCode,
    AccessToken,
    RefreshToken,
    ResponseMode,
    State,
    SessionState,
    ResponseType,
    GrantType,
    Nonce,
    Prompt,
    IdToken,
    Resource,
    DomainHint,
    Scopes,
    LoginHint,
    ClientAssertion,
    ClientAssertionType,
    CodeVerifier,
    CodeChallenge,
    CodeChallengeMethod,
    PostLogoutRedirectURI,
    LogoutURL,
    AdminConsent,
}

impl OAuthCredential {
    pub fn alias(self) -> &'static str {
        match self {
            OAuthCredential::ClientId => "client_id",
            OAuthCredential::ClientSecret => "client_secret",
            OAuthCredential::AuthorizeURL => "authorization_url",
            OAuthCredential::AccessTokenURL => "access_token_url",
            OAuthCredential::RefreshTokenURL => "refresh_token_url",
            OAuthCredential::RedirectURI => "redirect_uri",
            OAuthCredential::AccessCode => "code",
            OAuthCredential::AccessToken => "access_token",
            OAuthCredential::RefreshToken => "refresh_token",
            OAuthCredential::ResponseMode => "response_mode",
            OAuthCredential::ResponseType => "response_type",
            OAuthCredential::State => "state",
            OAuthCredential::SessionState => "session_state",
            OAuthCredential::GrantType => "grant_type",
            OAuthCredential::Nonce => "nonce",
            OAuthCredential::Prompt => "prompt",
            OAuthCredential::IdToken => "id_token",
            OAuthCredential::Resource => "resource",
            OAuthCredential::DomainHint => "domain_hint",
            OAuthCredential::Scopes => "scope",
            OAuthCredential::LoginHint => "login_hint",
            OAuthCredential::ClientAssertion => "client_assertion",
            OAuthCredential::ClientAssertionType => "client_assertion_type",
            OAuthCredential::CodeVerifier => "code_verifier",
            OAuthCredential::CodeChallenge => "code_challenge",
            OAuthCredential::CodeChallengeMethod => "code_challenge_method",
            OAuthCredential::LogoutURL => "logout_url",
            OAuthCredential::PostLogoutRedirectURI => "post_logout_redirect_uri",
            OAuthCredential::AdminConsent => "admin_consent",
        }
    }

    pub fn hash_alias(self) -> u64 {
        let to_hash = move |value: &str| {
            let mut s = DefaultHasher::new();
            value.hash(&mut s);
            s.finish()
        };

        to_hash(self.alias())
    }
}

impl ToString for OAuthCredential {
    fn to_string(&self) -> String {
        self.alias().to_string()
    }
}

/// # OAuth
/// An authorization and access token API implementing the OAuth 2.0 authorization
/// framework. This version is specifically meant for the OneDrive API V1.0
/// and the Graph beta API.
///
/// Requires knowing the OAuth grant that is being used
/// to request authorization and access tokens. This is to ensure that
/// the credentials used in requests include only information that is
/// required or optional for that specific grant and not any other. Even
/// if you accidently pass a value, such as a nonce, for a grant type
/// that does not use it, any request that is made will not include the
/// nonce regardless.
///
/// # Disclaimer
/// Using this API for other resource owners besides Microsoft may work but
/// functionality will more then likely be limited.
///
/// # Example
/// ```
/// use graph_oauth::oauth::{OAuth, GrantType};
/// let oauth = OAuth::new(GrantType::AuthorizationCode);
///
/// // or
/// let oauth_token_flow = OAuth::token_flow();
/// let oauth_code_flow = OAuth::code_flow();
/// let oauth_auth_grant = OAuth::authorization_code_grant();
/// let oauth_client_cred = OAuth::client_credentials_grant();
/// let oauth_implicit = OAuth::implicit_grant();
/// let oauth_open_id = OAuth::open_id_connect();
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, ToFile)]
pub struct OAuth {
    access_token: Option<AccessToken>,
    scopes: Vec<String>,
    credentials: BTreeMap<String, String>,
    grant: GrantType,
}

impl OAuth {
    /// Create a new OAuth instance.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::{OAuth, GrantType};
    ///
    /// let mut oauth = OAuth::new(GrantType::AuthorizationCode);
    /// ```
    pub fn new(grant: GrantType) -> OAuth {
        OAuth {
            access_token: None,
            scopes: Vec::new(),
            credentials: BTreeMap::new(),
            grant,
        }
    }

    /// Create a new OAuth instance for token flow.
    ///
    /// # See
    /// [Microsoft Token Flow Authorizaiton](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online)
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// let mut oauth = OAuth::token_flow();
    /// ```
    pub fn token_flow() -> OAuth {
        OAuth::new(GrantType::TokenFlow)
    }

    /// Create a new OAuth instance for code flow.
    ///
    /// # See
    /// [Microsoft Code Flow Authorizaiton](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online)
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// let mut oauth = OAuth::code_flow();
    /// ```
    pub fn code_flow() -> OAuth {
        OAuth::new(GrantType::CodeFlow)
    }

    /// Create a new OAuth instance for authorization code grant.
    ///
    /// # See
    /// [Authorization Code Grant for OAuth 2.0](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// let mut oauth = OAuth::authorization_code_grant();
    /// ```
    pub fn authorization_code_grant() -> OAuth {
        OAuth::new(GrantType::AuthorizationCode)
    }

    /// Create a new OAuth instance for the client credentials grant.
    ///
    /// # See
    /// [Client Credentials Grant for OAuth 2.0](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// let mut oauth = OAuth::client_credentials_grant();
    /// ```
    pub fn client_credentials_grant() -> OAuth {
        OAuth::new(GrantType::ClientCredentials)
    }

    /// Create a new OAuth instance for the implicit grant.
    ///
    /// # See
    /// [Implicit Grant for OAuth 2.0](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-implicit-grant-flow)
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// let mut oauth = OAuth::implicit_grant();
    /// ```
    pub fn implicit_grant() -> OAuth {
        OAuth::new(GrantType::Implicit)
    }

    /// Create a new OAuth instance for the open id connect grant.
    ///
    /// # See
    /// [Microsoft Open ID Connect](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc)
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// let mut oauth = OAuth::open_id_connect();
    /// ```
    pub fn open_id_connect() -> OAuth {
        OAuth::new(GrantType::OpenId)
    }

    /// Insert oauth credentials using the OAuthCredential enum.
    /// This method is used internally for each of the setter methods.
    /// Callers can optionally use this method to set credentials instead
    /// of the individual setter methods.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # use graph_oauth::oauth::OAuthCredential;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.insert(OAuthCredential::AuthorizeURL, "https://example.com".into());
    /// assert_eq!(oauth.contains(OAuthCredential::AuthorizeURL), true);
    /// println!("{:#?}", oauth.get(OAuthCredential::AuthorizeURL));
    /// ```
    pub fn insert(&mut self, oac: OAuthCredential, value: String) -> &mut OAuth {
        self.credentials
            .insert(oac.alias().to_string(), value.trim().to_string());
        self
    }

    pub fn entry(&mut self, oac: OAuthCredential, value: &str) -> &mut String {
        self.credentials
            .entry(oac.alias().to_string())
            .or_insert_with(|| value.to_string())
    }

    /// Get a previously set credential.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # use graph_oauth::oauth::OAuthCredential;
    /// # let mut oauth = OAuth::code_flow();
    /// let a = oauth.get(OAuthCredential::AuthorizeURL);
    /// ```
    pub fn get(&self, oac: OAuthCredential) -> Option<String> {
        self.credentials.get(oac.alias()).cloned()
    }

    /// Check if an OAuth credential has already been set.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # use graph_oauth::oauth::OAuthCredential;
    /// # let mut oauth = OAuth::code_flow();
    /// println!("{:#?}", oauth.contains(OAuthCredential::Nonce));
    /// ```
    pub fn contains(&self, t: OAuthCredential) -> bool {
        if t == OAuthCredential::Scopes {
            return !self.scopes.is_empty();
        }
        self.credentials.contains_key(t.alias())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.credentials.contains_key(key)
    }

    /// Remove a field from OAuth.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # use graph_oauth::oauth::OAuthCredential;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.client_id("client_id");
    ///
    /// assert_eq!(oauth.contains(OAuthCredential::ClientId), true);
    /// oauth.remove(OAuthCredential::ClientId);
    ///
    /// assert_eq!(oauth.contains(OAuthCredential::ClientId), false);
    /// ```
    pub fn remove(&mut self, oac: OAuthCredential) -> &mut OAuth {
        self.credentials.remove(oac.alias());
        self
    }

    /// Set the client id for an OAuth request.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # use graph_oauth::oauth::OAuthCredential;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.client_id("client_id");
    /// ```
    pub fn client_id(&mut self, client_id: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ClientId, client_id.into())
    }

    /// Set the state for an OAuth request.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # use graph_oauth::oauth::OAuthCredential;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.state("1234");
    /// ```
    pub fn state(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::State, value.into())
    }

    /// Set the client secret for an OAuth request.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.client_secret("client_secret");
    /// ```
    pub fn client_secret(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ClientSecret, value.into())
    }

    /// Set the authorization URL.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.authorize_url("https://example.com/authorize");
    /// ```
    pub fn authorize_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AuthorizeURL, value.into())
    }

    /// Set the access token url of a request for OAuth
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.access_token_url("https://example.com/token");
    /// ```
    pub fn access_token_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AccessTokenURL, value.into())
    }

    /// Set the refresh token url of a request for OAuth
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.refresh_token_url("https://example.com/token");
    /// ```
    pub fn refresh_token_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::RefreshTokenURL, value.into())
    }

    /// Set the redirect url of a request
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.redirect_uri("https://localhost:8888/redirect");
    /// ```
    pub fn redirect_uri(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::RedirectURI, value.into())
    }

    /// Set the access code.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.access_code("LDSF[POK43");
    /// ```
    pub fn access_code(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AccessCode, value.into())
    }

    /// Set the response mode.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.response_mode("query");
    /// ```
    pub fn response_mode(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ResponseMode, value.into())
    }

    /// Set the response type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.response_type("token");
    /// ```
    pub fn response_type(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ResponseType, value.into())
    }

    /// Set the nonce.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    ///
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.nonce("1234");
    /// ```
    pub fn nonce(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::Nonce, value.into())
    }

    /// Set the prompt for open id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    ///
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.prompt("login");
    /// ```
    pub fn prompt(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::Prompt, value.into())
    }

    /// Set id token for open id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::{OAuth, IdToken};
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.id_token(IdToken::new("1345", "code", "state", "session_state"));
    /// ```
    pub fn id_token(&mut self, value: IdToken) -> &mut OAuth {
        self.insert(OAuthCredential::IdToken, value.get_id_token());
        if let Some(code) = value.get_code() {
            self.access_code(code.as_str());
        }
        if let Some(state) = value.get_state() {
            let _ = self.entry(OAuthCredential::State, state.as_str());
        }
        if let Some(session_state) = value.get_session_state() {
            self.session_state(session_state.as_str());
        }
        self.insert(OAuthCredential::IdToken, value.get_id_token())
    }

    /// Set the session state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.session_state("session-state");
    /// ```
    pub fn session_state(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::SessionState, value.into())
    }

    /// Set the grant_type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.grant_type("token");
    /// ```
    pub fn grant_type(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::GrantType, value.into())
    }

    /// Set the resource.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.resource("resource");
    /// ```
    pub fn resource(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::Resource, value.into())
    }

    /// Set the grant_type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.code_verifier("code_verifier");
    /// ```
    pub fn code_verifier(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::CodeVerifier, value.into())
    }

    /// Set the grant_type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.domain_hint("domain_hint");
    /// ```
    pub fn domain_hint(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::DomainHint, value.into())
    }

    /// Set the code challenge.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.code_challenge("code_challenge");
    /// ```
    pub fn code_challenge(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::CodeChallenge, value.into())
    }

    /// Set the code challenge method.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.code_challenge_method("code_challenge_method");
    /// ```
    pub fn code_challenge_method(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::CodeChallengeMethod, value.into())
    }

    /// Set the login hint.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.login_hint("login_hint");
    /// ```
    pub fn login_hint(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::LoginHint, value.into())
    }

    /// Set the client assertion.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.client_assertion("client_assertion");
    /// ```
    pub fn client_assertion(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ClientAssertion, value.into())
    }

    /// Set the client assertion type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.client_assertion_type("client_assertion_type");
    /// ```
    pub fn client_assertion_type(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ClientAssertionType, value.into())
    }

    /// Set the url to send a post request that will log out the user.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.logout_url("https://example.com/logout?");
    /// ```
    pub fn logout_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::LogoutURL, value.into())
    }

    /// Set the redirect uri that user will be redirected to after logging out.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    /// oauth.post_logout_redirect_uri("http://localhost:8080");
    /// ```
    pub fn post_logout_redirect_uri(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::PostLogoutRedirectURI, value.into())
    }

    /// Set the admin consent. Only works for specific grants such as
    /// the client credentials grant.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::client_credentials_grant();
    /// oauth.admin_consent(true);
    /// ```
    pub fn admin_consent(&mut self, value: bool) -> &mut OAuth {
        if value {
            return self.insert(OAuthCredential::AdminConsent, "True".into());
        }
        self.insert(OAuthCredential::AdminConsent, "False".into())
    }

    /// Add a scope' for the OAuth URL.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    ///
    /// oauth.add_scope("Read")
    ///     .add_scope("Write")
    ///     .add_scope("ReadWrite.All");
    /// ```
    pub fn add_scope(&mut self, scope: &str) -> &mut OAuth {
        self.scopes.push(String::from(scope));
        self
    }

    /// Get the scopes.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    ///
    /// // the scopes take a separator just like Vec join.
    ///  let s = oauth.get_scopes(" ");
    /// println!("{:#?}", s);
    /// ```
    pub fn get_scopes(&self, sep: &str) -> String {
        self.scopes.join(sep).to_owned()
    }

    /// Remove a previously added scope.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # let mut oauth = OAuth::code_flow();
    ///
    /// // the scopes take a separator just like Vec join.
    ///  oauth.add_scope("scope");
    ///  oauth.remove_scope("scope");
    /// ```
    pub fn remove_scope(&mut self, scope: &str) {
        self.scopes.retain(|x| x != scope);
    }

    /// Set the access token.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::AccessToken;
    /// let mut oauth = OAuth::code_flow();
    /// let access_token = AccessToken::default();
    /// oauth.access_token(access_token);
    /// ```
    pub fn access_token(&mut self, ac: AccessToken) {
        self.access_token.replace(ac);
    }

    /// Get the access token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # use graph_oauth::oauth::AccessToken;
    /// # let access_token = AccessToken::default();
    /// # let mut oauth = OAuth::code_flow();
    /// # oauth.access_token(access_token);
    /// let access_token = oauth.get_access_token().unwrap();
    /// println!("{:#?}", access_token);
    /// ```
    pub fn get_access_token(&self) -> Option<AccessToken> {
        self.access_token.clone()
    }

    /// Get the refrsh token. This method returns the current refresh
    /// token stored in OAuth and does not make a request for a refresh
    /// token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::OAuth;
    /// # use graph_oauth::oauth::AccessToken;
    /// # let mut oauth = OAuth::code_flow();
    /// let mut  access_token = AccessToken::default();
    /// access_token.refresh_token(Some("refresh_token"));
    /// oauth.access_token(access_token);
    ///
    /// let refresh_token = oauth.get_refresh_token().unwrap();
    /// println!("{:#?}", refresh_token);
    /// ```
    pub fn get_refresh_token(&self) -> OAuthReq<String> {
        match self.get_access_token() {
            Some(token) => match token.get_refresh_token() {
                Some(t) => Ok(t),
                None => OAuthError::error_from::<String>(OAuthCredential::RefreshToken),
            },
            None => OAuthError::error_from::<String>(OAuthCredential::AccessToken),
        }
    }

    /// Sign the user out using the OneDrive v1.0 endpoint.
    ///
    /// # Example
    /// ```rust,ignore
    /// use graph_oauth::oauth::OAuth;
    /// let mut oauth = OAuth::code_flow();
    ///
    /// oauth.v1_logout().unwrap();
    /// ```
    pub fn v1_logout(&mut self) -> OAuthReq<Output> {
        let mut url = self.get_or_else(OAuthCredential::LogoutURL)?;
        if !url.ends_with('?') {
            url.push('?');
        }

        let mut vec = vec![
            url,
            "&client_id=".to_string(),
            self.get_or_else(OAuthCredential::ClientId)?,
            "&redirect_uri=".to_string(),
        ];

        if let Some(redirect) = self.get(OAuthCredential::PostLogoutRedirectURI) {
            vec.push(redirect)
        } else if let Some(redirect) = self.get(OAuthCredential::RedirectURI) {
            vec.push(redirect);
        }
        OAuthTooling::open_in_browser(vec.join("").as_str())
    }

    /// Sign the user out using the OneDrive v2.0 endpoint.
    ///
    /// # Example
    /// ```rust,ignore
    /// use graph_oauth::oauth::OAuth;
    /// let mut oauth = OAuth::client_credentials();
    ///
    /// oauth.v2_logout().unwrap();
    /// ```
    pub fn v2_logout(&self) -> OAuthReq<Output> {
        let mut url = self.get_or_else(OAuthCredential::LogoutURL)?;
        if let Some(redirect) = self.get(OAuthCredential::PostLogoutRedirectURI) {
            url.push_str(redirect.as_str());
        } else {
            let redirect_uri = self.get_or_else(OAuthCredential::RedirectURI)?;
            url.push_str(redirect_uri.as_str());
        }
        OAuthTooling::open_in_browser(url.as_str())
    }
}

impl OAuth {
    fn get_or_else(&self, c: OAuthCredential) -> Result<String, OAuthError> {
        self.get(c).ok_or_else(|| OAuthError::credential_error(c))
    }

    pub fn form_encode_credentials(
        &mut self,
        pairs: Vec<OAuthCredential>,
        encoder: &mut Serializer<String>,
    ) {
        pairs
            .iter()
            .filter(|oac| self.contains_key(oac.alias()) || oac.alias().eq("scope"))
            .for_each(|oac| {
                if oac.alias().eq("scope") && !self.scopes.is_empty() {
                    encoder.append_pair("scope", self.scopes.join(" ").as_str());
                } else if let Some(val) = self.get(oac.clone()) {
                    encoder.append_pair(oac.alias(), val.as_str());
                }
            });
    }

    pub fn encode_uri(&mut self, request_type: GrantRequest) -> OAuthReq<String> {
        let mut encoder = Serializer::new(String::new());
        match self.grant {
            GrantType::TokenFlow => match request_type {
                GrantRequest::Authorization => {
                    let _ = self.entry(OAuthCredential::ResponseType, "token");
                    self.form_encode_credentials(
                        GrantType::TokenFlow.available_credentials(GrantRequest::Authorization),
                        &mut encoder,
                    );
                    let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
                    if !url.ends_with('?') {
                        url.push('?');
                    }
                    url.push_str(encoder.finish().as_str());
                    Ok(url)
                },
                GrantRequest::AccessToken | GrantRequest::RefreshToken => {
                    OAuthError::grant_error(
                        self.grant,
                        GrantRequest::AccessToken,
                        "Grant type does not use request type. Please use OAuth::request_authorization() for browser requests"
                    )
                },
            },
            GrantType::CodeFlow => match request_type {
                GrantRequest::Authorization => {
                    let _ = self.entry(OAuthCredential::ResponseType, "code");
                    let _ = self.entry(OAuthCredential::ResponseMode, "query");
                    self.form_encode_credentials(
                        GrantType::CodeFlow.available_credentials(GrantRequest::Authorization),
                        &mut encoder,
                    );

                    let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
                    if !url.ends_with('?') {
                        url.push('?');
                    }
                    url.push_str(encoder.finish().as_str());
                    Ok(url)
                },
                GrantRequest::AccessToken => {
                    let _ = self.entry(OAuthCredential::ResponseType, "token");
                    let _ = self.entry(OAuthCredential::GrantType, "authorization_code");
                    self.form_encode_credentials(
                        GrantType::CodeFlow.available_credentials(GrantRequest::AccessToken),
                        &mut encoder,
                    );
                    Ok(encoder.finish())
                },
                GrantRequest::RefreshToken => {
                    let _ = self.entry(OAuthCredential::GrantType, "refresh_token");
                    let refresh_token = self.get_refresh_token()?;
                    encoder.append_pair("refresh_token", &refresh_token);
                    self.form_encode_credentials(
                        GrantType::CodeFlow.available_credentials(GrantRequest::RefreshToken),
                        &mut encoder,
                    );
                    Ok(encoder.finish())
                },
            },
            GrantType::AuthorizationCode => match request_type {
                GrantRequest::Authorization => {
                    let _ = self.entry(OAuthCredential::ResponseType, "code");
                    let _ = self.entry(OAuthCredential::ResponseMode, "query");
                    self.form_encode_credentials(
                        GrantType::AuthorizationCode.available_credentials(GrantRequest::Authorization),
                        &mut encoder,
                    );
                    let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
                    if !url.ends_with('?') {
                        url.push('?');
                    }
                    url.push_str(encoder.finish().as_str());
                    Ok(url)
                },
                GrantRequest::AccessToken | GrantRequest::RefreshToken => {
                    if request_type == GrantRequest::AccessToken {
                        let _ = self.entry(OAuthCredential::GrantType, "authorization_code");
                    } else {
                        let _ = self.entry(OAuthCredential::GrantType, "refresh_token");
                        encoder.append_pair("refresh_token", &self.get_refresh_token()?);
                    }
                    self.form_encode_credentials(
                        GrantType::AuthorizationCode.available_credentials(request_type),
                        &mut encoder,
                    );
                    Ok(encoder.finish())
                },
            },
            GrantType::ClientCredentials => match request_type {
                GrantRequest::Authorization => {
                    self.form_encode_credentials(
                        GrantType::ClientCredentials.available_credentials(GrantRequest::Authorization),
                        &mut encoder,
                    );

                    let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
                    if !url.ends_with('?') {
                        url.push('?');
                    }
                    url.push_str(encoder.finish().as_str());
                    Ok(url)
                },
                GrantRequest::AccessToken | GrantRequest::RefreshToken => {
                    // The following assumes that if the client secret is present, then the
                    // caller is performing the access token request with a shared
                    // secret. If the client secret is not preset, then the access
                    // token request with a certificate is used.
                    let _ = self.entry(OAuthCredential::GrantType, "client_credentials");
                    if self.contains(OAuthCredential::ClientSecret) {
                        let mut vec = GrantType::ClientCredentials.available_credentials(GrantRequest::AccessToken);
                        vec.remove_item(&OAuthCredential::ClientAssertion);
                        vec.remove_item(&OAuthCredential::ClientAssertionType);
                        self.form_encode_credentials(
                            vec,
                            &mut encoder,
                        );
                    } else {
                        let _ = self.entry(
                            OAuthCredential::ClientAssertionType,
                            "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
                        );
                        let mut vec = GrantType::ClientCredentials.available_credentials(GrantRequest::AccessToken);
                        vec.remove_item(&OAuthCredential::ClientSecret);
                        self.form_encode_credentials(
                            vec,
                            &mut encoder,
                        );
                    }

                    Ok(encoder.finish())
                },
            },
            GrantType::Implicit => match request_type {
                GrantRequest::Authorization => {
                    let _ = self.entry(OAuthCredential::ResponseType, "token");
                    self.form_encode_credentials(
                        GrantType::Implicit.available_credentials(GrantRequest::Authorization),
                        &mut encoder,
                    );
                    let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
                    if !url.ends_with('?') {
                        url.push('?');
                    }
                    url.push_str(encoder.finish().as_str());
                    Ok(url)
                },
                GrantRequest::AccessToken | GrantRequest::RefreshToken => {
                    OAuthError::grant_error(
                        self.grant,
                        GrantRequest::AccessToken,
                        "Grant type does not use request type. Please use OAuth::request_authorization() for browser requests"
                    )
                },
            },
            GrantType::OpenId => match request_type {
                GrantRequest::Authorization => {
                    if !self.scopes.contains(&"openid".into()) {
                        self.add_scope("openid");
                    }
                    if !self.contains(OAuthCredential::Nonce) {
                        return OAuthError::grant_error(
                            GrantType::OpenId,
                            GrantRequest::Authorization,
                            "Requires nonce parameter.",
                        );
                    }
                    let _ = self.entry(OAuthCredential::ResponseMode, "form_post");
                    let _ = self.entry(OAuthCredential::ResponseType, "id_token");
                    self.form_encode_credentials(
                        GrantType::OpenId.available_credentials(GrantRequest::Authorization),
                        &mut encoder,
                    );

                    let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
                    if !url.ends_with('?') {
                        url.push('?');
                    }
                    url.push_str(encoder.finish().as_str());
                    Ok(url)
                },
                GrantRequest::AccessToken => {
                    let _ = self.entry(OAuthCredential::GrantType, "authorization_code");
                    self.form_encode_credentials(
                        GrantType::OpenId.available_credentials(GrantRequest::AccessToken),
                        &mut encoder,
                    );
                    Ok(encoder.finish())
                },
                GrantRequest::RefreshToken => {
                    let _ = self.entry(OAuthCredential::GrantType, "refresh_token");
                    let refresh_token = self.get_refresh_token()?;
                    encoder.append_pair("refresh_token", &refresh_token);
                    self.form_encode_credentials(
                        GrantType::OpenId.available_credentials(GrantRequest::RefreshToken),
                        &mut encoder,
                    );
                    Ok(encoder.finish())
                },
            },
        }
    }
}

impl Grant for OAuth {
    fn request_authorization(&mut self) -> OAuthReq<Output> {
        let url = self.encode_uri(GrantRequest::Authorization)?;
        OAuthTooling::open_in_browser(url.as_str())
    }

    fn request_access_token(&mut self) -> OAuthReq<()> {
        let url = self.get_or_else(OAuthCredential::AccessTokenURL)?;
        let body = self.encode_uri(GrantRequest::AccessToken)?;
        let access_token = OAuthTooling::post_access_token(url.as_str(), body.as_str())?;
        self.access_token(access_token);
        Ok(())
    }

    fn request_refresh_token(&mut self) -> OAuthReq<()> {
        let url = self.get_or_else(OAuthCredential::RefreshTokenURL)?;
        let body = self.encode_uri(GrantRequest::RefreshToken)?;
        let access_token = OAuthTooling::post_access_token(url.as_str(), body.as_str())?;
        self.access_token(access_token);
        Ok(())
    }
}

impl From<GrantType> for OAuth {
    fn from(grant_name: GrantType) -> Self {
        OAuth::new(grant_name)
    }
}
