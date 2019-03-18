use crate::accesstoken::AccessToken;
use crate::encode::Encoder;
use crate::grants::{ClientCredentialsGrant, ImplicitGrant};
use crate::oautherror::OAuthError;
use graph_error::GraphError;
use reqwest::header;
use reqwest::Response;
use std::cell::RefCell;
use std::collections::btree_map::BTreeMap;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::io::ErrorKind;
use std::process::Output;
use std::string::ToString;
use strum::IntoEnumIterator;
use transform_request::prelude::*;
use url::form_urlencoded;
use url::percent_encoding::USERINFO_ENCODE_SET;
use webbrowser;

pub type OAuthReq<T> = Result<T, OAuthError>;

/// OAuthCredential list fields representing common OAuth credentials needed
/// to perform authentication in various formats such as token flow and
/// client credentials flow.
///
/// OAuthCredential is also represented by the enum Credential in order to
/// reference these fields without having to pass a String value.
#[derive(
    Debug,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    ToString,
    EnumDiscriminants,
    EnumMessage,
    EnumIter,
)]
#[strum_discriminants(name(Credential))]
#[strum_discriminants(derive(Ord, PartialOrd))]
#[strum_discriminants(derive(Serialize, Deserialize, Hash, EnumIter, ToString))]
pub enum OAuthCredential {
    #[strum(serialize = "client_id")]
    ClientId(String),
    #[strum(serialize = "client_secret")]
    ClientSecret(String),
    #[strum(serialize = "sign_in_url")]
    AuthorizeURL(String),
    #[strum(serialize = "access_token_url")]
    AccessTokenURL(String),
    #[strum(serialize = "refresh_token_url")]
    RefreshTokenURL(String),
    #[strum(serialize = "redirect_uri")]
    RedirectURI(String),
    #[strum(serialize = "code")]
    AccessCode(String),
    #[strum(serialize = "access_token")]
    AccessToken(String),
    #[strum(serialize = "refresh_token")]
    RefreshToken(String),
    #[strum(serialize = "response_mode")]
    ResponseMode(String),
    #[strum(serialize = "state")]
    State(String),
    #[strum(serialize = "response_type")]
    ResponseType(String),
    #[strum(serialize = "grant_type")]
    GrantType(String),
    #[strum(serialize = "nonce")]
    Nonce(String),
    #[strum(serialize = "username")]
    Username(String),
    #[strum(serialize = "password")]
    Password(String),
    #[strum(serialize = "scopes")]
    Scopes(String),
    #[strum(serialize = "post_logout_redirect_uri")]
    PostLogoutRedirectURI(String),
    #[strum(serialize = "logout_url")]
    LogoutURL(String),
}

impl OAuthCredential {
    pub fn hash(&self) -> u64 {
        let to_hash = move |value: String| {
            let mut s = DefaultHasher::new();
            value.hash(&mut s);
            s.finish()
        };
        to_hash(self.to_string())
    }

    pub fn fn_on_inner<F, T>(&self, f: F) -> T
    where
        F: Fn(&String) -> T,
    {
        match self {
            OAuthCredential::ClientId(s) => f(s),
            OAuthCredential::RedirectURI(s) => f(s),
            OAuthCredential::ClientSecret(s) => f(s),
            OAuthCredential::AuthorizeURL(s) => f(s),
            OAuthCredential::AccessTokenURL(s) => f(s),
            OAuthCredential::RefreshTokenURL(s) => f(s),
            OAuthCredential::AccessCode(s) => f(s),
            OAuthCredential::ResponseMode(s) => f(s),
            OAuthCredential::RefreshToken(s) => f(s),
            OAuthCredential::AccessToken(s) => f(s),
            OAuthCredential::State(s) => f(s),
            OAuthCredential::ResponseType(s) => f(s),
            OAuthCredential::GrantType(s) => f(s),
            OAuthCredential::Nonce(s) => f(s),
            OAuthCredential::Username(s) => f(s),
            OAuthCredential::Password(s) => f(s),
            OAuthCredential::Scopes(s) => f(s),
            OAuthCredential::LogoutURL(s) => f(s),
            OAuthCredential::PostLogoutRedirectURI(s) => f(s),
        }
    }

