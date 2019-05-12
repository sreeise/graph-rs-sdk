use crate::accesstoken::AccessToken;
use crate::grants::{ClientCredentialsGrant, GrantRequest, GrantType, ImplicitGrant, OpenId};
use crate::idtoken::IdToken;
use crate::oautherror::OAuthError;
use reqwest::{header, RequestBuilder};
use std::collections::btree_map::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::ErrorKind;
use std::process::Output;
use transform_request::prelude::*;
use url::form_urlencoded;
use url::form_urlencoded::Serializer;
use webbrowser;

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
    Scopes,
    PostLogoutRedirectURI,
    LogoutURL,
}

impl OAuthCredential {
    pub fn alias(self) -> &'static str {
        match self {
            OAuthCredential::ClientId => "client_id",
            OAuthCredential::ClientSecret => "client_secret",
            OAuthCredential::AuthorizeURL => "sign_in_url",
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
            OAuthCredential::Scopes => "scope",
            OAuthCredential::LogoutURL => "logout_url",
            OAuthCredential::PostLogoutRedirectURI => "post_logout_redirect_uri",
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
/// # Disclaimer
/// Using this API for other resource owners besides Microsoft may work but
/// functionality will more then likely be limited.
///
/// # Example
/// ```
/// use graph_oauth::oauth::OAuth;
/// let oauth = OAuth::new();
/// // or
/// let oauth = OAuth::default();
/// ```
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, ToFile)]
pub struct OAuth {
    access_token: Option<AccessToken>,
    scopes: Vec<String>,
    credentials: BTreeMap<String, String>,
    custom_credentials: BTreeMap<String, String>,
}

impl OAuth {
    /// Create a new OAuth instance.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::OAuthCredential;
    ///
    /// let mut oauth = OAuth::new();
    ///
    /// oauth.client_id("client_id")
    ///     .client_secret("client_secret");
    ///
    /// println!("{:#?}", oauth.get(OAuthCredential::ClientId));
    /// println!("{:#?}", oauth.get(OAuthCredential::ClientSecret));
    /// ```
    pub fn new() -> OAuth {
        OAuth {
            access_token: None,
            scopes: Vec::new(),
            credentials: BTreeMap::new(),
            custom_credentials: BTreeMap::new(),
        }
    }

    /// Insert oauth credentials using the OAuthCredential enum.
    /// This method is used internally for each of the setter methods.
    /// Callers can optionally use this method to set credentials instead
    /// of the individual setter methods.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::OAuthCredential;
    ///
    /// let mut oauth = OAuth::new();
    ///
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
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::OAuthCredential;
    ///
    /// let mut oauth = OAuth::new();
    ///
    /// let a = oauth.get(OAuthCredential::AuthorizeURL);
    /// ```
    pub fn get(&self, oac: OAuthCredential) -> Option<String> {
        self.credentials.get(oac.alias()).cloned()
    }

    /// Check if an OAuth credential has already been set.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::OAuthCredential;
    ///
    /// let mut oauth = OAuth::new();
    ///
    /// println!("{:#?}", oauth.contains(OAuthCredential::Nonce));
    /// ```
    pub fn contains(&self, t: OAuthCredential) -> bool {
        self.credentials.contains_key(t.alias())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.credentials.contains_key(key)
    }

