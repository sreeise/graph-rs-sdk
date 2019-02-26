use crate::accesstoken::AccessToken;
use crate::oautherror::OAuthError;
use reqwest::*;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::ErrorKind;
use std::process::Command;
use std::thread;
use std::thread::JoinHandle;
use url::define_encode_set;
use url::form_urlencoded;
use url::percent_encoding::DEFAULT_ENCODE_SET;

define_encode_set! {
    pub ONEDRIVE_RESERVED_ENCODE_SET = [DEFAULT_ENCODE_SET] | {'/', '*', '<', '>', '?', ':', '|', ' ', '#'}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum OAuthParam {
    ClientId = 0b1100101,
    ClientSecret = 0b1100110,
    SignInUrl = 0b1100111,
    AccessTokenURL = 0b1101000,
    RefreshTokenURL = 0b1101001,
    AccessCode = 0b1101010,
    CODE = 0b1101011,
    AccessToken = 0b1101100,
    RefreshToken = 0b1101101,
    ResponseMode = 0b1101110,
    AppIdResource = 0b1101111,
    STATE = 0b1110000,
    ResponseType = 0b1110001,
    GrantType = 0b1110010,
    RedirectUri = 0b1110011,
}

impl OAuthParam {
    pub fn hash(self) -> u64 {
        let mut s = DefaultHasher::new();
        s.write(self.as_alias().as_bytes());
        s.finish()
    }

    pub fn as_alias(self) -> &'static str {
        match self {
            OAuthParam::ClientId => "client_id",
            OAuthParam::ClientSecret => "client_secret",
            OAuthParam::SignInUrl => "sign_in_url",
            OAuthParam::AccessTokenURL => "access_token_url",
            OAuthParam::RefreshTokenURL => "refresh_token_url",
            OAuthParam::RedirectUri => "redirect_uri",
            OAuthParam::AccessCode => "code",
            OAuthParam::CODE => "code",
            OAuthParam::AccessToken => "access_token",
            OAuthParam::RefreshToken => "refresh_token",
            OAuthParam::ResponseMode => "response_mode",
            OAuthParam::ResponseType => "response_type",
            OAuthParam::AppIdResource => "app_id_resource",
            OAuthParam::STATE => "state",
            OAuthParam::GrantType => "grant_type",
        }
    }

    pub fn alias(self) -> OAuthCredential {
        match self {
            OAuthParam::ClientId => OAuthCredential::ClientId(self.as_alias().into()),
            OAuthParam::ClientSecret => OAuthCredential::ClientSecret(self.as_alias().into()),
            OAuthParam::SignInUrl => OAuthCredential::SignInURL(self.as_alias().into()),
            OAuthParam::AccessTokenURL => OAuthCredential::AccessTokenURL(self.as_alias().into()),
            OAuthParam::RefreshTokenURL => OAuthCredential::RefreshTokenURL(self.as_alias().into()),
            OAuthParam::RedirectUri => OAuthCredential::RedirectURI(self.as_alias().into()),
            OAuthParam::AccessCode => OAuthCredential::AccessCode(self.as_alias().into()),
            OAuthParam::CODE => OAuthCredential::Code(self.as_alias().into()),
            OAuthParam::AccessToken => OAuthCredential::AccessToken(self.as_alias().into()),
            OAuthParam::RefreshToken => OAuthCredential::RefreshToken(self.as_alias().into()),
            OAuthParam::ResponseMode => OAuthCredential::ResponseMode(self.as_alias().into()),
            OAuthParam::ResponseType => OAuthCredential::ResponseType(self.as_alias().into()),
            OAuthParam::AppIdResource => OAuthCredential::AppIdResource(self.as_alias().into()),
            OAuthParam::STATE => OAuthCredential::State(self.as_alias().into()),
            OAuthParam::GrantType => OAuthCredential::GrantType(self.as_alias().into()),
        }
    }

    pub fn as_uri_key(self) -> String {
        vec!["&", format!("{:?}", self).to_lowercase().as_str(), "="].join("")
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum OAuthCredential {
    ClientId(String),
    ClientSecret(String),
    SignInURL(String),
    AccessTokenURL(String),
    RefreshTokenURL(String),
    RedirectURI(String),
    AccessCode(String),
    Code(String),
    AccessToken(String),
    RefreshToken(String),
    ResponseMode(String),
    AppIdResource(String),
    State(String),
    ResponseType(String),
    GrantType(String),
}

impl OAuthCredential {
    pub fn as_str(&self) -> &str {
        match self {
            OAuthCredential::ClientId(s) => s.as_str(),
            OAuthCredential::RedirectURI(s) => s.as_str(),
            OAuthCredential::ClientSecret(s) => s.as_str(),
            OAuthCredential::SignInURL(s) => s.as_str(),
            OAuthCredential::AccessTokenURL(s) => s.as_str(),
            OAuthCredential::RefreshTokenURL(s) => s.as_str(),
            OAuthCredential::AccessCode(s) => s.as_str(),
            OAuthCredential::Code(s) => s.as_str(),
            OAuthCredential::ResponseMode(s) => s.as_str(),
            OAuthCredential::AppIdResource(s) => s.as_str(),
            OAuthCredential::RefreshToken(s) => s.as_str(),
            OAuthCredential::AccessToken(s) => s.as_str(),
            OAuthCredential::State(s) => s.as_str(),
            OAuthCredential::ResponseType(s) => s.as_str(),
            OAuthCredential::GrantType(s) => s.as_str(),
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
            OAuthCredential::SignInURL(s) => to_hash(s),
            OAuthCredential::AccessTokenURL(s) => to_hash(s),
            OAuthCredential::RefreshTokenURL(s) => to_hash(s),
            OAuthCredential::AccessCode(s) => to_hash(s),
            OAuthCredential::Code(s) => to_hash(s),
            OAuthCredential::ResponseMode(s) => to_hash(s),
            OAuthCredential::AppIdResource(s) => to_hash(s),
            OAuthCredential::RefreshToken(s) => to_hash(s),
            OAuthCredential::AccessToken(s) => to_hash(s),
            OAuthCredential::State(s) => to_hash(s),
            OAuthCredential::ResponseType(s) => to_hash(s),
            OAuthCredential::GrantType(s) => to_hash(s),
        }
    }

    pub fn alias(self) -> &'static str {
        match self {
            OAuthCredential::ClientId(_) => "client_id",
            OAuthCredential::ClientSecret(_) => "client_secret",
            OAuthCredential::SignInURL(_) => "sign_in_url",
            OAuthCredential::AccessTokenURL(_) => "access_token_url",
            OAuthCredential::RefreshTokenURL(_) => "refresh_token_url",
            OAuthCredential::RedirectURI(_) => "redirect_uri",
            OAuthCredential::AccessCode(_) => "code",
            OAuthCredential::Code(_) => "code",
            OAuthCredential::AccessToken(_) => "access_token",
            OAuthCredential::RefreshToken(_) => "refresh_token",
            OAuthCredential::ResponseMode(_) => "response_mode",
            OAuthCredential::ResponseType(_) => "response_type",
            OAuthCredential::AppIdResource(_) => "app_id_resource",
            OAuthCredential::State(_) => "state",
            OAuthCredential::GrantType(_) => "grant_type",
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<RefCell<AccessToken>>,
    scopes: Vec<String>,
    cred: Box<HashMap<u64, OAuthCredential>>,
}

impl OAuth {
    pub fn new() -> OAuth {
        OAuth {
            access_token: None,
            scopes: Vec::new(),
            cred: Box::new(HashMap::new()),
        }
    }

    fn account_tenant(&self, account_type: &str, endpoint: &str) -> String {
        let vec = vec![
            "https://login.microsoftonline.com/",
            account_type,
            "/oauth2/v2.0/",
            endpoint,
        ];

        vec.join("")
    }

    // TODO: The discovery endpoints that are used for
    pub fn for_common_native_accounts(&mut self) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::SignInUrl.hash(),
            OAuthCredential::SignInURL("https://login.live.com/oauth20_authorize.srf?".into()),
        );

        self.cred.insert(
            OAuthParam::AccessTokenURL.hash(),
            OAuthCredential::AccessTokenURL("https://login.live.com/oauth20_token.srf".into()),
        );

        self.cred.insert(
            OAuthParam::RefreshTokenURL.hash(),
            OAuthCredential::RefreshTokenURL("https://login.live.com/oauth20_token.srf".into()),
        );

        self
    }

    pub fn for_common_accounts(&mut self) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::SignInUrl.hash(),
            OAuthCredential::SignInURL(self.account_tenant("common", "authorize?")),
        );

        self.cred.insert(
            OAuthParam::AccessTokenURL.hash(),
            OAuthCredential::AccessTokenURL(self.account_tenant("common", "token?")),
        );

        self.cred.insert(
            OAuthParam::RefreshTokenURL.hash(),
            OAuthCredential::RefreshTokenURL(self.account_tenant("common", "token?")),
        );

        self
    }

    pub fn for_tenant_accounts(&mut self, tenant: &str) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::SignInUrl.hash(),
            OAuthCredential::SignInURL(self.account_tenant(tenant, "authorize?")),
        );

        self.cred.insert(
            OAuthParam::AccessTokenURL.hash(),
            OAuthCredential::AccessTokenURL(self.account_tenant(tenant, "token?")),
        );

        self.cred.insert(
            OAuthParam::RefreshTokenURL.hash(),
            OAuthCredential::RefreshTokenURL(self.account_tenant(tenant, "token?")),
        );

        self
    }

    pub fn for_organization_accounts(&mut self) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::SignInUrl.hash(),
            OAuthCredential::SignInURL(self.account_tenant("organizations", "authorize?")),
        );

        self.cred.insert(
            OAuthParam::AccessTokenURL.hash(),
            OAuthCredential::AccessTokenURL(self.account_tenant("organizations", "token?")),
        );

        self.cred.insert(
            OAuthParam::RefreshTokenURL.hash(),
            OAuthCredential::RefreshTokenURL(self.account_tenant("organizations", "token?")),
        );

        self
    }

    /// Adds the default scope url to the list of scopes: https://graph.microsoft.com/.default.
    /// This url allows using the scopes set for the application
    /// in api requests. These scopes are set in the application
    /// registration portal. Microsoft allows only web clients
    /// can use the default scope url.
    pub fn v1_default_scope(&mut self, use_default: bool) {
        if use_default {
            if self
                .scopes
                .iter()
                .position(|val| val.as_str() == "https://graph.microsoft.com/.default")
                .is_none()
            {
                self.scopes
                    .push("https://graph.microsoft.com/.default".to_string());
            }
        } else {
            self.scopes
                .iter()
                .position(|val| val.as_str() == "https://graph.microsoft.com/.default")
                .map(|idx| self.scopes.remove(idx));
        }
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
        self.cred.insert(
            OAuthParam::ClientId.hash(),
            OAuthCredential::ClientId(client_id.into()),
        );

        self
    }

    pub fn state(&mut self, state: &str) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::STATE.hash(),
            OAuthCredential::State(state.into()),
        );
        self
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
        self.cred.insert(
            OAuthParam::ClientSecret.hash(),
            OAuthCredential::ClientSecret(client_secret.into()),
        );

        self
    }

    /// Set the auth url of a request
    /// Set the authorization URL for OAuth.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::oauth::OAuth;
    ///
    /// let mut oauth = OAuth::new();
    /// oauth.sign_in_url("https://example.com/authorize");
    /// ```
    pub fn sign_in_url(&mut self, auth_url: &str) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::SignInUrl.hash(),
            OAuthCredential::SignInURL(auth_url.into()),
        );

        self
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
        self.cred.insert(
            OAuthParam::AccessTokenURL.hash(),
            OAuthCredential::AccessTokenURL(access_token_url.into()),
        );
        self
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
        self.cred.insert(
            OAuthParam::RefreshTokenURL.hash(),
            OAuthCredential::RefreshTokenURL(String::from(refresh_token_url)),
        );

        self
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
        self.cred.insert(
            OAuthParam::RedirectUri.hash(),
            OAuthCredential::RedirectURI(redirect_url.into()),
        );
        self
    }

    /// Set the code of a request - returned from log in and authorization
    pub fn access_code(&mut self, access_code: &str) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::AccessCode.hash(),
            OAuthCredential::AccessCode(access_code.into()),
        );
        self
    }

    pub fn response_mode(&mut self, response_mode: &str) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::ResponseMode.hash(),
            OAuthCredential::ResponseMode(response_mode.into()),
        );
        self
    }

    pub fn response_type(&mut self, response_type: &str) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::ResponseType.hash(),
            OAuthCredential::ResponseType(response_type.into()),
        );
        self
    }

    /// Set the code of a request - returned from log in and authorization
    pub fn app_id_resource(&mut self, app_id: &str) -> &mut OAuth {
        self.cred.insert(
            OAuthParam::AppIdResource.hash(),
            OAuthCredential::AppIdResource(app_id.into()),
        );
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
    /// oauth.add_scope("Read")
    ///     .add_scope("Write")
    ///     .add_scope("ReadWrite.All");
    /// ```
    pub fn add_scope(&mut self, scope: &str) -> &mut OAuth {
        self.scopes.push(String::from(scope));
        self
    }

    pub fn get_scopes(&self, sep: &str) -> String {
        self.scopes.join(sep).to_owned()
    }

    pub fn get(&self, t: OAuthParam) -> Option<&str> {
        match self.cred.get(&t.hash()) {
            Some(t) => Some(t.as_str()),
            None => None,
        }
    }

    pub fn contains(&self, t: OAuthParam) -> bool {
        self.cred.get(&t.hash()).is_some()
    }

    pub fn set(&mut self, t: OAuthParam, c: OAuthCredential) -> &mut OAuth {
        self.cred.insert(t.hash(), c);
        self
    }
}