    pub fn inner_to_string(&self) -> String {
        match self {
            OAuthCredential::ClientId(s) => s.to_string(),
            OAuthCredential::ClientSecret(s) => s.to_string(),
            OAuthCredential::RedirectURI(s) => s.to_string(),
            OAuthCredential::AuthorizeURL(s) => s.to_string(),
            OAuthCredential::AccessTokenURL(s) => s.to_string(),
            OAuthCredential::RefreshTokenURL(s) => s.to_string(),
            OAuthCredential::AccessCode(s) => s.to_string(),
            OAuthCredential::ResponseMode(s) => s.to_string(),
            OAuthCredential::RefreshToken(s) => s.to_string(),
            OAuthCredential::AccessToken(s) => s.to_string(),
            OAuthCredential::State(s) => s.to_string(),
            OAuthCredential::ResponseType(s) => s.to_string(),
            OAuthCredential::GrantType(s) => s.to_string(),
            OAuthCredential::Nonce(s) => s.to_string(),
            OAuthCredential::Username(s) => s.to_string(),
            OAuthCredential::Password(s) => s.to_string(),
            OAuthCredential::Scopes(s) => s.to_string(),
            OAuthCredential::LogoutURL(s) => s.to_string(),
            OAuthCredential::PostLogoutRedirectURI(s) => s.to_string(),
        }
    }

    pub fn hash_inner(self) -> u64 {
        let to_hash = move |value: String| {
            let mut s = DefaultHasher::new();
            value.hash(&mut s);
            s.finish()
        };

        match self {
            OAuthCredential::ClientId(s) => to_hash(s),
            OAuthCredential::ClientSecret(s) => to_hash(s),
            OAuthCredential::RedirectURI(s) => to_hash(s),
            OAuthCredential::AuthorizeURL(s) => to_hash(s),
            OAuthCredential::AccessTokenURL(s) => to_hash(s),
            OAuthCredential::RefreshTokenURL(s) => to_hash(s),
            OAuthCredential::AccessCode(s) => to_hash(s),
            OAuthCredential::ResponseMode(s) => to_hash(s),
            OAuthCredential::RefreshToken(s) => to_hash(s),
            OAuthCredential::AccessToken(s) => to_hash(s),
            OAuthCredential::State(s) => to_hash(s),
            OAuthCredential::ResponseType(s) => to_hash(s),
            OAuthCredential::GrantType(s) => to_hash(s),
            OAuthCredential::Nonce(s) => to_hash(s),
            OAuthCredential::Username(s) => to_hash(s),
            OAuthCredential::Password(s) => to_hash(s),
            OAuthCredential::Scopes(s) => to_hash(s),
            OAuthCredential::LogoutURL(s) => to_hash(s),
            OAuthCredential::PostLogoutRedirectURI(s) => to_hash(s),
        }
    }

