use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};

use reqwest::IntoUrl;

use url::Url;
use uuid::Uuid;

use graph_core::crypto::secure_random_32;
use graph_error::{AuthorizationFailure, IdentityResult, AF};

use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    AsQuery, Authority, AuthorizationUrl, AzureCloudInstance, OpenIdCredentialBuilder, Prompt,
    ResponseMode, ResponseType,
};
use crate::oauth_serializer::{AuthParameter, AuthSerializer};

use crate::identity::tracing_targets::CREDENTIAL_EXECUTOR;

#[cfg(feature = "interactive-auth")]
use {
    crate::identity::AuthorizationResponse,
    crate::interactive::{
        HostOptions, InteractiveAuthEvent, UserEvents, WebViewAuth, WebViewAuthorizationEvent,
        WebViewHostValidator, WebViewOptions,
    },
    graph_error::{WebViewError, WebViewResult},
    tao::{event_loop::EventLoopProxy, window::Window},
    wry::{WebView, WebViewBuilder},
};

const RESPONSE_TYPES_SUPPORTED: &[&str] = &["code", "id_token", "code id_token", "id_token token"];

/// OpenID Connect (OIDC) extends the OAuth 2.0 authorization protocol for use as an additional
/// authentication protocol. You can use OIDC to enable single sign-on (SSO) between your
/// OAuth-enabled applications by using a security token called an ID token.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc
#[derive(Clone)]
pub struct OpenIdAuthorizationUrlParameters {
    pub(crate) app_config: AppConfig,
    /// Required -
    /// Must include code for OpenID Connect sign-in.
    pub(crate) response_type: BTreeSet<ResponseType>,
    /// Optional (recommended)
    ///
    /// Specifies how the identity platform should return the requested token to your app.
    ///
    /// Specifies the method that should be used to send the resulting authorization code back
    /// to your app.
    ///
    /// Can be form_post or fragment.
    ///
    /// For web applications, Microsoft recommends using response_mode=form_post,
    /// to ensure the most secure transfer of tokens to your application.
    ///
    /// Open Id does not support the query response mode.
    ///
    /// Supported values:
    ///
    /// - fragment: Default when requesting an ID token by using the implicit flow.
    /// Also supported if requesting only a code.
    /// - form_post: Executes a POST containing the code to your redirect URI.
    /// Supported when requesting a code.
    pub(crate) response_mode: Option<ResponseMode>,
    /// Required
    /// A value generated and sent by your app in its request for an ID token. The same nonce
    /// value is included in the ID token returned to your app by the Microsoft identity platform.
    /// To mitigate token replay attacks, your app should verify the nonce value in the ID token
    /// is the same value it sent when requesting the token. The value is typically a unique,
    /// random string.
    ///
    /// Because openid requires a nonce as part of the OAuth flow a nonce is already included.
    /// The nonce is generated internally using the same requirements of generating a secure
    /// random string as is done when using proof key for code exchange (PKCE) in the
    /// authorization code grant. If you are unsure or unclear how the nonce works then it is
    /// recommended to stay with the generated nonce as it is cryptographically secure.
    pub(crate) nonce: String,
    /// Required -
    /// A value included in the request that also will be returned in the token response.
    /// It can be a string of any content you want. A randomly generated unique value typically
    /// is used to prevent cross-site request forgery attacks. The state also is used to encode
    /// information about the user's state in the app before the authentication request occurred,
    /// such as the page or view the user was on.
    pub(crate) state: Option<String>,
    /// Optional -
    /// Indicates the type of user interaction that is required. The only valid values at
    /// this time are login, none, consent, and select_account.
    ///
    /// The [Prompt::Login] claim forces the user to enter their credentials on that request,
    /// which negates single sign-on.
    ///
    /// The [Prompt::None] parameter is the opposite, and should be paired with a login_hint to
    /// indicate which user must be signed in. These parameters ensure that the user isn't
    /// presented with any interactive prompt at all. If the request can't be completed silently
    /// via single sign-on, the Microsoft identity platform returns an error. Causes include no
    /// signed-in user, the hinted user isn't signed in, or multiple users are signed in but no
    /// hint was provided.
    ///
    /// The [Prompt::Consent] claim triggers the OAuth consent dialog after the
    /// user signs in. The dialog asks the user to grant permissions to the app.
    ///
    /// Finally, [Prompt::SelectAccount] shows the user an account selector, negating silent SSO but
    /// allowing the user to pick which account they intend to sign in with, without requiring
    /// credential entry. You can't use both login_hint and select_account.
    pub(crate) prompt: BTreeSet<Prompt>,
    /// Optional -
    /// The realm of the user in a federated directory. This skips the email-based discovery
    /// process that the user goes through on the sign-in page, for a slightly more streamlined
    /// user experience. For tenants that are federated through an on-premises directory
    /// like AD FS, this often results in a seamless sign-in because of the existing login session.
    pub(crate) domain_hint: Option<String>,
    /// Optional -
    /// You can use this parameter to pre-fill the username and email address field of the
    /// sign-in page for the user, if you know the username ahead of time. Often, apps use
    /// this parameter during re-authentication, after already extracting the login_hint
    /// optional claim from an earlier sign-in.
    pub(crate) login_hint: Option<String>,
    verify_id_token: bool,
}