    /// Remove a field from OAuth.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::OAuthCredential;
    ///
    /// let mut oauth = OAuth::new();
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
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::OAuthCredential;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.client_id("client_id");
    /// ```
    pub fn client_id(&mut self, client_id: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ClientId, client_id.into())
    }

    /// Set the state for an OAuth request.
    ///
    /// # Example
    /// ```
    ///
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.state("1234");
    /// ```
    pub fn state(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::State, value.into())
    }

    /// Set the client secret for an OAuth request.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.client_secret("client_secret");
    /// ```
    pub fn client_secret(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ClientSecret, value.into())
    }

    /// Set the authorization URL.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.authorize_url("https://example.com/authorize");
    /// ```
    pub fn authorize_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AuthorizeURL, value.into())
    }

    /// Set the access token url of a request for OAuth
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.access_token_url("https://example.com/token");
    /// ```
    pub fn access_token_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AccessTokenURL, value.into())
    }

    /// Set the refresh token url of a request for OAuth
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.refresh_token_url("https://example.com/token");
    /// ```
    pub fn refresh_token_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::RefreshTokenURL, value.into())
    }

    /// Set the redirect url of a request
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.redirect_url("https://localhost:8888/redirect");
    /// ```
    pub fn redirect_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::RedirectURI, value.into())
    }

    /// Set the access code.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.access_code("LDSF[POK43");
    /// ```
    pub fn access_code(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AccessCode, value.into())
    }

    /// Set the response mode.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.response_mode("query");
    /// ```
    pub fn response_mode(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ResponseMode, value.into())
    }

    /// Set the response type.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.response_type("token");
    /// ```
    pub fn response_type(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ResponseType, value.into())
    }

    /// Set the nonce.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.nonce("1234");
    /// ```
    pub fn nonce(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::Nonce, value.into())
    }

    /// Set the prompt for open id.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.prompt("login");
    /// ```
    pub fn prompt(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::Prompt, value.into())
    }

    /// Set id token for open id.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::{OAuth, IdToken};
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.id_token(IdToken::new("1345"));
    /// ```
    pub fn id_token(&mut self, value: IdToken) -> &mut OAuth {
        self.insert(OAuthCredential::IdToken, value.get_id_token());
        if let Some(code) = value.get_code() {
            self.insert(OAuthCredential::AccessCode, code);
        }
        if let Some(state) = value.get_state() {
            let _ = self.entry(OAuthCredential::State, state.as_str());
        }
        if let Some(session_state) = value.get_session_state() {
            self.insert(OAuthCredential::SessionState, session_state);
        }
        self.insert(OAuthCredential::IdToken, value.get_id_token())
    }

    /// Set the grant_type.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.grant_type("token");
    /// ```
    pub fn grant_type(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::GrantType, value.into())
    }

    /// Set the url to send a post request that will log out the user.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.logout_url("https://example.com/logout?");
    /// ```
    pub fn logout_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::LogoutURL, value.into())
    }

    /// Set the redirect uri that user will be redirected to after logging out.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.post_logout_redirect_uri("http://localhost:8080");
    /// ```
    pub fn post_logout_redirect_uri(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::PostLogoutRedirectURI, value.into())
    }

    /// Add a scope' for the OAuth URL.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
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
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
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
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    ///
    /// // the scopes take a separator just like Vec join.
    ///  oauth.add_scope("scope");
    ///  oauth.remove_scope("scope");
    /// ```
    pub fn remove_scope(&mut self, scope: &str) {
        self.scopes.retain(|x| x != scope);
    }

    /// Set a custom OAuth credential.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    ///
    /// // Used in the body of the request as: &key=value
    /// oauth.custom("key", "value");
    /// // For instance:
    /// oauth.custom("nonce", "1234");
    /// ```
    pub fn custom(&mut self, key: &str, value: &str) -> &mut OAuth {
        self.custom_credentials.insert(key.into(), value.into());
        self
    }

    /// Set the access token.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::AccessToken;
    ///
    /// let a = AccessToken::default();
    /// let mut oauth = OAuth::new();
    ///
    /// oauth.access_token(a);
    /// ```
    pub fn access_token(&mut self, ac: AccessToken) {
        self.access_token.replace(ac);
    }

    /// Get the access token.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::AccessToken;
    ///
    /// let access_token = AccessToken::default();
    /// let mut oauth = OAuth::new();
    /// oauth.access_token(access_token);
    ///
    /// let access_token = oauth.get_access_token().unwrap();
    /// println!("{:#?}", access_token);
    /// ```
    pub fn get_access_token(&self) -> Option<AccessToken> {
        self.access_token.clone()
    }
}

// OAuth impl for error handling.
impl OAuth {
    fn error_from<T>(c: OAuthCredential) -> Result<T, OAuthError> {
        Err(OAuth::credential_error(c))
    }