    pub fn alias(&self) -> &'static str {
        match *self {
            OAuthCredential::ClientId(_) => "client_id",
            OAuthCredential::ClientSecret(_) => "client_secret",
            OAuthCredential::AuthorizeURL(_) => "sign_in_url",
            OAuthCredential::AccessTokenURL(_) => "access_token_url",
            OAuthCredential::RefreshTokenURL(_) => "refresh_token_url",
            OAuthCredential::RedirectURI(_) => "redirect_uri",
            OAuthCredential::AccessCode(_) => "code",
            OAuthCredential::AccessToken(_) => "access_token",
            OAuthCredential::RefreshToken(_) => "refresh_token",
            OAuthCredential::ResponseMode(_) => "response_mode",
            OAuthCredential::ResponseType(_) => "response_type",
            OAuthCredential::State(_) => "state",
            OAuthCredential::GrantType(_) => "grant_type",
            OAuthCredential::Nonce(_) => "nonce",
            OAuthCredential::Username(_) => "username",
            OAuthCredential::Password(_) => "password",
            OAuthCredential::Scopes(_) => "scope",
            OAuthCredential::LogoutURL(_) => "logout_url",
            OAuthCredential::PostLogoutRedirectURI(_) => "post_logout_redirect_uri",
        }
    }

    pub fn hash_alias(&self) -> u64 {
        let to_hash = move |value: &str| {
            let mut s = DefaultHasher::new();
            value.hash(&mut s);
            s.finish()
        };

        to_hash(self.alias())
    }
}

impl From<Credential> for OAuthCredential {
    fn from(c: Credential) -> OAuthCredential {
        match c {
            Credential::ClientId => OAuthCredential::ClientId("".into()),
            Credential::ClientSecret => OAuthCredential::ClientSecret("".into()),
            Credential::AuthorizeURL => OAuthCredential::AuthorizeURL("".into()),
            Credential::AccessTokenURL => OAuthCredential::AccessTokenURL("".into()),
            Credential::RefreshTokenURL => OAuthCredential::RefreshTokenURL("".into()),
            Credential::RedirectURI => OAuthCredential::RedirectURI("".into()),
            Credential::AccessCode => OAuthCredential::AccessCode("".into()),
            Credential::AccessToken => OAuthCredential::AccessToken("".into()),
            Credential::RefreshToken => OAuthCredential::RefreshToken("".into()),
            Credential::ResponseMode => OAuthCredential::ResponseMode("".into()),
            Credential::ResponseType => OAuthCredential::ResponseType("".into()),
            Credential::State => OAuthCredential::State("".into()),
            Credential::GrantType => OAuthCredential::GrantType("".into()),
            Credential::Nonce => OAuthCredential::Nonce("".into()),
            Credential::Username => OAuthCredential::Username("".into()),
            Credential::Password => OAuthCredential::Password("".into()),
            Credential::Scopes => OAuthCredential::Scopes("".into()),
            Credential::LogoutURL => OAuthCredential::LogoutURL("".into()),
            Credential::PostLogoutRedirectURI => OAuthCredential::PostLogoutRedirectURI("".into()),
        }
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
    access_token: Option<RefCell<AccessToken>>,
    scopes: Vec<String>,
    cred: BTreeMap<u64, OAuthCredential>,
    custom_cred: Option<RefCell<Vec<HashMap<String, String>>>>,
}

impl OAuth {
    pub fn new() -> OAuth {
        OAuth {
            access_token: None,
            scopes: Vec::new(),
            cred: BTreeMap::new(),
            custom_cred: None,
        }
    }
    /// Internal method to insert well known oauth credentials.
    fn insert(&mut self, oac: OAuthCredential) -> &mut OAuth {
        self.cred.insert(oac.hash_alias(), oac);
        self
    }

    /// Remove a previously set credential.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::Credential;
    ///
    /// let mut oauth = OAuth::new();
    ///
    /// oauth.client_id("client_id");
    /// assert_eq!(oauth.contains(Credential::ClientId), true);
    ///
    /// oauth.remove(Credential::ClientId);
    /// assert_eq!(oauth.contains(Credential::ClientId), false);
    /// ```
    pub fn remove(&mut self, c: Credential) -> &mut OAuth {
        let oac = OAuthCredential::from(c);
        self.cred.remove(&oac.hash_alias());
        self
    }

    /// Set the client secret for an OAuth request.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.client_id("client_id");
    /// ```
    pub fn client_id(&mut self, client_id: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ClientId(client_id.into()))
    }