impl Debug for OpenIdAuthorizationUrlParameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenIdAuthorizationUrlParameters")
            .field("app_config", &self.app_config)
            .field("response_type", &self.response_type)
            .field("response_mode", &self.response_mode)
            .field("prompt", &self.prompt)
            .finish()
    }
}

impl OpenIdAuthorizationUrlParameters {
    pub fn new<U: ToString, I: IntoIterator<Item = U>>(
        client_id: impl TryInto<Uuid>,
        redirect_uri: impl IntoUrl,
        scope: I,
    ) -> IdentityResult<OpenIdAuthorizationUrlParameters> {
        let mut scope_set = BTreeSet::new();
        scope_set.insert("openid".to_owned());
        scope_set.extend(scope.into_iter().map(|s| s.to_string()));

        let redirect_uri_result = Url::parse(redirect_uri.as_str());

        Ok(OpenIdAuthorizationUrlParameters {
            app_config: AppConfig::builder(client_id)
                .scope(scope_set)
                .redirect_uri(redirect_uri.into_url().or(redirect_uri_result)?)
                .build(),
            response_type: BTreeSet::from([ResponseType::IdToken]),
            response_mode: None,
            nonce: secure_random_32(),
            state: None,
            prompt: Default::default(),
            domain_hint: None,
            login_hint: None,
            verify_id_token: Default::default(),
        })
    }

    fn new_with_app_config(app_config: AppConfig) -> OpenIdAuthorizationUrlParameters {
        OpenIdAuthorizationUrlParameters {
            app_config,
            response_type: BTreeSet::from([ResponseType::IdToken]),
            response_mode: None,
            nonce: secure_random_32(),
            state: None,
            prompt: Default::default(),
            domain_hint: None,
            login_hint: None,
            verify_id_token: Default::default(),
        }
    }

    pub fn builder(client_id: impl TryInto<Uuid>) -> OpenIdAuthorizationUrlParameterBuilder {
        OpenIdAuthorizationUrlParameterBuilder::new(client_id)
    }

    pub fn into_credential(self, authorization_code: impl AsRef<str>) -> OpenIdCredentialBuilder {
        OpenIdCredentialBuilder::new_with_auth_code(
            self.app_config,
            authorization_code,
            self.verify_id_token,
        )
    }

    pub fn url(&self) -> IdentityResult<Url> {
        self.authorization_url()
    }

    pub fn url_with_host(&self, azure_cloud_instance: &AzureCloudInstance) -> IdentityResult<Url> {
        self.authorization_url_with_host(azure_cloud_instance)
    }

    /// Get the nonce.
    ///
    /// This value may be generated automatically by the client and may be useful for users
    /// who want to manually verify that the nonce stored in the client is the same as the
    /// nonce returned in the response from the authorization server.
    /// Verifying the nonce helps mitigate token replay attacks.
    pub fn nonce(&self) -> &String {
        &self.nonce
    }

