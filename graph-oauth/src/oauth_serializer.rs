use std::collections::btree_map::{BTreeMap, Entry};
use std::collections::{BTreeSet, HashMap};
use std::default::Default;
use std::fmt;
use std::fmt::Display;

use url::form_urlencoded::Serializer;

use graph_error::{AuthorizationFailure, IdentityResult};

use crate::identity::{AsQuery, Prompt, ResponseType};
use crate::strum::IntoEnumIterator;

/// Fields that represent common OAuth credentials.
#[derive(
    Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize, EnumIter,
)]
pub enum OAuthParameter {
    ClientId,
    ClientSecret,
    RedirectUri,
    AuthorizationCode,
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
    Scope,
    LoginHint,
    ClientAssertion,
    ClientAssertionType,
    CodeVerifier,
    CodeChallenge,
    CodeChallengeMethod,
    AdminConsent,
    Username,
    Password,
    DeviceCode,
}

impl OAuthParameter {
    pub fn alias(self) -> &'static str {
        match self {
            OAuthParameter::ClientId => "client_id",
            OAuthParameter::ClientSecret => "client_secret",
            OAuthParameter::RedirectUri => "redirect_uri",
            OAuthParameter::AuthorizationCode => "code",
            OAuthParameter::AccessToken => "access_token",
            OAuthParameter::RefreshToken => "refresh_token",
            OAuthParameter::ResponseMode => "response_mode",
            OAuthParameter::ResponseType => "response_type",
            OAuthParameter::State => "state",
            OAuthParameter::SessionState => "session_state",
            OAuthParameter::GrantType => "grant_type",
            OAuthParameter::Nonce => "nonce",
            OAuthParameter::Prompt => "prompt",
            OAuthParameter::IdToken => "id_token",
            OAuthParameter::Resource => "resource",
            OAuthParameter::DomainHint => "domain_hint",
            OAuthParameter::Scope => "scope",
            OAuthParameter::LoginHint => "login_hint",
            OAuthParameter::ClientAssertion => "client_assertion",
            OAuthParameter::ClientAssertionType => "client_assertion_type",
            OAuthParameter::CodeVerifier => "code_verifier",
            OAuthParameter::CodeChallenge => "code_challenge",
            OAuthParameter::CodeChallengeMethod => "code_challenge_method",
            OAuthParameter::AdminConsent => "admin_consent",
            OAuthParameter::Username => "username",
            OAuthParameter::Password => "password",
            OAuthParameter::DeviceCode => "device_code",
        }
    }

    fn is_debug_redacted(&self) -> bool {
        matches!(
            self,
            OAuthParameter::ClientId
                | OAuthParameter::ClientSecret
                | OAuthParameter::AccessToken
                | OAuthParameter::RefreshToken
                | OAuthParameter::IdToken
                | OAuthParameter::CodeVerifier
                | OAuthParameter::CodeChallenge
                | OAuthParameter::Password
        )
    }
}

impl Display for OAuthParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.alias().to_string())
    }
}

impl AsRef<str> for OAuthParameter {
    fn as_ref(&self) -> &'static str {
        self.alias()
    }
}

/// Serializer for query/x-www-form-urlencoded OAuth requests.
///
/// OAuth Serializer for query/form serialization that supports the OAuth 2.0 and OpenID
/// Connect protocols on Microsoft identity platform.
///
/// # Example
/// ```
/// use graph_oauth::extensions::OAuthSerializer;
/// let oauth = OAuthSerializer::new();
/// ```
#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct OAuthSerializer {
    scopes: BTreeSet<String>,
    parameters: BTreeMap<String, String>,
    log_pii: bool,
}

impl OAuthSerializer {
    /// Create a new OAuth instance.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::extensions::OAuthSerializer;
    ///
    /// let mut oauth = OAuthSerializer::new();
    /// ```
    pub fn new() -> OAuthSerializer {
        OAuthSerializer {
            scopes: BTreeSet::new(),
            parameters: BTreeMap::new(),
            log_pii: false,
        }
    }

    /// Insert oauth credentials using the OAuthParameter enum.
    /// This method is used internally for each of the setter methods.
    /// Callers can optionally use this method to set credentials instead
    /// of the individual setter methods.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # use graph_oauth::extensions::OAuthParameter;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.insert(OAuthParameter::AuthorizationCode, "code");
    /// assert!(oauth.contains(OAuthParameter::AuthorizationCode));
    /// println!("{:#?}", oauth.get(OAuthParameter::AuthorizationCode));
    /// ```
    pub fn insert<V: ToString>(&mut self, oac: OAuthParameter, value: V) -> &mut OAuthSerializer {
        self.parameters.insert(oac.to_string(), value.to_string());
        self
    }

