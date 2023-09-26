use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{
    Authority, AuthorizationQueryResponse, AuthorizationUrl, AzureCloudInstance, Crypto, Prompt,
    ResponseMode,
};
use crate::oauth::{ProofKeyForCodeExchange, ResponseType};
use graph_extensions::web::{InteractiveAuthenticator, WebViewOptions};

use graph_error::{AuthorizationResult, AF};

use crate::identity::credentials::app_config::AppConfig;
use reqwest::IntoUrl;
use std::collections::BTreeSet;
use url::form_urlencoded::Serializer;
use url::Url;
use uuid::Uuid;

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
pub struct AuthCodeAuthorizationUrlParameters {
    pub(crate) app_config: AppConfig,
    pub(crate) response_type: BTreeSet<ResponseType>,
    /// Optional
    /// Specifies how the identity platform should return the requested token to your app.
    ///
    /// Supported values:
    ///
    /// - query: Default when requesting an access token. Provides the code as a query string
    /// parameter on your redirect URI. The query parameter isn't supported when requesting an
    /// ID token by using the implicit flow.
    /// - fragment: Default when requesting an ID token by using the implicit flow.
    /// Also supported if requesting only a code.
    /// - form_post: Executes a POST containing the code to your redirect URI.
    /// Supported when requesting a code.
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

impl AuthCodeAuthorizationUrlParameters {
    pub fn new<T: AsRef<str>, U: IntoUrl>(
        client_id: T,
        redirect_uri: U,
    ) -> AuthorizationResult<AuthCodeAuthorizationUrlParameters> {
        let mut response_type = BTreeSet::new();
        response_type.insert(ResponseType::Code);
        let redirect_uri_result = Url::parse(redirect_uri.as_str());

        Ok(AuthCodeAuthorizationUrlParameters {
            app_config: AppConfig {
                tenant_id: None,
                client_id: Uuid::try_parse(client_id.as_ref())?,
                authority: Default::default(),
                azure_cloud_instance: Default::default(),
                extra_query_parameters: Default::default(),
                extra_header_parameters: Default::default(),
                redirect_uri: Some(redirect_uri.into_url().or(redirect_uri_result)?),
            },
            response_type,
            response_mode: None,
            nonce: None,
            state: None,
            scope: vec![],
            prompt: None,
            domain_hint: None,
            login_hint: None,
            code_challenge: None,
            code_challenge_method: None,
        })
    }

    pub fn builder<T: AsRef<str>>(client_id: T) -> AuthCodeAuthorizationUrlParameterBuilder {
        AuthCodeAuthorizationUrlParameterBuilder::new(client_id)
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
        self.url_with_host(&AzureCloudInstance::default())
    }

    pub fn url_with_host(
        &self,
        azure_cloud_instance: &AzureCloudInstance,
    ) -> AuthorizationResult<Url> {
        self.authorization_url_with_host(azure_cloud_instance)
    }

    /// Get the nonce.
    ///
    /// This value may be generated automatically by the client and may be useful for users
    /// who want to manually verify that the nonce stored in the client is the same as the
    /// nonce returned in the response from the authorization server.
    /// Verifying the nonce helps mitigate token replay attacks.
    pub fn nonce(&mut self) -> Option<&String> {
        self.nonce.as_ref()
    }

    pub fn interactive_webview_authentication(
        &self,
        interactive_web_view_options: Option<WebViewOptions>,
    ) -> anyhow::Result<AuthorizationQueryResponse> {
        let url_string = self
            .interactive_authentication(interactive_web_view_options)?
            .ok_or(anyhow::Error::msg(
                "Unable to get url from redirect in web view".to_string(),
            ))?;
        dbg!(&url_string);
        /*


        if let Ok(url) = Url::parse(url_string.as_str()) {
            dbg!(&url);

            if let Some(query) = url.query() {
                let response_query: AuthResponseQuery = serde_urlencoded::from_str(query)?;
            }

        }

        let query: HashMap<String, String> =  url.query_pairs().map(|(key, value)| (key.to_string(), value.to_string()))
                        .collect();

                    let code = query.get("code");
                    let id_token = query.get("id_token");
                    let access_token = query.get("access_token");
                    let state = query.get("state");
                    let nonce = query.get("nonce");
                    dbg!(&code, &id_token, &access_token, &state, &nonce);
         */

        let url = Url::parse(&url_string)?;
        let query = url.query().or(url.fragment()).ok_or(AF::msg_err(
            "query | fragment",
            &format!("No query or fragment returned on redirect, url: {url}"),
        ))?;

        let response_query: AuthorizationQueryResponse = serde_urlencoded::from_str(query)?;
        Ok(response_query)
    }
}

mod web_view_authenticator {
    use crate::identity::{AuthCodeAuthorizationUrlParameters, AuthorizationUrl};
    use graph_extensions::web::{InteractiveAuthenticator, InteractiveWebView, WebViewOptions};