// Methods for getting/requesting access codes (by sign in), access tokens, and refresh tokens.
impl OAuth {
    /// Set the access token.
    pub fn access_token(&mut self, ac: AccessToken) {
        self.access_token.replace(RefCell::new(ac));
    }

    pub fn get_access_token(&self) -> Option<RefCell<AccessToken>> {
        self.access_token.clone()
    }

    /// RFC 6750: https://www.rfc-editor.org/rfc/rfc6750.txt - Authorization Request Header Field
    fn request(&self, url: &str, access_code: &str, code_body: &str) -> Result<Response> {
        let client = reqwest::Client::builder().build()?;

        client
            .post(url)
            .header(header::AUTHORIZATION, access_code)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(code_body.to_string())
            .send()
    }

    pub fn request_access_token(&mut self) -> std::result::Result<(), OAuthError> {
        // The request URL.
        let u = match self.get(OAuthParam::AccessTokenURL) {
            Some(t) => t,
            None => return OAuth::error_from::<()>(OAuthParam::AccessTokenURL),
        };

        let access_code = match self.get(OAuthParam::AccessCode) {
            Some(t) => t,
            None => return OAuth::error_from::<()>(OAuthParam::AccessCode),
        };

        // The request body.
        let uri = match self.encoded_access_token_uri() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        let mut response = self.request(u, access_code, uri.as_str());
        let ac: AccessToken = AccessToken::from(&mut response);
        self.access_token(ac);

        Ok(())
    }