    /// Insert and OAuth credential using the entry trait and
    /// returning the credential. This internally calls
    /// `entry.(OAuthParameter).or_insret_with(value)`.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # use graph_oauth::extensions::OAuthParameter;
    /// # let mut oauth = OAuthSerializer::new();
    /// let entry = oauth.entry_with(OAuthParameter::AuthorizationCode, "code");
    /// assert_eq!(entry, "code")
    /// ```
    pub fn entry_with<V: ToString>(&mut self, oac: OAuthParameter, value: V) -> &mut String {
        self.parameters
            .entry(oac.alias().to_string())
            .or_insert_with(|| value.to_string())
    }

    /// A view into a single entry in a map, which may either be vacant or occupied.
    ///
    /// This `enum` is constructed from the [`entry`] method on [`BTreeMap`].
    ///
    /// [`entry`]: BTreeMap::entry
    pub fn entry<V: ToString>(&mut self, oac: OAuthParameter) -> Entry<String, String> {
        self.parameters.entry(oac.alias().to_string())
    }

    /// Get a previously set credential.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # use graph_oauth::extensions::OAuthParameter;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.authorization_code("code");
    /// let code = oauth.get(OAuthParameter::AuthorizationCode);
    /// assert_eq!("code", code.unwrap().as_str());
    /// ```
    pub fn get(&self, oac: OAuthParameter) -> Option<String> {
        self.parameters.get(oac.alias()).cloned()
    }

    /// Check if an OAuth credential has already been set.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # use graph_oauth::extensions::OAuthParameter;
    /// # let mut oauth = OAuthSerializer::new();
    /// println!("{:#?}", oauth.contains(OAuthParameter::Nonce));
    /// ```
    pub fn contains(&self, t: OAuthParameter) -> bool {
        if t == OAuthParameter::Scope {
            return !self.scopes.is_empty();
        }
        self.parameters.contains_key(t.alias())
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.parameters.contains_key(key)
    }

    /// Remove a field from OAuth.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # use graph_oauth::extensions::OAuthParameter;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.client_id("client_id");
    ///
    /// assert_eq!(oauth.contains(OAuthParameter::ClientId), true);
    /// oauth.remove(OAuthParameter::ClientId);
    ///
    /// assert_eq!(oauth.contains(OAuthParameter::ClientId), false);
    /// ```
    pub fn remove(&mut self, oac: OAuthParameter) -> &mut OAuthSerializer {
        self.parameters.remove(oac.alias());
        self
    }