    impl InteractiveAuthenticator for AuthCodeAuthorizationUrlParameters {
        fn interactive_authentication(
            &self,
            interactive_web_view_options: Option<WebViewOptions>,
        ) -> anyhow::Result<Option<String>> {
            let uri = self.authorization_url()?;
            let redirect_uri = self.redirect_uri().cloned().unwrap();
            let web_view_options = interactive_web_view_options.unwrap_or_default();
            let _timeout = web_view_options.timeout;
            let (sender, receiver) = std::sync::mpsc::channel();

            std::thread::spawn(move || {
                InteractiveWebView::interactive_authentication(
                    uri,
                    redirect_uri,
                    web_view_options,
                    sender,
                )
                .unwrap();
            });

            let mut iter = receiver.try_iter();
            let mut next = iter.next();
            while next.is_none() {
                next = iter.next();
            }

            Ok(next)
        }
    }
}

impl AuthorizationUrl for AuthCodeAuthorizationUrlParameters {
    fn redirect_uri(&self) -> Option<&Url> {
        self.app_config.redirect_uri.as_ref()
    }

    fn authorization_url(&self) -> AuthorizationResult<Url> {
        self.authorization_url_with_host(&AzureCloudInstance::default())
    }

    fn authorization_url_with_host(
        &self,
        azure_cloud_instance: &AzureCloudInstance,
    ) -> AuthorizationResult<Url> {
        let mut serializer = OAuthSerializer::new();

        if let Some(redirect_uri) = self.app_config.redirect_uri.as_ref() {
            if redirect_uri.as_str().trim().is_empty() {
                return AF::result("redirect_uri");
            } else {
                serializer.redirect_uri(redirect_uri.as_str());
            }
        }

        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AF::result("client_id");
        }

        if self.scope.is_empty() {
            return AF::result("scope");
        }

        if self.scope.contains(&String::from("openid")) {
            return AF::msg_result(
                "openid",
                "Scope openid is not valid for authorization code - instead use OpenIdCredential",
            );
        }

        serializer
            .client_id(client_id.as_str())
            .extend_scopes(self.scope.clone())
            .authority(azure_cloud_instance, &self.app_config.authority);

        let response_types: Vec<String> =
            self.response_type.iter().map(|s| s.to_string()).collect();