    /// Request an new AccessToken using the refresh token.
    pub fn request_refresh_token(&mut self) -> std::result::Result<(), OAuthError> {
        // The request URL.
        let u = match self.get(OAuthParam::AccessTokenURL) {
            Some(t) => t,
            None => return OAuth::error_from::<()>(OAuthParam::AccessTokenURL),
        };

        let access_code = match self.get(OAuthParam::AccessCode) {
            Some(t) => t,
            None => return OAuth::error_from::<()>(OAuthParam::AccessCode),
        };

        // The request body.
        let uri = match self.encoded_refresh_token_uri() {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        let cur_ac = self.access_token.clone();
        let current_ac = match cur_ac {
            Some(t) => t,
            None => return OAuth::error_from::<()>(OAuthParam::AccessToken),
        };

        let ac_mut = current_ac.into_inner();

        let mut response = self.request(u, access_code, uri.as_str());
        let mut ac: AccessToken = AccessToken::from(&mut response);
        match &ac_mut.get_refresh_token() {
            Some(t) => ac.set_refresh_token(t.to_owned().as_str()),
            None => {},
        };

        self.access_token(ac);
        Ok(())
    }

    pub fn encoded_refresh_token_uri(&self) -> std::result::Result<String, OAuthError> {
        self.encode_form_uri(OAuthParam::RefreshTokenURL)
    }

    pub fn encoded_access_token_uri(&self) -> std::result::Result<String, OAuthError> {
        self.encode_form_uri(OAuthParam::AccessTokenURL)
    }

    pub fn encoded_sign_in_url(&self) -> std::result::Result<String, OAuthError> {
        self.encode_url(OAuthParam::SignInUrl)
    }

    fn encode_url(&self, parameter: OAuthParam) -> std::result::Result<String, OAuthError> {
        if let Some(url) = self.get(parameter) {
            let mut url = url.to_string();
            if !url.ends_with('?') {
                url.push('?');
            }

            match self.encode_form_uri(parameter) {
                Ok(t) => url.push_str(t.as_str()),
                Err(e) => return Err(e),
            }

            return Ok(url);
        }

        OAuth::error_from(parameter)
    }

    fn set_encode_pair(
        &self,
        param: OAuthParam,
        encoder: &mut url::form_urlencoded::Serializer<String>,
    ) -> std::result::Result<(), OAuthError> {
        match self.get(param) {
            Some(t) => {
                let s = param.as_alias().to_string();
                encoder.append_pair(s.to_lowercase().as_str(), t);
                Ok(())
            },
            None => OAuth::error_from::<()>(param),
        }
    }

    pub fn get_refresh_token(&self) -> std::result::Result<String, OAuthError> {
        match self.get_access_token() {
            Some(t) => {
                let ac = t.try_borrow_mut();
                if let Ok(rt) = ac {
                    let token = rt.clone();
                    let refresh = token.get_refresh_token();
                    if let Some(t) = refresh {
                        return Ok(t);
                    } else {
                        return OAuth::error_from::<String>(OAuthParam::RefreshToken);
                    }
                } else {
                    return OAuth::error_from::<String>(OAuthParam::AccessToken);
                }
            },
            None => OAuth::error_from::<String>(OAuthParam::RefreshToken),
        }
    }

    pub fn encode_form_uri(
        &self,
        parameter: OAuthParam,
    ) -> std::result::Result<String, OAuthError> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());