    /// Set the client secret for an OAuth request.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.state("1234");
    /// ```
    pub fn state(&mut self, state: &str) -> &mut OAuth {
        self.insert(OAuthCredential::State(state.into()))
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
    pub fn client_secret(&mut self, client_secret: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ClientSecret(client_secret.into()))
    }

    /// Set the auth url of a request
    /// Set the authorization URL for OAuth.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.authorize_url("https://example.com/authorize");
    /// ```
    pub fn authorize_url(&mut self, authorize_url: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AuthorizeURL(authorize_url.into()))
    }

    /// Set the token url of a request for OAuth
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.access_token_url("https://example.com/token");
    /// ```
    pub fn access_token_url(&mut self, access_token_url: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AccessTokenURL(access_token_url.into()))
    }

    /// Set the token url of a request for OAuth
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.refresh_token_url("https://example.com/token");
    /// ```
    pub fn refresh_token_url(&mut self, refresh_token_url: &str) -> &mut OAuth {
        self.insert(OAuthCredential::RefreshTokenURL(refresh_token_url.into()))
    }

    /// Set the redirect uri of a request
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.redirect_url("https://localhost:8888/redirect");
    /// ```
    pub fn redirect_url(&mut self, redirect_url: &str) -> &mut OAuth {
        self.insert(OAuthCredential::RedirectURI(redirect_url.into()))
    }

    /// Set the redirect uri.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.access_code("LDSF[POK43");
    /// ```
    pub fn access_code(&mut self, access_code: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AccessCode(access_code.into()))
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
    pub fn response_mode(&mut self, response_mode: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ResponseMode(response_mode.into()))
    }

    /// Set the response mode.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.response_type("token");
    /// ```
    pub fn response_type(&mut self, response_type: &str) -> &mut OAuth {
        self.insert(OAuthCredential::ResponseType(response_type.into()))
    }

    /// Set the response mode.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.nonce("1234");
    /// ```
    pub fn nonce(&mut self, nonce: &str) -> &mut OAuth {
        self.insert(OAuthCredential::Nonce(nonce.into()))
    }
    /// Set the response mode.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.username("username");
    /// ```
    pub fn username(&mut self, u: &str) -> &mut OAuth {
        self.insert(OAuthCredential::Username(u.into()))
    }

    /// Set the response mode.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.password("password");
    /// ```
    pub fn password(&mut self, p: &str) -> &mut OAuth {
        self.insert(OAuthCredential::Password(p.into()))
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
        self.access_token.replace(RefCell::new(ac));
    }

    /// Set the response mode.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.grant_type("token");
    /// ```
    pub fn grant_type(&mut self, s: &str) -> &mut OAuth {
        self.insert(OAuthCredential::GrantType(s.into()))
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
    pub fn logout_url(&mut self, url: &str) -> &mut OAuth {
        self.insert(OAuthCredential::LogoutURL(url.into()))
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
    pub fn post_logout_redirect_uri(&mut self, uri: &str) -> &mut OAuth {
        self.insert(OAuthCredential::PostLogoutRedirectURI(uri.into()))
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
        let s = self.scopes.clone();
        self.insert(OAuthCredential::Scopes(s.join(" ")));
        self
    }

    /// Add a scope' for the OAuth URL.
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

    /// Get a previously set credential.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::Credential;
    ///
    /// let mut oauth = OAuth::new();
    ///
    /// let a = oauth.get(Credential::AuthorizeURL);
    /// ```
    pub fn get(&self, c: Credential) -> Option<String> {
        let t = self.cred.get(&OAuthCredential::from(c).hash_alias());

        let oac = match t {
            Some(t) => t,
            None => return None,
        };
        let s: String = oac.inner_to_string();
        Some(s.to_string())
    }

    /// Check if an OAuth credential has already been set.
    /// Add a scope' for the OAuth URL.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    /// use graph_oauth::oauth::Credential;
    ///
    /// let mut oauth = OAuth::new();
    ///
    /// println!("{:#?}", oauth.contains(Credential::Nonce));
    /// ```
    pub fn contains(&self, t: Credential) -> bool {
        self.get(t).is_some()
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
        if let Some(c) = &self.custom_cred {
            let mut c_clone = c.borrow_mut();
            let mut hm: HashMap<String, String> = HashMap::new();
            hm.insert(key.into(), value.into());
            c_clone.push(hm);
        } else {
            let mut c = Vec::new();
            let mut hm: HashMap<String, String> = HashMap::new();
            hm.insert(key.into(), value.into());
            c.push(hm);
            self.custom_cred = Some(RefCell::new(c));
        }

        self
    }

    pub fn some_to_vec(&self) -> Vec<String> {
        Credential::iter()
            .filter(|x| self.get(*x).is_some())
            .map(|x| {
                [
                    "&",
                    OAuthCredential::from(x).alias(),
                    "=",
                    self.get(x).unwrap().as_str(),
                ]
                .join("")
            })
            .collect()
    }
}

// Methods for getting/requesting access codes (by sign in), access tokens, and refresh tokens.
impl OAuth {
    pub fn get_access_token(&self) -> Option<RefCell<AccessToken>> {
        self.access_token.clone()
    }

    pub fn request_json(
        &self,
        url: &str,
        access_code: &str,
        code_body: &str,
    ) -> OAuthReq<Response> {
        let client = reqwest::Client::builder().build()?;

        client
            .post(url)
            .header(header::AUTHORIZATION, access_code)
            .header(header::CONTENT_TYPE, "application/json")
            .body(code_body.to_string())
            .send()
            .map_err(OAuthError::from)
    }

    /// RFC 6750: https://www.rfc-editor.org/rfc/rfc6750.txt - Authorization Request Header Field
    #[allow(dead_code)]
    fn request(&self, url: &str, access_code: &str, code_body: &str) -> OAuthReq<Response> {
        let client = reqwest::Client::builder().build()?;

        client
            .post(url)
            .header(header::AUTHORIZATION, access_code)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(code_body.to_string())
            .send()
            .map_err(OAuthError::from)
    }

    pub fn encoded_refresh_token_uri(&self) -> std::result::Result<String, OAuthError> {
        self.encode_form_uri(Credential::RefreshTokenURL)
    }

    pub fn encoded_access_token_uri(&self) -> std::result::Result<String, OAuthError> {
        self.encode_form_uri(Credential::AccessTokenURL)
    }

    pub fn encoded_sign_in_url(&self) -> std::result::Result<String, OAuthError> {
        self.encode_url(Credential::AuthorizeURL)
    }

    fn encode_url(&self, parameter: Credential) -> OAuthReq<String> {
        let mut url = self.get_or_else(parameter)?;

        if !url.ends_with('?') {
            url.push('?');
        }

        let uri = self.encode_form_uri(parameter)?;
        url.push_str(uri.as_str());
        Ok(url)
    }

    fn set_encode_pair(
        &self,
        param: Credential,
        encoder: &mut url::form_urlencoded::Serializer<String>,
    ) -> std::result::Result<(), OAuthError> {
        match self.get(param) {
            Some(t) => {
                let s = OAuthCredential::from(param);
                encoder.append_pair(s.alias(), &t);
                Ok(())
            },
            None => OAuth::error_from::<()>(param),
        }
    }

    pub fn get_refresh_token(&self) -> OAuthReq<String> {
        match self.get_access_token() {
            Some(token) => token
                .try_borrow_mut()
                .and_then(|at| {
                    let token = at.clone();
                    let refresh = token.get_refresh_token().unwrap();
                    Ok(refresh)
                })
                .map_err(OAuthError::from),
            None => OAuth::error_from::<String>(Credential::AccessToken),
        }
    }

    pub fn encode_form_uri(&self, parameter: Credential) -> OAuthReq<String> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());

        self.set_encode_pair(Credential::ClientId, &mut encoder)?;
        self.set_encode_pair(Credential::RedirectURI, &mut encoder)?;

        // Optional parameter. Errors are ignored.
        let _ = self.set_encode_pair(Credential::ClientSecret, &mut encoder);

        match parameter {
            Credential::AuthorizeURL => {
                // https://tools.ietf.org/html/rfc6749#section-4.1.1
                if self.contains(Credential::ResponseMode) {
                    self.set_encode_pair(Credential::ResponseMode, &mut encoder)?;
                }

                encoder.append_pair("scope", self.get_scopes(" ").as_str());

                if self.contains(Credential::ResponseType) {
                    self.set_encode_pair(Credential::ResponseType, &mut encoder)?;
                } else {
                    encoder.append_pair("response_type", "code");
                }

                // Optional parameter. Errors are ignored.
                let _ = self.set_encode_pair(Credential::State, &mut encoder);
            },
            Credential::AccessTokenURL => {
                // https://tools.ietf.org/html/rfc6749#section-4.1.3
                if let Some(t) = self.get(Credential::ResponseMode) {
                    encoder.append_pair("response_mode", &t);
                }

                if let Some(t) = self.get(Credential::ResponseType) {
                    encoder.append_pair("response_type", &t);
                } else {
                    encoder.append_pair("response_type", "token");
                }

                self.set_encode_pair(Credential::AccessCode, &mut encoder)?;
                encoder.append_pair("grant_type", "authorization_code");
            },
            Credential::RefreshTokenURL => {
                // https://tools.ietf.org/html/rfc6749#section-6
                let refresh_token = self.get_refresh_token()?;
                encoder.append_pair("refresh_token", &refresh_token);

                if self.contains(Credential::GrantType) {
                    self.set_encode_pair(Credential::GrantType, &mut encoder)?;
                } else {
                    encoder.append_pair("grant_type", "refresh_token");
                }
            },
            _ => {},
        }

        self.encoded_custom_credentials(&mut encoder);

        Ok(encoder.finish())
    }

    #[allow(dead_code)]
    fn credential_to_pair(&self, c: OAuthCredential) -> String {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        let key = c.alias();
        let value = c.to_string();
        let value = value.as_str();
        encoder.append_pair(key, value);
        encoder.finish()
    }

    fn encoded_custom_credentials(&self, encoder: &mut form_urlencoded::Serializer<String>) {
        if let Some(c) = &self.custom_cred {
            let c_clone = c.borrow_mut().clone();
            for hm in c_clone.iter() {
                for (k, v) in hm {
                    encoder.append_pair(k.as_str(), v.as_str());
                }
            }
        }
    }

    pub fn v1_logout(&mut self) -> OAuthReq<Output> {
        let mut url = self.get_or_else(Credential::LogoutURL)?;
        if !url.ends_with('?') {
            url.push('?');
        }
        let client_id = self.get_or_else(Credential::ClientId)?;
        url.push_str("&client_id=");
        url.push_str(client_id.as_str());
        url.push_str("&redirect_uri=");
        if let Some(redirect) = self.get(Credential::PostLogoutRedirectURI) {
            url.push_str(redirect.as_str());
        } else if let Some(redirect) = self.get(Credential::RedirectURI) {
            url.push_str(redirect.as_str());
        }

        //client.get(url.as_str()).send().map_err(OAuthError::from)
        self.browser_sign_in_url(url)
    }

    pub fn v2_logout(&self) -> OAuthReq<Output> {
        let mut url = self.get_or_else(Credential::LogoutURL)?;
        if let Some(redirect) = self.get(Credential::PostLogoutRedirectURI) {
            url.push_str(redirect.as_str());
        } else {
            let redirect_uri = self.get_or_else(Credential::RedirectURI)?;
            url.push_str(redirect_uri.as_str());
        }
        self.browser_sign_in_url(url)
    }
}