    /// Set the client id for an OAuth request.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # use graph_oauth::extensions::OAuthParameter;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.client_id("client_id");
    /// ```
    pub fn client_id(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::ClientId, value)
    }

    /// Set the state for an OAuth request.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # use graph_oauth::extensions::OAuthParameter;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.state("1234");
    /// ```
    pub fn state(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::State, value)
    }

    /// Set the client secret for an OAuth request.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.client_secret("client_secret");
    /// ```
    pub fn client_secret(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::ClientSecret, value)
    }

    /// Set the redirect url of a request
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.redirect_uri("https://localhost:8888/redirect");
    /// ```
    pub fn redirect_uri(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::RedirectUri, value)
    }

    /// Set the access code.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut serializer = OAuthSerializer::new();
    /// serializer.authorization_code("LDSF[POK43");
    /// ```
    pub fn authorization_code(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::AuthorizationCode, value)
    }

    /// Set the response mode.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut serializer = OAuthSerializer::new();
    /// serializer.response_mode("query");
    /// ```
    pub fn response_mode(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::ResponseMode, value)
    }

    /// Set the response type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.response_type("token");
    /// ```
    pub fn response_type<T: ToString>(&mut self, value: T) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::ResponseType, value)
    }

    pub fn response_types(
        &mut self,
        value: std::collections::btree_set::Iter<'_, ResponseType>,
    ) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::ResponseType, value.as_query())
    }

    /// Set the nonce.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    ///
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.nonce("1234");
    /// ```
    pub fn nonce(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::Nonce, value)
    }

    /// Set the prompt for open id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    ///
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.prompt("login");
    /// ```
    pub fn prompt(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::Prompt, value)
    }

    pub fn prompts(&mut self, value: &[Prompt]) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::Prompt, value.to_vec().as_query())
    }

    /// Set the session state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.session_state("session-state");
    /// ```
    pub fn session_state(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::SessionState, value)
    }

    /// Set the grant_type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.grant_type("token");
    /// ```
    pub fn grant_type(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::GrantType, value)
    }

    /// Set the resource.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.resource("resource");
    /// ```
    pub fn resource(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::Resource, value)
    }

    /// Set the code verifier.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.code_verifier("code_verifier");
    /// ```
    pub fn code_verifier(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::CodeVerifier, value)
    }

    /// Set the domain hint.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.domain_hint("domain_hint");
    /// ```
    pub fn domain_hint(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::DomainHint, value)
    }

    /// Set the code challenge.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.code_challenge("code_challenge");
    /// ```
    pub fn code_challenge(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::CodeChallenge, value)
    }

    /// Set the code challenge method.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.code_challenge_method("code_challenge_method");
    /// ```
    pub fn code_challenge_method(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::CodeChallengeMethod, value)
    }

    /// Set the login hint.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.login_hint("login_hint");
    /// ```
    pub fn login_hint(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::LoginHint, value)
    }

    /// Set the client assertion.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.client_assertion("client_assertion");
    /// ```
    pub fn client_assertion(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::ClientAssertion, value)
    }

    /// Set the client assertion type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.client_assertion_type("client_assertion_type");
    /// ```
    pub fn client_assertion_type(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::ClientAssertionType, value)
    }

    /// Set the redirect uri that user will be redirected to after logging out.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::{OAuthSerializer, OAuthParameter};
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.username("user");
    /// assert!(oauth.contains(OAuthParameter::Username))
    /// ```
    pub fn username(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::Username, value)
    }

    /// Set the redirect uri that user will be redirected to after logging out.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::{OAuthSerializer, OAuthParameter};
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.password("user");
    /// assert!(oauth.contains(OAuthParameter::Password))
    /// ```
    pub fn password(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::Password, value)
    }

    pub fn refresh_token(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::RefreshToken, value)
    }

    /// Set the device code for the device authorization flow.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::{OAuthSerializer, OAuthParameter};
    /// # let mut oauth = OAuthSerializer::new();
    /// oauth.device_code("device_code");
    /// assert!(oauth.contains(OAuthParameter::DeviceCode))
    /// ```
    pub fn device_code(&mut self, value: &str) -> &mut OAuthSerializer {
        self.insert(OAuthParameter::DeviceCode, value)
    }

    /// Add a scope' for the OAuth URL.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    ///
    /// oauth.add_scope("Sites.Read")
    ///     .add_scope("Sites.ReadWrite")
    ///     .add_scope("Sites.ReadWrite.All");
    /// assert_eq!(oauth.join_scopes(" "), "Sites.Read Sites.ReadWrite Sites.ReadWrite.All");
    /// ```
    pub fn add_scope<T: ToString>(&mut self, scope: T) -> &mut OAuthSerializer {
        self.scopes.insert(scope.to_string());
        self
    }

    /// Get the scopes.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// let mut oauth = OAuthSerializer::new();
    /// oauth.add_scope("Files.Read");
    /// oauth.add_scope("Files.ReadWrite");
    ///
    /// let scopes = oauth.get_scopes();
    /// assert!(scopes.contains("Files.Read"));
    /// assert!(scopes.contains("Files.ReadWrite"));
    /// ```
    pub fn get_scopes(&self) -> &BTreeSet<String> {
        &self.scopes
    }

    /// Join scopes.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    ///
    /// // the scopes take a separator just like Vec join.
    ///  let s = oauth.join_scopes(" ");
    /// println!("{:#?}", s);
    /// ```
    pub fn join_scopes(&self, sep: &str) -> String {
        self.scopes
            .iter()
            .map(|s| &**s)
            .collect::<Vec<&str>>()
            .join(sep)
    }

    /// Set scope. Overriding all previous scope values.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # use std::collections::HashSet;
    /// # let mut oauth = OAuthSerializer::new();
    ///
    /// let scopes = vec!["Files.Read", "Files.ReadWrite"];
    /// oauth.extend_scopes(&scopes);
    ///
    /// assert_eq!(oauth.join_scopes(" "), "Files.Read Files.ReadWrite");
    /// ```
    pub fn set_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, iter: I) -> &mut Self {
        self.scopes = iter.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Extend scopes.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # use std::collections::HashSet;
    /// # let mut oauth = OAuthSerializer::new();
    ///
    /// let scopes = vec!["Files.Read", "Files.ReadWrite"];
    /// oauth.extend_scopes(&scopes);
    ///
    /// assert_eq!(oauth.join_scopes(" "), "Files.Read Files.ReadWrite");
    /// ```
    pub fn extend_scopes<T: ToString, I: IntoIterator<Item = T>>(&mut self, iter: I) -> &mut Self {
        self.scopes.extend(iter.into_iter().map(|s| s.to_string()));
        self
    }

    /// Check if OAuth contains a specific scope.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::OAuthSerializer;
    /// # let mut oauth = OAuthSerializer::new();
    ///
    /// oauth.add_scope("Files.Read");
    /// assert_eq!(oauth.contains_scope("Files.Read"), true);
    ///
    /// // Or using static scopes
    /// oauth.add_scope("File.ReadWrite");
    /// assert!(oauth.contains_scope("File.ReadWrite"));
    /// ```
    pub fn contains_scope<T: ToString>(&self, scope: T) -> bool {
        self.scopes.contains(&scope.to_string())
    }
}

