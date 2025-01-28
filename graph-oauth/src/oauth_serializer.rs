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
pub enum AuthParameter {
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

impl AuthParameter {
    pub fn alias(self) -> &'static str {
        match self {
            AuthParameter::ClientId => "client_id",
            AuthParameter::ClientSecret => "client_secret",
            AuthParameter::RedirectUri => "redirect_uri",
            AuthParameter::AuthorizationCode => "code",
            AuthParameter::AccessToken => "access_token",
            AuthParameter::RefreshToken => "refresh_token",
            AuthParameter::ResponseMode => "response_mode",
            AuthParameter::ResponseType => "response_type",
            AuthParameter::State => "state",
            AuthParameter::SessionState => "session_state",
            AuthParameter::GrantType => "grant_type",
            AuthParameter::Nonce => "nonce",
            AuthParameter::Prompt => "prompt",
            AuthParameter::IdToken => "id_token",
            AuthParameter::Resource => "resource",
            AuthParameter::DomainHint => "domain_hint",
            AuthParameter::Scope => "scope",
            AuthParameter::LoginHint => "login_hint",
            AuthParameter::ClientAssertion => "client_assertion",
            AuthParameter::ClientAssertionType => "client_assertion_type",
            AuthParameter::CodeVerifier => "code_verifier",
            AuthParameter::CodeChallenge => "code_challenge",
            AuthParameter::CodeChallengeMethod => "code_challenge_method",
            AuthParameter::AdminConsent => "admin_consent",
            AuthParameter::Username => "username",
            AuthParameter::Password => "password",
            AuthParameter::DeviceCode => "device_code",
        }
    }

    fn is_debug_redacted(&self) -> bool {
        matches!(
            self,
            AuthParameter::ClientId
                | AuthParameter::ClientSecret
                | AuthParameter::AccessToken
                | AuthParameter::RefreshToken
                | AuthParameter::IdToken
                | AuthParameter::CodeVerifier
                | AuthParameter::CodeChallenge
                | AuthParameter::Password
        )
    }
}

impl Display for AuthParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.alias())
    }
}

impl AsRef<str> for AuthParameter {
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
/// use graph_oauth::extensions::AuthSerializer;
/// let oauth = AuthSerializer::new();
/// ```
#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthSerializer {
    scopes: BTreeSet<String>,
    parameters: BTreeMap<String, String>,
    log_pii: bool,
}