    #[cfg(feature = "interactive-auth")]
    pub fn interactive_webview_authentication(
        &self,
        client_secret: impl AsRef<str>,
        web_view_options: WebViewOptions,
    ) -> WebViewResult<WebViewAuthorizationEvent<OpenIdCredentialBuilder>> {
        if self.response_mode.eq(&Some(ResponseMode::FormPost)) {
            return Err(AF::msg_err(
                "response_mode",
                "interactive auth does not support ResponseMode::FormPost at this time",
            ))?;
        }
        let uri = self.url()?;
        let redirect_uri = self.redirect_uri().cloned().unwrap();
        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            OpenIdAuthorizationUrlParameters::run(
                uri,
                vec![redirect_uri],
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

        match next {
            None => unreachable!(),
            Some(auth_event) => match auth_event {
                InteractiveAuthEvent::InvalidRedirectUri(reason) => {
                    Err(WebViewError::InvalidUri(reason))
                }
                InteractiveAuthEvent::ReachedRedirectUri(uri) => {
                    let query = uri
                        .query()
                        .or(uri.fragment())
                        .ok_or(WebViewError::InvalidUri(format!(
                            "uri missing query or fragment: {}",
                            uri
                        )))?;

                    let authorization_response: AuthorizationResponse =
                        serde_urlencoded::from_str(query).map_err(|_| {
                            WebViewError::InvalidUri(format!(
                                "unable to deserialize query or fragment: {}",
                                uri
                            ))
                        })?;

                    if authorization_response.is_err() {
                        tracing::debug!(target: "graph_rs_sdk::interactive_auth", "error in authorization query or fragment from redirect uri");
                        return Ok(WebViewAuthorizationEvent::Unauthorized(
                            authorization_response,
                        ));
                    }

                    tracing::debug!(target: "graph_rs_sdk::interactive_auth", "parsed authorization query or fragment from redirect uri");

                    let mut credential_builder = OpenIdCredentialBuilder::from((
                        self.app_config.clone(),
                        authorization_response.clone(),
                    ));

                    credential_builder.with_client_secret(client_secret);

                    if self.verify_id_token {
                        credential_builder.with_id_token_verification(true);
                    }

                    Ok(WebViewAuthorizationEvent::Authorized {
                        authorization_response,
                        credential_builder,
                    })
                }
                InteractiveAuthEvent::WindowClosed(window_close_reason) => Ok(
                    WebViewAuthorizationEvent::WindowClosed(window_close_reason.to_string()),
                ),
            },
        }
    }
}

impl AuthorizationUrl for OpenIdAuthorizationUrlParameters {
    fn redirect_uri(&self) -> Option<&Url> {
        self.app_config.redirect_uri.as_ref()
    }

    fn authorization_url(&self) -> IdentityResult<Url> {
        self.authorization_url_with_host(&self.app_config.azure_cloud_instance)
    }

    fn authorization_url_with_host(
        &self,
        azure_cloud_instance: &AzureCloudInstance,
    ) -> IdentityResult<Url> {
        let mut serializer = AuthSerializer::new();

        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AuthorizationFailure::result("client_id");
        }

        if self.app_config.scope.is_empty() || !self.app_config.scope.contains("openid") {
            let mut scope = self.app_config.scope.clone();
            scope.insert("openid".into());
            serializer.set_scope(scope);
        } else {
            serializer.set_scope(self.app_config.scope.clone());
        }

        serializer
            .client_id(client_id.as_str())
            .nonce(self.nonce.as_str());