impl OAuthSerializer {
    fn try_as_tuple(&self, oac: &OAuthParameter) -> IdentityResult<(String, String)> {
        if oac.eq(&OAuthParameter::Scope) {
            if self.scopes.is_empty() {
                return Err(AuthorizationFailure::required(oac));
            }
            Ok((oac.alias().to_owned(), self.join_scopes(" ")))
        } else {
            Ok((
                oac.alias().to_owned(),
                self.get(*oac).ok_or(AuthorizationFailure::required(oac))?,
            ))
        }
    }

    pub fn encode_query(
        &mut self,
        optional_fields: Vec<OAuthParameter>,
        required_fields: Vec<OAuthParameter>,
    ) -> IdentityResult<String> {
        let mut serializer = Serializer::new(String::new());
        for parameter in required_fields {
            if parameter.alias().eq("scope") {
                if self.scopes.is_empty() {
                    return AuthorizationFailure::result::<String>(parameter.alias());
                } else {
                    serializer.append_pair("scope", self.join_scopes(" ").as_str());
                }
            } else {
                let value = self
                    .get(parameter)
                    .ok_or(AuthorizationFailure::required(parameter))?;

                serializer.append_pair(parameter.alias(), value.as_str());
            }
        }

        for parameter in optional_fields {
            if parameter.alias().eq("scope") && !self.scopes.is_empty() {
                serializer.append_pair("scope", self.join_scopes(" ").as_str());
            } else if let Some(val) = self.get(parameter) {
                serializer.append_pair(parameter.alias(), val.as_str());
            }
        }

        Ok(serializer.finish())
    }

    pub fn as_credential_map(
        &mut self,
        optional_fields: Vec<OAuthParameter>,
        required_fields: Vec<OAuthParameter>,
    ) -> IdentityResult<HashMap<String, String>> {
        let mut required_map = required_fields
            .iter()
            .map(|oac| self.try_as_tuple(oac))
            .collect::<IdentityResult<HashMap<String, String>>>()?;

        let optional_map: HashMap<String, String> = optional_fields
            .iter()
            .flat_map(|oac| self.try_as_tuple(oac))
            .collect();

        required_map.extend(optional_map);
        Ok(required_map)
    }
}

/// Extend the OAuth credentials.
///
/// # Example
/// ```
/// # use graph_oauth::extensions::{OAuthSerializer, OAuthParameter};
/// # use std::collections::HashMap;
/// # let mut oauth = OAuthSerializer::new();
/// let mut map: HashMap<OAuthParameter, &str> = HashMap::new();
/// map.insert(OAuthParameter::ClientId, "client_id");
/// map.insert(OAuthParameter::ClientSecret, "client_secret");
///
/// oauth.extend(map);
/// # assert_eq!(oauth.get(OAuthParameter::ClientId), Some("client_id".to_string()));
/// # assert_eq!(oauth.get(OAuthParameter::ClientSecret), Some("client_secret".to_string()));
/// ```
impl<V: ToString> Extend<(OAuthParameter, V)> for OAuthSerializer {
    fn extend<I: IntoIterator<Item = (OAuthParameter, V)>>(&mut self, iter: I) {
        iter.into_iter().for_each(|entry| {
            self.insert(entry.0, entry.1);
        });
    }
}