impl AuthSerializer {
    /// Create a new OAuth instance.
    ///
    /// # Example
    /// ```
    /// use graph_oauth::extensions::AuthSerializer;
    ///
    /// let mut oauth = AuthSerializer::new();
    /// ```
    pub fn new() -> AuthSerializer {
        AuthSerializer {
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
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # use graph_oauth::extensions::AuthParameter;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.insert(AuthParameter::AuthorizationCode, "code");
    /// assert!(oauth.contains(AuthParameter::AuthorizationCode));
    /// println!("{:#?}", oauth.get(AuthParameter::AuthorizationCode));
    /// ```
    pub fn insert<V: ToString>(&mut self, oac: AuthParameter, value: V) -> &mut AuthSerializer {
        self.parameters.insert(oac.to_string(), value.to_string());
        self
    }

    /// Insert and OAuth credential using the entry trait and
    /// returning the credential. This internally calls
    /// `entry.(OAuthParameter).or_insret_with(value)`.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # use graph_oauth::extensions::AuthParameter;
    /// # let mut oauth = AuthSerializer::new();
    /// let entry = oauth.entry_with(AuthParameter::AuthorizationCode, "code");
    /// assert_eq!(entry, "code")
    /// ```
    pub fn entry_with<V: ToString>(&mut self, oac: AuthParameter, value: V) -> &mut String {
        self.parameters
            .entry(oac.alias().to_string())
            .or_insert_with(|| value.to_string())
    }

    /// A view into a single entry in a map, which may either be vacant or occupied.
    ///
    /// This `enum` is constructed from the [`entry`] method on [`BTreeMap`].
    ///
    /// [`entry`]: BTreeMap::entry
    pub fn entry<V: ToString>(&mut self, oac: AuthParameter) -> Entry<String, String> {
        self.parameters.entry(oac.alias().to_string())
    }

    /// Get a previously set credential.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # use graph_oauth::extensions::AuthParameter;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.authorization_code("code");
    /// let code = oauth.get(AuthParameter::AuthorizationCode);
    /// assert_eq!("code", code.unwrap().as_str());
    /// ```
    pub fn get(&self, oac: AuthParameter) -> Option<String> {
        self.parameters.get(oac.alias()).cloned()
    }

    /// Check if an OAuth credential has already been set.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # use graph_oauth::extensions::AuthParameter;
    /// # let mut oauth = AuthSerializer::new();
    /// println!("{:#?}", oauth.contains(AuthParameter::Nonce));
    /// ```
    pub fn contains(&self, t: AuthParameter) -> bool {
        if t == AuthParameter::Scope {
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
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # use graph_oauth::extensions::AuthParameter;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.client_id("client_id");
    ///
    /// assert_eq!(oauth.contains(AuthParameter::ClientId), true);
    /// oauth.remove(AuthParameter::ClientId);
    ///
    /// assert_eq!(oauth.contains(AuthParameter::ClientId), false);
    /// ```
    pub fn remove(&mut self, oac: AuthParameter) -> &mut AuthSerializer {
        self.parameters.remove(oac.alias());
        self
    }

    /// Set the client id for an OAuth request.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # use graph_oauth::extensions::AuthParameter;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.client_id("client_id");
    /// ```
    pub fn client_id(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::ClientId, value)
    }

    /// Set the state for an OAuth request.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # use graph_oauth::extensions::AuthParameter;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.state("1234");
    /// ```
    pub fn state(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::State, value)
    }

    /// Set the client secret for an OAuth request.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.client_secret("client_secret");
    /// ```
    pub fn client_secret(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::ClientSecret, value)
    }

    /// Set the redirect url of a request
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.redirect_uri("https://localhost:8888/redirect");
    /// ```
    pub fn redirect_uri(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::RedirectUri, value)
    }