// OAuth impl for error handling.
impl OAuth {
    pub fn error_from<T>(c: Credential) -> Result<T, OAuthError> {
        Err(OAuth::credential_error(c))
    }

    fn credential_error(c: Credential) -> OAuthError {
        OAuthError::error_kind(
            ErrorKind::NotFound,
            format!("MISSING OR INVALID: {:#?}", c).as_str(),
        )
    }

    fn get_or_else(&self, c: Credential) -> Result<String, OAuthError> {
        self.get(c).ok_or_else(|| OAuth::credential_error(c))
    }
}

impl OAuth {
    pub fn browser_sign_in(&mut self) -> Result<Output, OAuthError> {
        match self.encoded_sign_in_url() {
            Ok(t) => self.browser_sign_in_url(t),
            Err(e) => Err(e),
        }
    }

    pub fn browser_sign_in_url(&self, url: String) -> std::result::Result<Output, OAuthError> {
        let url = url.as_str();
        webbrowser::open(url).map_err(OAuthError::from)
    }
}

impl OAuth {
    pub fn form_encode_all_pairs(&mut self) -> Result<String, OAuthError> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        Credential::iter()
            .skip_while(|x| self.get(*x).is_none())
            .for_each(|x| {
                let _ = self.set_encode_pair(x, &mut encoder);
            });
        encoder.append_pair("scope", self.get_scopes(" ").as_str());
        Ok(encoder.finish())
    }

    pub fn form_encode_pairs(
        &mut self,
        filter: Vec<Credential>,
        mut encoder: &mut form_urlencoded::Serializer<String>,
    ) -> Result<(), OAuthError> {
        Credential::iter()
            .filter(|x| Some(x) == filter.iter().find(|t| *t == x))
            .skip_while(|x| self.get(*x).is_none())
            .for_each(|x| {
                self.set_encode_pair(x, &mut encoder).unwrap();
            });

        Ok(())
    }

    pub fn utf8_encode_pairs(&mut self, filter: Vec<Credential>) -> Result<String, OAuthError> {
        let mut s = String::new();
        Credential::iter()
            .filter(|x| Some(x) == filter.iter().find(|t| *t == x))
            .skip_while(|x| self.get(*x).is_none())
            .for_each(|x| {
                // self.set_encode_pair(x, &mut encoder);
                if let Some(t) = self.get(x) {
                    let key = OAuthCredential::from(x);
                    let v = vec!["&", key.alias(), "="];
                    s.push_str(v.join("").as_str());
                    s.push_str(Encoder::utf8_percent_encode(&t, USERINFO_ENCODE_SET).as_str())
                }
            });

        Ok(s)
    }
}