        if response_types.is_empty() {
            serializer.response_type("code");
            if let Some(response_mode) = self.response_mode.as_ref() {
                serializer.response_mode(response_mode.as_ref());
            }
        } else {
            let response_type = response_types.join(" ").trim().to_owned();
            if response_type.is_empty() {
                serializer.response_type("code");
            } else {
                serializer.response_type(response_type);
            }

            // Set response_mode
            if self.response_type.contains(&ResponseType::IdToken) {
                if self.response_mode.is_none() || self.response_mode.eq(&Some(ResponseMode::Query))
                {
                    serializer.response_mode(ResponseMode::Fragment.as_ref());
                } else if let Some(response_mode) = self.response_mode.as_ref() {
                    serializer.response_mode(response_mode.as_ref());
                }
            } else if let Some(response_mode) = self.response_mode.as_ref() {
                serializer.response_mode(response_mode.as_ref());
            }
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

        if let Some(nonce) = self.nonce.as_ref() {
            serializer.nonce(nonce);
        }

        if let Some(code_challenge) = self.code_challenge.as_ref() {
            serializer.code_challenge(code_challenge.as_str());
        }

        if let Some(code_challenge_method) = self.code_challenge_method.as_ref() {
            serializer.code_challenge_method(code_challenge_method.as_str());
        }

        let mut encoder = Serializer::new(String::new());
        serializer.encode_query(
            vec![
                OAuthParameter::ResponseMode,
                OAuthParameter::State,
                OAuthParameter::Prompt,
                OAuthParameter::LoginHint,
                OAuthParameter::DomainHint,
                OAuthParameter::Nonce,
                OAuthParameter::CodeChallenge,
                OAuthParameter::CodeChallengeMethod,
            ],
            vec![
                OAuthParameter::ClientId,
                OAuthParameter::ResponseType,
                OAuthParameter::RedirectUri,
                OAuthParameter::Scope,
            ],
            &mut encoder,
        )?;

        let authorization_url = serializer
            .get(OAuthParameter::AuthorizationUrl)
            .ok_or(AF::msg_internal_err("authorization_url"))?;
        let mut url = Url::parse(authorization_url.as_str())?;
        url.set_query(Some(encoder.finish().as_str()));
        Ok(url)
    }
}

#[derive(Clone)]
pub struct AuthCodeAuthorizationUrlParameterBuilder {
    parameters: AuthCodeAuthorizationUrlParameters,
}

impl AuthCodeAuthorizationUrlParameterBuilder {
    pub fn new<T: AsRef<str>>(client_id: T) -> AuthCodeAuthorizationUrlParameterBuilder {
        let mut response_type = BTreeSet::new();
        response_type.insert(ResponseType::Code);
        AuthCodeAuthorizationUrlParameterBuilder {
            parameters: AuthCodeAuthorizationUrlParameters {
                app_config: AppConfig::new_with_client_id(client_id.as_ref()),
                response_mode: None,
                response_type,
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

    pub(crate) fn new_with_app_config(
        app_config: AppConfig,
    ) -> AuthCodeAuthorizationUrlParameterBuilder {
        let mut response_type = BTreeSet::new();
        response_type.insert(ResponseType::Code);
        AuthCodeAuthorizationUrlParameterBuilder {
            parameters: AuthCodeAuthorizationUrlParameters {
                app_config,
                response_mode: None,
                response_type,
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

    pub fn with_redirect_uri<U: IntoUrl>(&mut self, redirect_uri: U) -> &mut Self {
        self.parameters.app_config.redirect_uri = Some(redirect_uri.into_url().unwrap());
        self
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.parameters.app_config.client_id =
            Uuid::try_parse(client_id.as_ref()).expect("Invalid Client Id - Must be a Uuid ");
        self
    }

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.parameters.app_config.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.parameters.app_config.authority = authority.into();
        self
    }

    /// Default is code. Must include code for the authorization code flow.
    /// Can also include id_token or token if using the hybrid flow.
    pub fn with_response_type<I: IntoIterator<Item = ResponseType>>(
        &mut self,
        response_type: I,
    ) -> &mut Self {
        self.parameters
            .response_type
            .extend(response_type.into_iter());
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
        self.parameters.response_mode = Some(response_mode);
        self
    }

    /// A value included in the request, generated by the app, that is included in the
    /// resulting id_token as a claim. The app can then verify this value to mitigate token
    /// replay attacks. The value is typically a randomized, unique string that can be used
    /// to identify the origin of the request.
    pub fn with_nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
        self.parameters.nonce = Some(nonce.as_ref().to_owned());
        self
    }

    /// A value included in the request, generated by the app, that is included in the
    /// resulting id_token as a claim. The app can then verify this value to mitigate token
    /// replay attacks. The value is typically a randomized, unique string that can be used
    /// to identify the origin of the request.
    ///
    /// The nonce is generated in the same way as generating a PKCE.
    ///
    /// Internally this method uses the Rust ring cyrpto library to
    /// generate a secure random 32-octet sequence that is base64 URL
    /// encoded (no padding). This sequence is hashed using SHA256 and
    /// base64 URL encoded (no padding) resulting in a 43-octet URL safe string.
    #[doc(hidden)]
    pub(crate) fn with_nonce_generated(&mut self) -> anyhow::Result<&mut Self> {
        self.parameters.nonce = Some(Crypto::sha256_secure_string()?.1);
        Ok(self)
    }

    pub fn with_state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
        self.parameters.state = Some(state.as_ref().to_owned());
        self
    }

    /// Set the required permissions for the authorization request.
    ///
    /// Providing a scope of `id_token` automatically adds [ResponseType::IdToken]
    /// and generates a secure nonce value.
    /// See [AuthCodeAuthorizationUrlParameterBuilder::with_nonce_generated]
    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.parameters.scope.extend(
            scope
                .into_iter()
                .map(|s| s.to_string())
                .map(|s| s.trim().to_owned()),
        );

        if self.parameters.nonce.is_none()
            && self.parameters.scope.contains(&String::from("id_token"))
        {
            let _ = self.with_id_token_scope();
        }
        self
    }

    /// Automatically adds `profile` and `email` to the scope parameter.
    ///
    /// If you need a refresh token then include `offline_access` as a scope.
    /// The `offline_access` scope is not included here.
    pub fn with_default_scope(&mut self) -> anyhow::Result<&mut Self> {
        self.parameters
            .scope
            .extend(vec!["profile".to_owned(), "email".to_owned()]);
        Ok(self)
    }

    /// Adds the `offline_access` scope parameter which tells the authorization server
    /// to include a refresh token in the redirect uri query.
    pub fn with_refresh_token_scope(&mut self) -> &mut Self {
        self.parameters
            .scope
            .extend(vec!["offline_access".to_owned()]);
        self
    }

    /// Adds the `id_token` scope parameter which tells the authorization server
    /// to include an id token in the redirect uri query.
    ///
    /// Including the `id_token` scope also adds the id_token response type
    /// and adds the `openid` scope parameter.
    ///
    /// Including `id_token` also requires a nonce parameter.
    /// This is generated automatically.
    /// See [AuthCodeAuthorizationUrlParameterBuilder::with_nonce_generated]
    fn with_id_token_scope(&mut self) -> anyhow::Result<&mut Self> {
        self.with_nonce_generated()?;
        self.parameters.response_type.extend(ResponseType::IdToken);
        self.parameters.scope.extend(vec!["id_token".to_owned()]);
        Ok(self)
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
        self.parameters.prompt = Some(prompt);
        self
    }

    pub fn with_domain_hint<T: AsRef<str>>(&mut self, domain_hint: T) -> &mut Self {
        self.parameters.domain_hint = Some(domain_hint.as_ref().to_owned());
        self
    }

    pub fn with_login_hint<T: AsRef<str>>(&mut self, login_hint: T) -> &mut Self {
        self.parameters.login_hint = Some(login_hint.as_ref().to_owned());
        self
    }

    /// Used to secure authorization code grants by using Proof Key for Code Exchange (PKCE).
    /// Required if code_challenge_method is included.
    pub fn with_code_challenge<T: AsRef<str>>(&mut self, code_challenge: T) -> &mut Self {
        self.parameters.code_challenge = Some(code_challenge.as_ref().to_owned());
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
        self.parameters.code_challenge_method = Some(code_challenge_method.as_ref().to_owned());
        self
    }

    /// Sets the code_challenge and code_challenge_method using the [ProofKeyForCodeExchange]
    /// Callers should keep the [ProofKeyForCodeExchange] and provide it to the credential
    /// builder in order to set the client verifier and request an access token.
    pub fn with_pkce(
        &mut self,
        proof_key_for_code_exchange: &ProofKeyForCodeExchange,
    ) -> &mut Self {
        self.with_code_challenge(proof_key_for_code_exchange.code_challenge.as_str());
        self.with_code_challenge_method(proof_key_for_code_exchange.code_challenge_method.as_str());
        self
    }

    pub fn build(&self) -> AuthCodeAuthorizationUrlParameters {
        self.parameters.clone()
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
        self.parameters.url()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_uri() {
        let authorizer = AuthCodeAuthorizationUrlParameters::builder(Uuid::new_v4().to_string())
            .with_redirect_uri("https://localhost:8080")
            .with_scope(["read", "write"])
            .build();

        let url_result = authorizer.url();
        assert!(url_result.is_ok());
    }

    #[test]
    fn url_with_host() {
        let authorizer = AuthCodeAuthorizationUrlParameters::builder(Uuid::new_v4().to_string())
            .with_redirect_uri("https://localhost:8080")
            .with_scope(["read", "write"])
            .build();

        let url_result = authorizer.url_with_host(&AzureCloudInstance::AzureGermany);
        assert!(url_result.is_ok());
    }

    #[test]
    fn response_mode_set() {
        let url = AuthCodeAuthorizationUrlParameters::builder(Uuid::new_v4().to_string())
            .with_redirect_uri("https://localhost:8080")
            .with_scope(["read", "write"])
            .with_response_type(ResponseType::IdToken)
            .url()
            .unwrap();

        let query = url.query().unwrap();
        dbg!(query);
        assert!(query.contains("response_mode=fragment"));
        assert!(query.contains("response_type=code+id_token"));
    }

    #[test]
    fn response_mode_not_set() {
        let url = AuthCodeAuthorizationUrlParameters::builder(Uuid::new_v4().to_string())
            .with_redirect_uri("https://localhost:8080")
            .with_scope(["read", "write"])
            .url()
            .unwrap();

        let query = url.query().unwrap();
        assert!(!query.contains("response_mode"));
        assert!(query.contains("response_type=code"));
    }

    #[test]
    fn multi_response_type_set() {
        let url = AuthCodeAuthorizationUrlParameters::builder(Uuid::new_v4().to_string())
            .with_redirect_uri("https://localhost:8080")
            .with_scope(["read", "write"])
            .with_response_mode(ResponseMode::FormPost)
            .with_response_type(vec![ResponseType::IdToken, ResponseType::Code])
            .url()
            .unwrap();

        let query = url.query().unwrap();
        assert!(query.contains("response_mode=form_post"));
        assert!(query.contains("response_type=code+id_token"));
    }

    #[test]
    fn generate_nonce() {
        let url = AuthCodeAuthorizationUrlParameters::builder(Uuid::new_v4().to_string())
            .with_redirect_uri("https://localhost:8080")
            .with_scope(["read", "write"])
            .with_response_type(vec![ResponseType::Code, ResponseType::IdToken])
            .with_nonce_generated()
            .unwrap()
            .url()
            .unwrap();

        let query = url.query().unwrap();
        assert!(query.contains("response_mode=fragment"));
        assert!(query.contains("response_type=code+id_token"));
        assert!(query.contains("nonce"));
    }
}