    /// Set the access code.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut serializer = AuthSerializer::new();
    /// serializer.authorization_code("LDSF[POK43");
    /// ```
    pub fn authorization_code(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::AuthorizationCode, value)
    }

    /// Set the response mode.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut serializer = AuthSerializer::new();
    /// serializer.response_mode("query");
    /// ```
    pub fn response_mode(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::ResponseMode, value)
    }

    /// Set the response type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.response_type("token");
    /// ```
    pub fn response_type<T: ToString>(&mut self, value: T) -> &mut AuthSerializer {
        self.insert(AuthParameter::ResponseType, value)
    }

    pub fn response_types(
        &mut self,
        value: std::collections::btree_set::Iter<'_, ResponseType>,
    ) -> &mut AuthSerializer {
        self.insert(AuthParameter::ResponseType, value.as_query())
    }

    /// Set the nonce.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    ///
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.nonce("1234");
    /// ```
    pub fn nonce(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::Nonce, value)
    }

    /// Set the prompt for open id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    ///
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.prompt("login");
    /// ```
    pub fn prompt(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::Prompt, value)
    }

    pub fn prompts(&mut self, value: &[Prompt]) -> &mut AuthSerializer {
        self.insert(AuthParameter::Prompt, value.to_vec().as_query())
    }

    /// Set the session state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.session_state("session-state");
    /// ```
    pub fn session_state(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::SessionState, value)
    }

    /// Set the grant_type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.grant_type("token");
    /// ```
    pub fn grant_type(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::GrantType, value)
    }

    /// Set the resource.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.resource("resource");
    /// ```
    pub fn resource(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::Resource, value)
    }

    /// Set the code verifier.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.code_verifier("code_verifier");
    /// ```
    pub fn code_verifier(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::CodeVerifier, value)
    }

    /// Set the domain hint.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.domain_hint("domain_hint");
    /// ```
    pub fn domain_hint(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::DomainHint, value)
    }

    /// Set the code challenge.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.code_challenge("code_challenge");
    /// ```
    pub fn code_challenge(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::CodeChallenge, value)
    }

    /// Set the code challenge method.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.code_challenge_method("code_challenge_method");
    /// ```
    pub fn code_challenge_method(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::CodeChallengeMethod, value)
    }

    /// Set the login hint.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.login_hint("login_hint");
    /// ```
    pub fn login_hint(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::LoginHint, value)
    }

    /// Set the client assertion.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.client_assertion("client_assertion");
    /// ```
    pub fn client_assertion(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::ClientAssertion, value)
    }

    /// Set the client assertion type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.client_assertion_type("client_assertion_type");
    /// ```
    pub fn client_assertion_type(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::ClientAssertionType, value)
    }

    /// Set the client username
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::{AuthSerializer, AuthParameter};
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.username("user");
    /// assert!(oauth.contains(AuthParameter::Username))
    /// ```
    pub fn username(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::Username, value)
    }

    /// Set the client password
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::{AuthSerializer, AuthParameter};
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.password("user");
    /// assert!(oauth.contains(AuthParameter::Password))
    /// ```
    pub fn password(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::Password, value)
    }

    pub fn refresh_token(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::RefreshToken, value)
    }

    /// Set the device code for the device authorization flow.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::{AuthSerializer, AuthParameter};
    /// # let mut oauth = AuthSerializer::new();
    /// oauth.device_code("device_code");
    /// assert!(oauth.contains(AuthParameter::DeviceCode))
    /// ```
    pub fn device_code(&mut self, value: &str) -> &mut AuthSerializer {
        self.insert(AuthParameter::DeviceCode, value)
    }

    /// Add a scope' for the OAuth URL.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
    ///
    /// oauth.add_scope("Sites.Read")
    ///     .add_scope("Sites.ReadWrite")
    ///     .add_scope("Sites.ReadWrite.All");
    /// assert_eq!(oauth.join_scopes(" "), "Sites.Read Sites.ReadWrite Sites.ReadWrite.All");
    /// ```
    pub fn add_scope<T: ToString>(&mut self, scope: T) -> &mut AuthSerializer {
        self.scopes.insert(scope.to_string());
        self
    }

    /// Get the scopes.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::extensions::AuthSerializer;
    /// let mut oauth = AuthSerializer::new();
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
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
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
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # use std::collections::HashSet;
    /// # let mut oauth = AuthSerializer::new();
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
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # use std::collections::HashSet;
    /// # let mut oauth = AuthSerializer::new();
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
    /// # use graph_oauth::extensions::AuthSerializer;
    /// # let mut oauth = AuthSerializer::new();
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

impl AuthSerializer {
    fn try_as_tuple(&self, oac: &AuthParameter) -> IdentityResult<(String, String)> {
        if oac.eq(&AuthParameter::Scope) {
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
        optional_fields: Vec<AuthParameter>,
        required_fields: Vec<AuthParameter>,
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
        optional_fields: Vec<AuthParameter>,
        required_fields: Vec<AuthParameter>,
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
/// # use graph_oauth::extensions::{AuthSerializer, AuthParameter};
/// # use std::collections::HashMap;
/// # let mut oauth = AuthSerializer::new();
/// let mut map: HashMap<AuthParameter, &str> = HashMap::new();
/// map.insert(AuthParameter::ClientId, "client_id");
/// map.insert(AuthParameter::ClientSecret, "client_secret");
///
/// oauth.extend(map);
/// # assert_eq!(oauth.get(AuthParameter::ClientId), Some("client_id".to_string()));
/// # assert_eq!(oauth.get(AuthParameter::ClientSecret), Some("client_secret".to_string()));
/// ```
impl<V: ToString> Extend<(AuthParameter, V)> for AuthSerializer {
    fn extend<I: IntoIterator<Item = (AuthParameter, V)>>(&mut self, iter: I) {
        iter.into_iter().for_each(|entry| {
            self.insert(entry.0, entry.1);
        });
    }
}

impl fmt::Debug for AuthSerializer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map_debug: BTreeMap<&str, &str> = BTreeMap::new();
        for (key, value) in self.parameters.iter() {
            if self.log_pii {
                map_debug.insert(key.as_str(), value.as_str());
            } else if let Some(oac) = AuthParameter::iter()
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