        if self.response_type.is_empty() {
            serializer.response_type(ResponseType::Code);
        } else {
            let response_types = self.response_type.as_query();
            if !RESPONSE_TYPES_SUPPORTED.contains(&response_types.as_str()) {
                let err = format!(
                    "provided response_type is not supported - supported response types are: {}",
                    RESPONSE_TYPES_SUPPORTED
                        .iter()
                        .map(|s| format!("`{}`", s))
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                tracing::error!(
                    target: CREDENTIAL_EXECUTOR,
                    err
                );
                return AuthorizationFailure::msg_result("response_type", err);
            }

            serializer.response_types(self.response_type.iter());
        }

        if let Some(response_mode) = self.response_mode.as_ref() {
            if response_mode.eq(&ResponseMode::Query) {
                return Err(AF::msg_err(
                    "response_mode",
                    "openid does not support ResponseMode::Query. Use ResponseMode::Fragment or ResponseMode::FormPost",
                ));
            }

            serializer.response_mode(response_mode.as_ref());
        }

        if let Some(redirect_uri) = self.app_config.redirect_uri.as_ref() {
            serializer.redirect_uri(redirect_uri.as_ref());
        }

        if let Some(state) = self.state.as_ref() {
            serializer.state(state.as_str());
        }

        if !self.prompt.is_empty() {
            serializer.prompt(&self.prompt.as_query());
        }

        if let Some(domain_hint) = self.domain_hint.as_ref() {
            serializer.domain_hint(domain_hint.as_str());
        }

        if let Some(login_hint) = self.login_hint.as_ref() {
            serializer.login_hint(login_hint.as_str());
        }

        let query = serializer.encode_query(
            vec![
                AuthParameter::ResponseMode,
                AuthParameter::RedirectUri,
                AuthParameter::State,
                AuthParameter::Prompt,
                AuthParameter::LoginHint,
                AuthParameter::DomainHint,
            ],
            vec![
                AuthParameter::ClientId,
                AuthParameter::ResponseType,
                AuthParameter::Scope,
                AuthParameter::Nonce,
            ],
        )?;

        let mut uri = azure_cloud_instance.auth_uri(&self.app_config.authority)?;
        uri.set_query(Some(query.as_str()));
        Ok(uri)
    }
}

#[cfg(feature = "interactive-auth")]
impl WebViewAuth for OpenIdAuthorizationUrlParameters {
    fn webview(
        host_options: HostOptions,
        window: &Window,
        proxy: EventLoopProxy<UserEvents>,
    ) -> anyhow::Result<WebView> {
        let start_uri = host_options.start_uri.clone();
        let validator = WebViewHostValidator::try_from(host_options)?;
        Ok(WebViewBuilder::new(window)
            .with_url(start_uri.as_ref())?
            // Disables file drop
            .with_file_drop_handler(|_| true)
            .with_navigation_handler(move |uri| {
                if let Ok(url) = Url::parse(uri.as_str()) {
                    let is_valid_host = validator.is_valid_uri(&url);
                    let is_redirect = validator.is_redirect_host(&url);

                    if is_redirect {
                        proxy.send_event(UserEvents::ReachedRedirectUri(url))
                            .unwrap();
                        proxy.send_event(UserEvents::InternalCloseWindow)
                            .unwrap();
                        return true;
                    }

                    is_valid_host
                } else {
                    tracing::debug!(target: "graph_rs_sdk::interactive_auth", "unable to navigate webview - url is none");
                    proxy.send_event(UserEvents::CloseWindow).unwrap();
                    false
                }
            })
            .build()?)
    }
}

pub struct OpenIdAuthorizationUrlParameterBuilder {
    credential: OpenIdAuthorizationUrlParameters,
}

impl OpenIdAuthorizationUrlParameterBuilder {
    pub(crate) fn new(client_id: impl TryInto<Uuid>) -> OpenIdAuthorizationUrlParameterBuilder {
        OpenIdAuthorizationUrlParameterBuilder {
            credential: OpenIdAuthorizationUrlParameters::new_with_app_config(
                AppConfig::builder(client_id).build(),
            ),
        }
    }

    pub(crate) fn new_with_app_config(
        mut app_config: AppConfig,
    ) -> OpenIdAuthorizationUrlParameterBuilder {
        app_config.scope.insert("openid".into());
        OpenIdAuthorizationUrlParameterBuilder {
            credential: OpenIdAuthorizationUrlParameters::new_with_app_config(app_config),
        }
    }

    pub fn with_redirect_uri(&mut self, redirect_uri: Url) -> &mut Self {
        self.credential.app_config.redirect_uri = Some(redirect_uri);
        self
    }

    pub fn with_client_id(&mut self, client_id: impl TryInto<Uuid>) -> &mut Self {
        self.credential.app_config.client_id = client_id.try_into().unwrap_or_default();
        self
    }

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.credential.app_config.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.credential.app_config.authority = authority.into();
        self
    }

    /// Default is code.
    /// Must include code for the open id connect flow.
    /// Can also include id_token or token if using the hybrid flow.
    ///
    /// Supported response types are:
    ///
    /// - code
    /// - id_token
    /// - code id_token
    /// - id_token token
    pub fn with_response_type<I: IntoIterator<Item = ResponseType>>(
        &mut self,
        response_type: I,
    ) -> &mut Self {
        self.credential.response_type = BTreeSet::from_iter(response_type);
        self
    }

    /// Specifies how the identity platform should return the requested token to your app.
    ///
    /// Supported values:
    ///
    /// - **fragment**: Default when requesting an ID token by using the implicit flow.
    ///     Also supported if requesting only a code.
    /// - **form_post**: Executes a POST containing the code to your redirect URI.
    ///     Supported when requesting a code.
    pub fn with_response_mode(&mut self, response_mode: ResponseMode) -> &mut Self {
        self.credential.response_mode = Some(response_mode);
        self
    }

