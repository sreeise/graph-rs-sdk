use crate::accesstoken::AccessToken;
use crate::grants::{ClientCredentialsGrant, ImplicitGrant};
use crate::oautherror::OAuthError;
use reqwest::{header, RequestBuilder};
use std::collections::btree_map::BTreeMap;
use std::collections::{hash_map::DefaultHasher, HashMap};
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
///
/// OAuthCredential is also represented by the enum Credential in order to
/// reference these fields without having to pass a String value.

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
    ResponseType,
    GrantType,
    Nonce,
    Scopes,
    PostLogoutRedirectURI,
    LogoutURL,
}

impl OAuthCredential {
    pub fn alias(&self) -> &'static str {
        match *self {
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
            OAuthCredential::GrantType => "grant_type",
            OAuthCredential::Nonce => "nonce",
            OAuthCredential::Scopes => "scope",
            OAuthCredential::LogoutURL => "logout_url",
            OAuthCredential::PostLogoutRedirectURI => "post_logout_redirect_uri",
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
    encoded: BTreeMap<String, String>,
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
            encoded: BTreeMap::new(),
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
        self.credentials.insert(oac.alias().to_string(), value);
        self
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

    /// Check if an OAuth credential has already been set.
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

    /// Check if an OAuth credential has already been set.
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

    /// Set the client secret for an OAuth request.
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
    pub fn authorize_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AuthorizeURL, value.into())
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
    pub fn access_token_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::AccessTokenURL, value.into())
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
    pub fn refresh_token_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::RefreshTokenURL, value.into())
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
    pub fn redirect_url(&mut self, value: &str) -> &mut OAuth {
        self.insert(OAuthCredential::RedirectURI, value.into())
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

    /// Set the response mode.
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

    /// Set the response mode.
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

    /// Set the response mode.
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
    pub fn browser_sign_in(&mut self) -> Result<Output, OAuthError> {
        match self.get_or_else(OAuthCredential::AuthorizeURL) {
            Ok(t) => self.browser_sign_in_url(t),
            Err(e) => Err(e),
        }
    }

    pub fn browser_sign_in_url(&self, url: String) -> std::result::Result<Output, OAuthError> {
        let url = url.as_str();
        webbrowser::open(url).map_err(OAuthError::from)
    }

    pub fn v1_logout(&mut self) -> OAuthReq<Output> {
        let mut url = self.get_or_else(OAuthCredential::LogoutURL)?;
        if !url.ends_with('?') {
            url.push('?');
        }
        let client_id = self.get_or_else(OAuthCredential::ClientId)?;
        url.push_str("&client_id=");
        url.push_str(client_id.as_str());
        url.push_str("&redirect_uri=");
        if let Some(redirect) = self.get(OAuthCredential::PostLogoutRedirectURI) {
            url.push_str(redirect.as_str());
        } else if let Some(redirect) = self.get(OAuthCredential::RedirectURI) {
            url.push_str(redirect.as_str());
        }

        webbrowser::open(url.as_str()).map_err(OAuthError::from)
    }

    pub fn v2_logout(&self) -> OAuthReq<Output> {
        let mut url = self.get_or_else(OAuthCredential::LogoutURL)?;
        if let Some(redirect) = self.get(OAuthCredential::PostLogoutRedirectURI) {
            url.push_str(redirect.as_str());
        } else {
            let redirect_uri = self.get_or_else(OAuthCredential::RedirectURI)?;
            url.push_str(redirect_uri.as_str());
        }
        webbrowser::open(url.as_str()).map_err(OAuthError::from)
    }
}