    fn credential_error(c: OAuthCredential) -> OAuthError {
        OAuthError::error_kind(
            ErrorKind::NotFound,
            format!("MISSING OR INVALID: {:#?}", c).as_str(),
        )
    }

    fn get_or_else(&self, c: OAuthCredential) -> Result<String, OAuthError> {
        self.get(c).ok_or_else(|| OAuth::credential_error(c))
    }
}

impl OAuth {
    pub fn open_in_browser(&self, url: String) -> std::result::Result<Output, OAuthError> {
        let url = url.as_str();
        webbrowser::open(url).map_err(OAuthError::from)
    }

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
        self.open_in_browser(vec.join(""))
    }

    pub fn v2_logout(&self) -> OAuthReq<Output> {
        let mut url = self.get_or_else(OAuthCredential::LogoutURL)?;
        if let Some(redirect) = self.get(OAuthCredential::PostLogoutRedirectURI) {
            url.push_str(redirect.as_str());
        } else {
            let redirect_uri = self.get_or_else(OAuthCredential::RedirectURI)?;
            url.push_str(redirect_uri.as_str());
        }
        self.open_in_browser(url)
    }
}

impl OAuth {
    pub fn get_refresh_token(&self) -> OAuthReq<String> {
        match self.get_access_token() {
            Some(token) => match token.get_refresh_token() {
                Some(t) => Ok(t),
                None => OAuth::error_from::<String>(OAuthCredential::RefreshToken),
            },
            None => OAuth::error_from::<String>(OAuthCredential::AccessToken),
        }
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

    fn client(&self, url: &str, code_body: &str, content_type: &str) -> OAuthReq<RequestBuilder> {
        let client = reqwest::Client::builder().build()?;
        let access_code = self.get_or_else(OAuthCredential::AccessCode)?;
        Ok(client
            .post(url)
            .header(header::AUTHORIZATION, access_code.as_str())
            .header(header::CONTENT_TYPE, content_type)
            .body(code_body.to_string()))
    }

    pub fn encode_uri(&mut self, grant_type: GrantType) -> OAuthReq<String> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        match grant_type {
            GrantType::ClientCredentials(request_type) => match request_type {
                GrantRequest::Authorization => {
                    let _ = self.entry(OAuthCredential::ResponseType, "code");
                    let _ = self.entry(OAuthCredential::ResponseMode, "query");
                    self.form_encode_credentials(
                        vec![
                            OAuthCredential::ClientId,
                            OAuthCredential::ClientSecret,
                            OAuthCredential::RedirectURI,
                            OAuthCredential::State,
                            OAuthCredential::ResponseMode,
                            OAuthCredential::ResponseType,
                            OAuthCredential::Scopes,
                        ],
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
                        vec![
                            OAuthCredential::ClientId,
                            OAuthCredential::ClientSecret,
                            OAuthCredential::RedirectURI,
                            OAuthCredential::ResponseType,
                            OAuthCredential::GrantType,
                            OAuthCredential::AccessCode,
                        ],
                        &mut encoder,
                    );
                    Ok(encoder.finish())
                },
                GrantRequest::RefreshToken => {
                    let _ = self.entry(OAuthCredential::GrantType, "refresh_token");
                    let refresh_token = self.get_refresh_token()?;
                    encoder.append_pair("refresh_token", &refresh_token);
                    self.form_encode_credentials(
                        vec![
                            OAuthCredential::ClientId,
                            OAuthCredential::ClientSecret,
                            OAuthCredential::RedirectURI,
                            OAuthCredential::GrantType,
                            OAuthCredential::AccessCode,
                        ],
                        &mut encoder,
                    );
                    Ok(encoder.finish())
                },
            },
            GrantType::Implicit(request_type) => match request_type {
                GrantRequest::Authorization |
                GrantRequest::AccessToken |
                GrantRequest::RefreshToken => {
                    if !self.scopes.is_empty() &&
                        (self.scopes.contains(&"offline_access".to_string()) ||
                            self.scopes.contains(&"wl.offline_access".to_string()))
                    {
                        return Err(OAuthError::error_kind(
                            ErrorKind::InvalidData,
                            "Implicit grant types cannot use offline access scopes",
                        ));
                    }

                    let _ = self.entry(OAuthCredential::ResponseType, "token");
                    self.form_encode_credentials(
                        vec![
                            OAuthCredential::ClientId,
                            OAuthCredential::RedirectURI,
                            OAuthCredential::Scopes,
                            OAuthCredential::ResponseType,
                        ],
                        &mut encoder,
                    );
                    let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
                    if !url.ends_with('?') {
                        url.push('?');
                    }
                    url.push_str(encoder.finish().as_str());
                    Ok(url)
                },
            },
            GrantType::OpenId(request_type) => match request_type {
                GrantRequest::Authorization => {
                    let _ = self.entry(OAuthCredential::ResponseMode, "form_post");
                    self.form_encode_credentials(
                        vec![
                            OAuthCredential::ClientId,
                            OAuthCredential::ResponseType,
                            OAuthCredential::RedirectURI,
                            OAuthCredential::ResponseMode,
                            OAuthCredential::Scopes,
                            OAuthCredential::State,
                            OAuthCredential::Nonce,
                            OAuthCredential::Prompt,
                            OAuthCredential::Resource,
                        ],
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
                        vec![
                            OAuthCredential::ClientId,
                            OAuthCredential::ClientSecret,
                            OAuthCredential::ResponseType,
                            OAuthCredential::RedirectURI,
                            OAuthCredential::ResponseMode,
                            OAuthCredential::Scopes,
                            OAuthCredential::State,
                            OAuthCredential::Nonce,
                            OAuthCredential::Resource,
                        ],
                        &mut encoder,
                    );
                    Ok(encoder.finish())
                },
                GrantRequest::RefreshToken => unimplemented!(),
            },
        }
    }
}