        // self.set_encode_pair(OAuthParam::ClientId, &mut encoder)?;
        self.set_encode_pair(OAuthParam::ClientId, &mut encoder)?;
        self.set_encode_pair(OAuthParam::RedirectUri, &mut encoder)?;
        // Optional parameters. Errors are ignored.
        let _ = self.set_encode_pair(OAuthParam::ClientSecret, &mut encoder);
        let _ = self.set_encode_pair(OAuthParam::AppIdResource, &mut encoder);

        match parameter {
            OAuthParam::SignInUrl => {
                if self.contains(OAuthParam::ResponseMode) {
                    self.set_encode_pair(OAuthParam::ResponseMode, &mut encoder)?;
                }

                if !self.scopes.is_empty() {
                    encoder.append_pair("scope", self.get_scopes(" ").as_str());
                }

                if self.contains(OAuthParam::ResponseType) {
                    self.set_encode_pair(OAuthParam::ResponseType, &mut encoder)?;
                } else {
                    encoder.append_pair("response_type", "code");
                }
            },
            OAuthParam::AccessTokenURL => {
                self.set_encode_pair(OAuthParam::AccessCode, &mut encoder)?;
                encoder.append_pair("grant_type", "authorization_code");
            },
            OAuthParam::RefreshTokenURL => {
                match self.get_access_token() {
                    Some(t) => {
                        let ac = t.try_borrow_mut();
                        if let Ok(rt) = ac {
                            let token = rt.clone();
                            let refresh = token.get_refresh_token();
                            if let Some(t) = refresh {
                                encoder.append_pair("refresh_token", &t);
                            } else {
                                return OAuth::error_from::<String>(OAuthParam::RefreshToken);
                            }
                        } else {
                            return OAuth::error_from::<String>(OAuthParam::AccessToken);
                        }
                    },
                    None => return OAuth::error_from::<String>(OAuthParam::RefreshToken),
                }

                if self.contains(OAuthParam::GrantType) {
                    self.set_encode_pair(OAuthParam::GrantType, &mut encoder)?;
                } else {
                    encoder.append_pair("grant_type", "refresh_token");
                }
            },
            _ => {},
        }

        Ok(encoder.finish())
    }
}

// OAuth impl for error handling.
impl OAuth {
    fn error_from<T>(parameter: OAuthParam) -> std::result::Result<T, OAuthError> {
        match parameter {
            OAuthParam::SignInUrl
            | OAuthParam::AccessTokenURL
            | OAuthParam::RefreshTokenURL
            => Err(OAuthError::error_kind(
                ErrorKind::NotFound,
                format!(
                    "MISSING, INVALID: {:?}\nYou can use default methods for these: common_accounts(), organization_accounts(), or tenant_accounts().",
                    &parameter)
                    .as_str(),
            )),
            OAuthParam::AccessToken | OAuthParam::CODE => Err(OAuthError::error_kind(
                ErrorKind::NotFound,
                "MISSING, INVALID, OR MALFORMED PARAMETER: AccessToken",
            )),
            _ => Err(OAuthError::error_kind(
                ErrorKind::NotFound,
                format!("MISSING, INVALID: {:?}", &parameter).as_str(),
            )),
        }
    }
}

impl OAuth {
    pub fn browser_sign_in(&mut self) -> Option<JoinHandle<()>> {
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
}
