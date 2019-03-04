use crate::accesstoken::AccessToken;
use crate::encode::Encoder;
use crate::oautherror::OAuthResult;
pub use crate::oautherror::{OAuthError, OAuthReq};
use graph_error::GraphError;
use reqwest::header;
use reqwest::Response;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;
use std::io::ErrorKind;
use std::process::Command;
use std::string::ToString;
use std::thread;
use std::thread::JoinHandle;
use strum::IntoEnumIterator;
use url::form_urlencoded;
use url::percent_encoding::USERINFO_ENCODE_SET;

pub type ReqSend = reqwest::Result<Response>;

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
#[strum_discriminants(derive(EnumMessage, Ord, PartialOrd))]
#[strum_discriminants(derive(Serialize, Deserialize, Hash, EnumIter, ToString))]
pub enum OAuthCredential {
    #[strum(detailed_message = "Missing or malformed: client_id")]
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

    pub fn as_str(&self) -> &str {
        match self {
            OAuthCredential::ClientId(s) => s.as_str(),
            OAuthCredential::RedirectURI(s) => s.as_str(),
            OAuthCredential::ClientSecret(s) => s.as_str(),
            OAuthCredential::AuthorizeURL(s) => s.as_str(),
            OAuthCredential::AccessTokenURL(s) => s.as_str(),
            OAuthCredential::RefreshTokenURL(s) => s.as_str(),
            OAuthCredential::AccessCode(s) => s.as_str(),
            OAuthCredential::ResponseMode(s) => s.as_str(),
            OAuthCredential::RefreshToken(s) => s.as_str(),
            OAuthCredential::AccessToken(s) => s.as_str(),
            OAuthCredential::State(s) => s.as_str(),
            OAuthCredential::ResponseType(s) => s.as_str(),
            OAuthCredential::GrantType(s) => s.as_str(),
            OAuthCredential::Nonce(s) => s.as_str(),
            OAuthCredential::Username(s) => s.as_str(),
            OAuthCredential::Password(s) => s.as_str(),
            OAuthCredential::Scopes(s) => s.as_str(),
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
        }
    }