    /// A value included in the request, generated by the app, that is included in the
    /// resulting id_token as a claim. The app can then verify this value to mitigate token
    /// replay attacks. The value is typically a randomized, unique string that can be used
    /// to identify the origin of the request.
    ///
    /// Because openid requires a nonce as part of the openid flow a secure random nonce
    /// is already generated for OpenIdCredential. Providing a nonce here will override this
    /// generated nonce.
    pub fn with_nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
        self.credential.nonce = nonce.as_ref().to_owned();
        self
    }

    pub fn with_state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
        self.credential.state = Some(state.as_ref().to_owned());
        self
    }

    /// Takes an iterator of scopes to use in the request.
    /// Replaces current scopes if any were added previously.
    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.credential.app_config.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
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
    pub fn with_prompt<I: IntoIterator<Item = Prompt>>(&mut self, prompt: I) -> &mut Self {
        self.credential.prompt.extend(prompt.into_iter());
        self
    }

    /// Optional
    /// The realm of the user in a federated directory. This skips the email-based discovery
    /// process that the user goes through on the sign-in page, for a slightly more streamlined
    /// user experience. For tenants that are federated through an on-premises directory
    /// like AD FS, this often results in a seamless sign-in because of the existing login session.
    pub fn with_domain_hint<T: AsRef<str>>(&mut self, domain_hint: T) -> &mut Self {
        self.credential.domain_hint = Some(domain_hint.as_ref().to_owned());
        self
    }

    /// Optional
    /// You can use this parameter to pre-fill the username and email address field of the
    /// sign-in page for the user, if you know the username ahead of time. Often, apps use
    /// this parameter during reauthentication, after already extracting the login_hint
    /// optional claim from an earlier sign-in.
    pub fn with_login_hint<T: AsRef<str>>(&mut self, login_hint: T) -> &mut Self {
        self.credential.login_hint = Some(login_hint.as_ref().to_owned());
        self
    }

    pub fn with_id_token_verification(&mut self, verify_id_token: bool) -> &mut Self {
        self.credential.verify_id_token = verify_id_token;
        self
    }

    #[cfg(feature = "interactive-auth")]
    pub fn with_interactive_auth(
        &self,
        client_secret: impl AsRef<str>,
        options: WebViewOptions,
    ) -> WebViewResult<WebViewAuthorizationEvent<OpenIdCredentialBuilder>> {
        self.credential
            .interactive_webview_authentication(client_secret, options)
    }

    pub fn build(&self) -> OpenIdAuthorizationUrlParameters {
        self.credential.clone()
    }

    pub fn url_with_host(&self, azure_cloud_instance: &AzureCloudInstance) -> IdentityResult<Url> {
        self.credential.url_with_host(azure_cloud_instance)
    }

    pub fn url(&self) -> IdentityResult<Url> {
        self.credential.url()
    }

    pub fn as_credential(&self, authorization_code: impl AsRef<str>) -> OpenIdCredentialBuilder {
        OpenIdCredentialBuilder::new_with_auth_code(
            self.credential.app_config.clone(),
            authorization_code,
            false,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::identity::TokenCredentialExecutor;

    #[test]
    #[should_panic]
    fn panics_on_invalid_response_type_code_token() {
        let _ = OpenIdAuthorizationUrlParameters::builder(Uuid::new_v4())
            .with_response_type([ResponseType::Code, ResponseType::Token])
            .with_scope(["scope"])
            .url()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn panics_on_invalid_client_id() {
        let _ = OpenIdAuthorizationUrlParameters::builder("client_id")
            .with_response_type([ResponseType::Token])
            .with_scope(["scope"])
            .url()
            .unwrap();
    }

    #[test]
    fn scope_openid_automatically_set() {
        let url = OpenIdAuthorizationUrlParameters::builder(Uuid::new_v4())
            .with_response_type([ResponseType::Code])
            .with_scope(["user.read"])
            .url()
            .unwrap();
        let query = url.query().unwrap();
        assert!(query.contains("scope=openid+user.read"))
    }

    #[test]
    fn into_credential() {
        let client_id = Uuid::new_v4();
        let url_builder = OpenIdAuthorizationUrlParameters::builder(client_id)
            .with_response_type([ResponseType::Code])
            .with_scope(["user.read"])
            .build();
        let mut credential = url_builder.into_credential("code");
        let confidential_client = credential
            .with_client_secret("secret")
            .with_tenant("tenant")
            .build();

        assert_eq!(
            confidential_client.client_id().to_string(),
            client_id.to_string()
        );
    }
}