impl fmt::Debug for OAuthSerializer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map_debug: BTreeMap<&str, &str> = BTreeMap::new();
        for (key, value) in self.parameters.iter() {
            if self.log_pii {
                map_debug.insert(key.as_str(), value.as_str());
            } else if let Some(oac) = OAuthParameter::iter()
                .find(|oac| oac.alias().eq(key.as_str()) && oac.is_debug_redacted())
            {
                map_debug.insert(oac.alias(), "[REDACTED]");
            } else {
                map_debug.insert(key.as_str(), value.as_str());
            }
        }

        f.debug_struct("OAuthSerializer")
            .field("credentials", &map_debug)
            .field("scopes", &self.scopes)
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn oauth_parameters_from_credential() {
        // Doesn't matter the flow here as this is for testing
        // that the credentials are entered/retrieved correctly.
        let mut oauth = OAuthSerializer::new();
        oauth
            .client_id("client_id")
            .client_secret("client_secret")
            .redirect_uri("https://example.com/redirect?")
            .authorization_code("ADSLFJL4L3")
            .response_mode("response_mode")
            .response_type("response_type")
            .state("state")
            .grant_type("grant_type")
            .nonce("nonce")
            .prompt("login")
            .session_state("session_state")
            .client_assertion("client_assertion")
            .client_assertion_type("client_assertion_type")
            .code_verifier("code_verifier")
            .login_hint("login_hint")
            .domain_hint("domain_hint")
            .resource("resource");

        OAuthParameter::iter().for_each(|credential| {
            if oauth.contains(credential) {
                match credential {
                    OAuthParameter::ClientId => {
                        assert_eq!(oauth.get(credential), Some("client_id".into()))
                    }
                    OAuthParameter::ClientSecret => {
                        assert_eq!(oauth.get(credential), Some("client_secret".into()))
                    }
                    OAuthParameter::RedirectUri => assert_eq!(
                        oauth.get(credential),
                        Some("https://example.com/redirect?".into())
                    ),
                    OAuthParameter::AuthorizationCode => {
                        assert_eq!(oauth.get(credential), Some("ADSLFJL4L3".into()))
                    }
                    OAuthParameter::ResponseMode => {
                        assert_eq!(oauth.get(credential), Some("response_mode".into()))
                    }
                    OAuthParameter::ResponseType => {
                        assert_eq!(oauth.get(credential), Some("response_type".into()))
                    }
                    OAuthParameter::State => {
                        assert_eq!(oauth.get(credential), Some("state".into()))
                    }
                    OAuthParameter::GrantType => {
                        assert_eq!(oauth.get(credential), Some("grant_type".into()))
                    }
                    OAuthParameter::Nonce => {
                        assert_eq!(oauth.get(credential), Some("nonce".into()))
                    }
                    OAuthParameter::Prompt => {
                        assert_eq!(oauth.get(credential), Some("login".into()))
                    }
                    OAuthParameter::SessionState => {
                        assert_eq!(oauth.get(credential), Some("session_state".into()))
                    }
                    OAuthParameter::ClientAssertion => {
                        assert_eq!(oauth.get(credential), Some("client_assertion".into()))
                    }
                    OAuthParameter::ClientAssertionType => {
                        assert_eq!(oauth.get(credential), Some("client_assertion_type".into()))
                    }
                    OAuthParameter::CodeVerifier => {
                        assert_eq!(oauth.get(credential), Some("code_verifier".into()))
                    }
                    OAuthParameter::Resource => {
                        assert_eq!(oauth.get(credential), Some("resource".into()))
                    }
                    _ => {}
                }
            }
        });
    }

    #[test]
    fn remove_credential() {
        // Doesn't matter the flow here as this is for testing
        // that the credentials are entered/retrieved correctly.
        let mut oauth = OAuthSerializer::new();
        oauth
            .client_id("bb301aaa-1201-4259-a230923fds32")
            .redirect_uri("http://localhost:8888/redirect")
            .client_secret("CLDIE3F")
            .authorization_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");
        assert!(oauth.get(OAuthParameter::ClientId).is_some());
        oauth.remove(OAuthParameter::ClientId);
        assert!(oauth.get(OAuthParameter::ClientId).is_none());
        oauth.client_id("client_id");
        assert!(oauth.get(OAuthParameter::ClientId).is_some());

        assert!(oauth.get(OAuthParameter::RedirectUri).is_some());
        oauth.remove(OAuthParameter::RedirectUri);
        assert!(oauth.get(OAuthParameter::RedirectUri).is_none());
    }

    #[test]
    fn setters() {
        // Doesn't matter the flow here as this is for testing
        // that the credentials are entered/retrieved correctly.
        let mut oauth = OAuthSerializer::new();
        oauth
            .client_id("client_id")
            .client_secret("client_secret")
            .redirect_uri("https://example.com/redirect")
            .authorization_code("access_code");

        let test_setter = |c: OAuthParameter, s: &str| {
            let result = oauth.get(c);
            assert!(result.is_some());
            assert!(result.is_some());
            assert_eq!(result.unwrap(), s);
        };

        test_setter(OAuthParameter::ClientId, "client_id");
        test_setter(OAuthParameter::ClientSecret, "client_secret");
        test_setter(OAuthParameter::RedirectUri, "https://example.com/redirect");
        test_setter(OAuthParameter::AuthorizationCode, "access_code");
    }
}