impl OAuth {
    pub fn utf8_encode_password_credentials(&mut self) -> Result<String, OAuthError> {
        // https://tools.ietf.org/html/rfc6749#section-4.3
        let mut pairs: HashMap<String, String> = HashMap::new();

        let pw = self
            .get(Credential::Password)
            .ok_or_else(|| OAuth::credential_error(Credential::Password))?;

        let username = self
            .get(Credential::Username)
            .ok_or_else(|| OAuth::credential_error(Credential::Username))?;

        pairs.insert("username".into(), username);
        pairs.insert("password".into(), pw);

        if self.contains(Credential::ClientId) {
            let ci = self.get_or_else(Credential::ClientId)?;
            pairs.insert("client_id".into(), ci);
        }

        match Encoder::utf8_url_encode_pairs(pairs, USERINFO_ENCODE_SET) {
            Ok(v) => {
                let mut s = String::from("grant_type=password");
                s.push_str(v.as_str());
                Ok(s)
            },
            Err(e) => Err(e),
        }
    }
}

impl ClientCredentialsGrant for OAuth {
    fn authorization_request(&mut self) -> OAuthReq<Output> {
        // https://tools.ietf.org/html/rfc6749#section-4.1.1
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        let vec_pairs = vec![
            Credential::ClientId,
            Credential::RedirectURI,
            Credential::ResponseType,
            Credential::ResponseMode,
        ];

        self.form_encode_pairs(vec_pairs, &mut encoder)
            .map_err(OAuthError::from)?;

        if let Some(state) = self.get(Credential::State) {
            encoder.append_pair("state", state.as_str());
        }

        if !self.scopes.is_empty() {
            encoder.append_pair("scope", self.scopes.join(" ").as_str());
        }

        if self.get(Credential::ResponseType).is_none() {
            encoder.append_pair("response_type", "code");
        }

        if self.get(Credential::ResponseMode).is_none() {
            encoder.append_pair("response_mode", "query");
        }

        let mut url = self.get(Credential::AuthorizeURL).unwrap().to_string();
        if !url.ends_with('?') {
            url.push('?');
        }
        url.push_str(encoder.finish().as_str());

        self.browser_sign_in_url(url)
    }