    pub fn alias(&self) -> &'static str {
        match self {
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
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OAuth {
    access_token: Option<RefCell<AccessToken>>,
    scopes: Vec<String>,
    cred: Box<HashMap<u64, OAuthCredential>>,
    custom_cred: Option<RefCell<Vec<HashMap<String, String>>>>,
}

impl OAuth {
    pub fn new() -> OAuth {
        OAuth {
            access_token: None,
            scopes: Vec::new(),
            cred: Box::new(HashMap::new()),
            custom_cred: None,
        }
    }
    /// Internal method to insert well known oauth credentials.
    pub fn insert(&mut self, oac: OAuthCredential) -> &mut OAuth {
        self.cred.insert(oac.hash_alias(), oac);
        self
    }

    pub fn remove(&mut self, oac: OAuthCredential) -> &mut OAuth {
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

    pub fn scope(&mut self, s: &str) -> &mut OAuth {
        self.insert(OAuthCredential::Scopes(s.into()))
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
    pub fn get(&self, c: Credential) -> Option<&str> {
        let t = self
            .cred
            .iter()
            .find(|v| v.0 == &OAuthCredential::from(c).hash());
        match t {
            Some(e) => Some(e.1.as_str()),
            None => None,
        }
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
            let mut hm = HashMap::new();
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
                    self.get(x).unwrap(),
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

    pub fn request_json(&self, url: &str, access_code: &str, code_body: &str) -> ReqSend {
        let client = reqwest::Client::builder().build()?;

        client
            .post(url)
            .header(header::AUTHORIZATION, access_code)
            .header(header::CONTENT_TYPE, "application/json")
            .body(code_body.to_string())
            .send()
    }

    /// RFC 6750: https://www.rfc-editor.org/rfc/rfc6750.txt - Authorization Request Header Field
    #[allow(dead_code)]
    fn request(&self, url: &str, access_code: &str, code_body: &str) -> ReqSend {
        let client = reqwest::Client::builder().build()?;

        client
            .post(url)
            .header(header::AUTHORIZATION, access_code)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(code_body.to_string())
            .send()
    }

    pub fn request_access_token(&mut self) -> OAuthReq<()> {
        // The request URL.
        let url = self
            .get(Credential::AccessTokenURL)
            .ok_or_else(|| OAuth::credential_error(Credential::AccessTokenURL))?;

        let access_code = self
            .get(Credential::AccessCode)
            .ok_or_else(|| OAuth::credential_error(Credential::AccessCode))?;

        // The request body.
        let uri = self.encoded_access_token_uri()?;

        let mut response = self.request(url, access_code, uri.as_str())?;
        let status = response.status().as_u16();

        if GraphError::is_error(status) {
            return Err(OAuthError::from(GraphError::from(status)));
        } else {
            let ac: AccessToken = response.json()?;
            self.access_token(ac);
        }

        Ok(())
    }

    /// Request an new AccessToken using the refresh token.
    pub fn request_refresh_token(&mut self) -> OAuthReq<()> {
        // The request URL.
        let url = self
            .get(Credential::AccessTokenURL)
            .ok_or_else(|| OAuth::credential_error(Credential::AccessTokenURL))?;

        let access_code = self
            .get(Credential::AccessCode)
            .ok_or_else(|| OAuth::credential_error(Credential::AccessCode))?;

        // The request body.
        let uri = self.encoded_access_token_uri()?;

        let cur_ac = self.access_token.clone();
        let current_ac = match cur_ac {
            Some(t) => t,
            None => return OAuth::error_from::<()>(Credential::AccessToken),
        };

        let ac_mut = current_ac.into_inner();

        let mut response = self.request_json(url, access_code, uri.as_str())?;
        let mut ac: AccessToken = AccessToken::from_response(&mut response)?;
        if let Some(t) = ac_mut.get_refresh_token() {
            ac.refresh_token(Some(t.as_str()));
        }

        self.access_token(ac);
        Ok(())
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
        if let Some(url) = self.get(parameter) {
            let mut url = url.to_string();
            if !url.ends_with('?') {
                url.push('?');
            }

            match self.encode_form_uri(parameter) {
                Ok(t) => url.push_str(t.to_string().as_str()),
                Err(e) => return Err(e),
            }

            return Ok(url);
        }

        OAuth::error_from(parameter)
    }

    fn set_encode_pair(
        &self,
        param: Credential,
        encoder: &mut url::form_urlencoded::Serializer<String>,
    ) -> std::result::Result<(), OAuthError> {
        match self.get(param) {
            Some(t) => {
                let s = OAuthCredential::from(param);
                encoder.append_pair(s.alias(), t);
                Ok(())
            },
            None => OAuth::error_from::<()>(param),
        }
    }

    pub fn get_refresh_token(&self) -> OAuthReq<String> {
        match self.get_access_token() {
            Some(t) => {
                let ac = t.try_borrow_mut();
                if let Ok(rt) = ac {
                    let token = rt.clone();
                    let refresh = token.get_refresh_token();
                    if let Some(t) = refresh {
                        return Ok(t);
                    } else {
                        return OAuth::error_from::<String>(Credential::RefreshToken);
                    }
                } else {
                    return OAuth::error_from::<String>(Credential::AccessToken);
                }
            },
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
                    encoder.append_pair("response_mode", t);
                }

                if let Some(t) = self.get(Credential::ResponseType) {
                    encoder.append_pair("response_type", t);
                } else {
                    encoder.append_pair("response_type", "token");
                }

                self.set_encode_pair(Credential::AccessCode, &mut encoder)?;
                encoder.append_pair("grant_type", "authorization_code");
            },
            Credential::RefreshTokenURL => {
                // https://tools.ietf.org/html/rfc6749#section-6
                if let Some(t) = self.get(Credential::ResponseType) {
                    encoder.append_pair("response_type", t);
                } else {
                    encoder.append_pair("response_type", "token");
                }

                match self.get_access_token() {
                    Some(t) => {
                        let ac = t.try_borrow_mut();
                        if let Ok(rt) = ac {
                            let token = rt.clone();
                            let refresh = token.get_refresh_token();
                            if let Some(t) = refresh {
                                encoder.append_pair("refresh_token", &t);
                            } else {
                                return OAuth::error_from::<String>(Credential::RefreshToken);
                            }
                        } else {
                            return OAuth::error_from::<String>(Credential::AccessToken);
                        }
                    },
                    None => return OAuth::error_from::<String>(Credential::AccessToken),
                }

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
        let value = c.as_str();
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
}

// OAuth impl for error handling.
impl OAuth {
    fn error_from<T>(c: Credential) -> std::result::Result<T, OAuthError> {
        Err(OAuth::credential_error(c))
    }

    fn credential_error(c: Credential) -> OAuthError {
        OAuthError::error_kind(
            ErrorKind::NotFound,
            format!("MISSING OR INVALID: {:#?}", c).as_str(),
        )
    }
}

impl OAuth {
    pub fn browser_sign_in(&mut self) -> std::result::Result<(), std::io::Error> {
        let u = self.encoded_sign_in_url().unwrap();
        let handle = thread::spawn(move || {
            let url = u.as_str();
            Command::new("xdg-open").arg(url).spawn().unwrap();
        });

        handle.join().unwrap();
        Ok(())
    }

    pub fn browser_sign_in_spawn(&mut self) -> Option<JoinHandle<()>> {
        match self.encoded_sign_in_url() {
            Ok(t) => Some(thread::spawn(move || {
                let url = t.as_str();
                Command::new("xdg-open")
                    .arg(url)
                    .spawn()
                    .expect("Could not open browser");
            })),
            Err(e) => {
                println!("{:#?}", e);
                None
            },
        }
    }

    pub fn browser_sign_in_url(&self, url: &str) -> std::result::Result<(), std::io::Error> {
        Command::new("xdg-open").arg(url).spawn()?;
        Ok(())
    }
}

impl OAuth {
    pub fn authorization_request(&mut self) -> Result<(), OAuthError> {
        // https://tools.ietf.org/html/rfc6749#section-4.1.1
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        let vec_pairs = vec![
            Credential::ClientId,
            Credential::RedirectURI,
            Credential::ResponseType,
            Credential::ResponseMode,
            Credential::State,
        ];

        self.form_encode_pairs(vec_pairs, &mut encoder)
            .map_err(OAuthError::from)?;
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

        self.browser_sign_in_url(&url).map_err(OAuthError::from)
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
                    s.push_str(Encoder::utf8_percent_encode(t, USERINFO_ENCODE_SET).as_str())
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

        pairs.insert("username".into(), username.into());
        pairs.insert("password".into(), pw.into());

        if self.contains(Credential::ClientId) {
            let ci = self
                .get(Credential::ClientId)
                .ok_or_else(|| OAuth::credential_error(Credential::ClientId))?;

            pairs.insert("client_id".into(), ci.into());
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

    pub fn implicit_grant(&mut self) -> OAuthReq<()> {
        // https://tools.ietf.org/html/rfc6749#section-4.2
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        let mut vec_pairs = vec![
            Credential::ClientId,
            Credential::RedirectURI,
            Credential::ResponseType,
        ];

        if self.contains(Credential::State) {
            vec_pairs.push(Credential::State);
        }
        self.form_encode_pairs(vec_pairs, &mut encoder)?;
        let scope = self.get_scopes(" ");
        if !scope.is_empty() {
            encoder.append_pair("scope", scope.as_str());
        }

        self.encoded_custom_credentials(&mut encoder);

        let url = self
            .get(Credential::AccessTokenURL)
            .ok_or_else(|| OAuth::credential_error(Credential::AccessTokenURL))?;

        let access_code = self
            .get(Credential::AccessCode)
            .ok_or_else(|| OAuth::credential_error(Credential::AccessCode))?;

        let body = encoder.finish();
        let mut response = self.request(url, access_code, body.as_str())?;
        let at: AccessToken = response.json()?;
        self.access_token(at);
        Ok(())
    }
}