impl OAuth {
    pub fn get_refresh_token(&self) -> OAuthReq<String> {
        match self.get_access_token() {
            Some(token) => match token.get_refresh_token() {
                Some(t) => return Ok(t),
                None => return OAuth::error_from::<String>(OAuthCredential::RefreshToken),
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
            .filter(|oac| self.contains_key(oac.alias()))
            .for_each(|oac| {
                if let Some(val) = self.get(oac.clone()) {
                    encoder.append_pair(oac.alias(), val.as_str());
                }
            });
    }

    fn client(&self, url: &str, code_body: &str, content_type: &str) -> RequestBuilder {
        let client = reqwest::Client::builder().build().unwrap();
        let access_code = self.get(OAuthCredential::AccessCode).unwrap();
        let rb = client
            .post(url)
            .header(header::AUTHORIZATION, access_code.as_str())
            .header(header::CONTENT_TYPE, content_type)
            .body(code_body.to_string());
        rb
    }

    pub fn encoded_authorization_url(&mut self) -> OAuthReq<String> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        self.insert(OAuthCredential::ResponseType, "code".into());
        self.insert(OAuthCredential::ResponseMode, "query".into());
        self.form_encode_credentials(
            vec![
                OAuthCredential::ClientId,
                OAuthCredential::ClientSecret,
                OAuthCredential::RedirectURI,
                OAuthCredential::State,
                OAuthCredential::ResponseMode,
                OAuthCredential::ResponseType,
            ],
            &mut encoder,
        );

        if !self.scopes.is_empty() {
            encoder.append_pair("scope", self.scopes.join(" ").as_str());
        }
        let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
        if !url.ends_with('?') {
            url.push('?');
        }
        url.push_str(encoder.finish().as_str());
        return Ok(url);
    }

    pub fn encoded_access_token_uri(&mut self) -> OAuthReq<String> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        self.insert(OAuthCredential::ResponseType, "token".into());
        self.insert(OAuthCredential::GrantType, "authorization_code".into());
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

        let body = encoder.finish();
        return Ok(body);
    }

    pub fn encoded_refresh_token_uri(&mut self) -> OAuthReq<String> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        self.insert(OAuthCredential::GrantType, "refresh_token".into());
        let refresh_token = self.get_refresh_token().unwrap();
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

        let body = encoder.finish();
        return Ok(body);
    }
}

impl ClientCredentialsGrant for OAuth {
    fn request_authorization(&mut self) -> OAuthReq<Output> {
        // https://tools.ietf.org/html/rfc6749#section-4.1.1
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        self.insert(OAuthCredential::ResponseType, "code".into());
        self.insert(OAuthCredential::ResponseMode, "query".into());
        self.form_encode_credentials(
            vec![
                OAuthCredential::ClientId,
                OAuthCredential::ClientSecret,
                OAuthCredential::RedirectURI,
                OAuthCredential::State,
                OAuthCredential::ResponseMode,
                OAuthCredential::ResponseType,
            ],
            &mut encoder,
        );

        if !self.scopes.is_empty() {
            encoder.append_pair("scope", self.scopes.join(" ").as_str());
        }

        let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
        if !url.ends_with('?') {
            url.push('?');
        }

        url.push_str(encoder.finish().as_str());
        webbrowser::open(url.as_str()).map_err(OAuthError::from)
    }

    fn request_access_token(&mut self) -> OAuthReq<()> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());

        self.insert(OAuthCredential::ResponseType, "token".into());
        self.insert(OAuthCredential::GrantType, "authorization_code".into());
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

        let body = encoder.finish();
        let url = self.get_or_else(OAuthCredential::AccessTokenURL).unwrap();

        let builder = self.client(
            url.as_str(),
            body.as_str(),
            "application/x-www-form-urlencoded",
        );
        let access_token = AccessToken::transform(builder)?;
        self.access_token(access_token);
        Ok(())
    }

    fn request_refresh_token(&mut self) -> OAuthReq<()> {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        self.insert(OAuthCredential::GrantType, "refresh_token".into());
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

        let url = self.get_or_else(OAuthCredential::RefreshTokenURL)?;
        let body = encoder.finish();

        let builder = self.client(
            url.as_str(),
            body.as_str(),
            "application/x-www-form-urlencoded",
        );
        let access_token = AccessToken::transform(builder)?;
        self.access_token(access_token);
        Ok(())
    }
}

impl ImplicitGrant for OAuth {
    fn request_access_token(&mut self) -> OAuthReq<Output> {
        // https://tools.ietf.org/html/rfc6749#section-4.2
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        let mut map = HashMap::new();
        let client_id = self.get_or_else(OAuthCredential::ClientId)?;
        let redirect_uri = self.get_or_else(OAuthCredential::RedirectURI)?;

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

        let mut url = self.get_or_else(OAuthCredential::AuthorizeURL)?;
        if !url.ends_with('?') {
            url.push('?');
        }

        let body = encoder.finish();
        url.push_str(body.as_str());

        self.browser_sign_in_url(url)
    }
}