impl ClientCredentialsGrant for OAuth {
    fn request_authorization(&mut self) -> OAuthReq<Output> {
        // https://tools.ietf.org/html/rfc6749#section-4.1.1
        let url = self.encode_uri(GrantType::ClientCredentials(GrantRequest::Authorization))?;
        self.open_in_browser(url)
    }

    fn request_access_token(&mut self) -> OAuthReq<()> {
        let url = self.get_or_else(OAuthCredential::AccessTokenURL)?;
        let body = self.encode_uri(GrantType::ClientCredentials(GrantRequest::AccessToken))?;
        let builder = self.client(
            url.trim(),
            body.as_str(),
            "application/x-www-form-urlencoded",
        )?;
        let access_token = AccessToken::transform(builder)?;
        self.access_token(access_token);
        Ok(())
    }

    fn request_refresh_token(&mut self) -> OAuthReq<()> {
        let url = self.get_or_else(OAuthCredential::RefreshTokenURL)?;
        let body = self.encode_uri(GrantType::ClientCredentials(GrantRequest::RefreshToken))?;
        let builder = self.client(
            url.trim(),
            body.as_str(),
            "application/x-www-form-urlencoded",
        )?;
        let access_token = AccessToken::transform(builder)?;
        self.access_token(access_token);
        Ok(())
    }
}

impl ImplicitGrant for OAuth {
    fn request_access_token(&mut self) -> OAuthReq<Output> {
        // https://tools.ietf.org/html/rfc6749#section-4.2
        let url = self.encode_uri(GrantType::Implicit(GrantRequest::AccessToken))?;
        self.open_in_browser(url)
    }
}

impl OpenId for OAuth {
    fn request_authorization(&mut self) -> OAuthReq<Output> {
        let url = self.encode_uri(GrantType::OpenId(GrantRequest::Authorization))?;
        self.open_in_browser(url)
    }

    fn request_access_token(&mut self) -> OAuthReq<()> {
        let url = self.get_or_else(OAuthCredential::AccessTokenURL)?;
        let body = self.encode_uri(GrantType::OpenId(GrantRequest::AccessToken))?;
        let builder = self.client(
            url.trim(),
            body.as_str(),
            "application/x-www-form-urlencoded",
        )?;
        let access_token = AccessToken::transform(builder)?;
        self.access_token(access_token);
        Ok(())
    }
}