    fn request_access_token(&mut self) -> OAuthReq<()> {
        // The request URL.
        let url = self.get_or_else(Credential::AccessTokenURL)?;
        let access_code = self.get_or_else(Credential::AccessCode)?;

        // The request body.
        let uri = self.encoded_access_token_uri()?;

        let mut response = self.request(&url, &access_code, &uri)?;

        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(OAuthError::from(GraphError::from(status)));
        } else {
            let mut ac: AccessToken = response.json()?;
            ac.timestamp();
            self.access_token(ac);
        }

        Ok(())
    }

    fn request_refresh_token(&mut self) -> Result<(), OAuthError> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        let vec_pairs = vec![Credential::ClientId, Credential::RedirectURI];

        self.form_encode_pairs(vec_pairs, &mut encoder)
            .map_err(OAuthError::from)?;

        if self.contains(Credential::ClientSecret) {
            encoder.append_pair(
                "client_secret",
                self.get_or_else(Credential::ClientSecret)?.as_str(),
            );
        }

        let refresh_token = self.get_refresh_token()?;
        encoder.append_pair("refresh_token", refresh_token.as_str());
        encoder.append_pair("grant_type", "refresh_token");

        let url = self.get_or_else(Credential::RefreshTokenURL)?;
        let access_code = self.get_or_else(Credential::AccessCode)?;
        let body = encoder.finish();

        let mut response = self.request(&url, &access_code, &body)?;

        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(OAuthError::from(GraphError::from(status)));
        } else {
            let mut ac: AccessToken = response.json()?;
            ac.refresh_token(Some(refresh_token.clone().as_str()));
            ac.timestamp();
            self.access_token(ac);
        }

        Ok(())
    }
}

impl ImplicitGrant for OAuth {
    fn request_access_token(&mut self) -> OAuthReq<Output> {
        // https://tools.ietf.org/html/rfc6749#section-4.2
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        let mut map = HashMap::new();
        let client_id = self.get_or_else(Credential::ClientId)?;
        let redirect_uri = self.get_or_else(Credential::RedirectURI)?;

        map.insert("client_id", client_id.as_str());
        map.insert("redirect_uri", redirect_uri.as_str());

        let scope = self.get_scopes(" ");
        if !scope.is_empty() {
            if scope.contains("offline_access") || scope.contains("wl.offline_access") {
                return Err(OAuthError::error_kind(
                    ErrorKind::InvalidData,
                    "Implicit grant types cannot use offline access scopes",
                ));
            }
            map.insert("scope", scope.as_str());
        }

        map.insert("response_type", "token");
        map.iter().for_each(|(key, value)| {
            encoder.append_pair(*key, *value);
        });

        let mut url = self.get_or_else(Credential::AuthorizeURL)?;
        if !url.ends_with('?') {
            url.push('?');
        }

        let body = encoder.finish();
        url.push_str(body.as_str());

        self.browser_sign_in_url(url)
    }
}
