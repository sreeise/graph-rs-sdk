use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::process::exit;
use std::str::FromStr;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use http::{HeaderMap, HeaderName, HeaderValue};
use url::Url;
use uuid::Uuid;

use graph_core::http::{
    AsyncResponseConverterExt, HttpResponseExt, JsonHttpResponse, ResponseConverterExt,
};
use graph_error::{
    AuthExecutionError, AuthExecutionResult, AuthTaskExecutionResult, AuthorizationFailure,
    DeviceCodeWebViewResult, IdentityResult, WebViewExecutionError, WebViewResult, AF,
};

use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    Authority, AuthorizationQueryResponse, AzureCloudInstance, DeviceCode, ForceTokenRefresh,
    PollDeviceCodeEvent, PublicClientApplication, TokenCredentialExecutor,
};
use crate::oauth::{InteractiveDeviceCodeEvent, Token};
use crate::web::{InteractiveAuthEvent, WebViewOptions, WindowCloseReason};
use crate::web::{InteractiveAuthenticator, InteractiveWebView};

const DEVICE_CODE_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:device_code";

credential_builder!(
    DeviceCodeCredentialBuilder,
    PublicClientApplication<DeviceCodeCredential>
);

/// Allows users to sign in to input-constrained devices such as a smart TV, IoT device,
/// or a printer. To enable this flow, the device has the user visit a webpage in a browser on
/// another device to sign in. Once the user signs in, the device is able to get access tokens
/// and refresh tokens as needed.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
#[derive(Clone)]
pub struct DeviceCodeCredential {
    pub(crate) app_config: AppConfig,
    /// Required when requesting a new access token using a refresh token
    /// The refresh token needed to make an access token request using a refresh token.
    /// Do not include an authorization code when using a refresh token.
    pub(crate) refresh_token: Option<String>,
    /// Required.
    /// The device_code returned in the device authorization request.
    /// A device_code is a long string used to verify the session between the client and the authorization server.
    /// The client uses this parameter to request the access token from the authorization server.
    pub(crate) device_code: Option<String>,
    serializer: OAuthSerializer,
    token_cache: InMemoryCacheStore<Token>,
}

impl DeviceCodeCredential {
    pub fn new<U: ToString, I: IntoIterator<Item = U>>(
        client_id: impl AsRef<str>,
        device_code: impl AsRef<str>,
        scope: I,
    ) -> DeviceCodeCredential {
        DeviceCodeCredential {
            app_config: AppConfig::builder(client_id.as_ref()).scope(scope).build(),
            refresh_token: None,
            device_code: Some(device_code.as_ref().to_owned()),
            serializer: Default::default(),
            token_cache: Default::default(),
        }
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }

    pub fn with_device_code<T: AsRef<str>>(&mut self, device_code: T) -> &mut Self {
        self.device_code = Some(device_code.as_ref().to_owned());
        self
    }

    pub fn builder(client_id: impl AsRef<str>) -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder::new(client_id.as_ref())
    }

    fn execute_cached_token_refresh(&mut self, cache_id: String) -> AuthExecutionResult<Token> {
        let response = self.execute()?;
        let new_token: Token = response.json()?;
        self.token_cache.store(cache_id, new_token.clone());

        if new_token.refresh_token.is_some() {
            self.refresh_token = new_token.refresh_token.clone();
        }

        Ok(new_token)
    }

    async fn execute_cached_token_refresh_async(
        &mut self,
        cache_id: String,
    ) -> AuthExecutionResult<Token> {
        let response = self.execute_async().await?;
        let new_token: Token = response.json().await?;

        if new_token.refresh_token.is_some() {
            self.refresh_token = new_token.refresh_token.clone();
        }

        self.token_cache.store(cache_id, new_token.clone());
        Ok(new_token)
    }
}

impl Debug for DeviceCodeCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceCodeCredential")
            .field("app_config", &self.app_config)
            .finish()
    }
}

#[async_trait]
impl TokenCache for DeviceCodeCredential {
    type Token = Token;

    fn get_token_silent(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();

        match self.app_config.force_token_refresh {
            ForceTokenRefresh::Never => {
                // Attempt to bypass a read on the token store by using previous
                // refresh token stored outside of RwLock
                if self.refresh_token.is_some() {
                    if let Ok(token) = self.execute_cached_token_refresh(cache_id.clone()) {
                        return Ok(token);
                    }
                }

                if let Some(token) = self.token_cache.get(cache_id.as_str()) {
                    if token.is_expired_sub(time::Duration::minutes(5)) {
                        if let Some(refresh_token) = token.refresh_token.as_ref() {
                            self.refresh_token = Some(refresh_token.to_owned());
                        }

                        self.execute_cached_token_refresh(cache_id)
                    } else {
                        Ok(token)
                    }
                } else {
                    self.execute_cached_token_refresh(cache_id)
                }
            }
            ForceTokenRefresh::Once | ForceTokenRefresh::Always => {
                let token_result = self.execute_cached_token_refresh(cache_id);
                if self.app_config.force_token_refresh == ForceTokenRefresh::Once {
                    self.app_config.force_token_refresh = ForceTokenRefresh::Never;
                }
                token_result
            }
        }
    }

    async fn get_token_silent_async(&mut self) -> Result<Self::Token, AuthExecutionError> {
        let cache_id = self.app_config.cache_id.to_string();

        match self.app_config.force_token_refresh {
            ForceTokenRefresh::Never => {
                // Attempt to bypass a read on the token store by using previous
                // refresh token stored outside of RwLock
                if self.refresh_token.is_some() {
                    if let Ok(token) = self
                        .execute_cached_token_refresh_async(cache_id.clone())
                        .await
                    {
                        return Ok(token);
                    }
                }

                if let Some(old_token) = self.token_cache.get(cache_id.as_str()) {
                    if old_token.is_expired_sub(time::Duration::minutes(5)) {
                        if let Some(refresh_token) = old_token.refresh_token.as_ref() {
                            self.refresh_token = Some(refresh_token.to_owned());
                        }

                        self.execute_cached_token_refresh_async(cache_id).await
                    } else {
                        Ok(old_token.clone())
                    }
                } else {
                    self.execute_cached_token_refresh_async(cache_id).await
                }
            }
            ForceTokenRefresh::Once | ForceTokenRefresh::Always => {
                let token_result = self.execute_cached_token_refresh_async(cache_id).await;
                if self.app_config.force_token_refresh == ForceTokenRefresh::Once {
                    self.app_config.force_token_refresh = ForceTokenRefresh::Never;
                }
                token_result
            }
        }
    }
}

impl TokenCredentialExecutor for DeviceCodeCredential {
    fn uri(&mut self) -> IdentityResult<Url> {
        let azure_cloud_instance = self.azure_cloud_instance();
        self.serializer
            .authority_device_code(&azure_cloud_instance, &self.authority());

        if self.device_code.is_none() && self.refresh_token.is_none() {
            Ok(self
                .azure_cloud_instance()
                .device_code_uri(&self.authority())?)
        } else {
            Ok(self.azure_cloud_instance().token_uri(&self.authority())?)
        }
    }

    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AuthorizationFailure::result(OAuthParameter::ClientId.alias());
        }

        self.serializer
            .client_id(client_id.as_str())
            .set_scope(self.app_config.scope.clone());

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AuthorizationFailure::msg_result(
                    OAuthParameter::RefreshToken.alias(),
                    "Found empty string for refresh token",
                );
            }

            self.serializer
                .grant_type("refresh_token")
                .device_code(refresh_token.as_ref());

            return self.serializer.as_credential_map(
                vec![],
                vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::RefreshToken,
                    OAuthParameter::Scope,
                    OAuthParameter::GrantType,
                ],
            );
        } else if let Some(device_code) = self.device_code.as_ref() {
            if device_code.trim().is_empty() {
                return AuthorizationFailure::msg_result(
                    OAuthParameter::DeviceCode.alias(),
                    "Found empty string for device code",
                );
            }

            self.serializer
                .grant_type(DEVICE_CODE_GRANT_TYPE)
                .device_code(device_code.as_ref());

            return self.serializer.as_credential_map(
                vec![],
                vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::DeviceCode,
                    OAuthParameter::Scope,
                    OAuthParameter::GrantType,
                ],
            );
        }

        self.serializer.as_credential_map(
            vec![],
            vec![OAuthParameter::ClientId, OAuthParameter::Scope],
        )
    }

    fn client_id(&self) -> &Uuid {
        &self.app_config.client_id
    }

    fn authority(&self) -> Authority {
        self.app_config.authority.clone()
    }

    fn azure_cloud_instance(&self) -> AzureCloudInstance {
        self.app_config.azure_cloud_instance
    }

    fn app_config(&self) -> &AppConfig {
        &self.app_config
    }
}

#[derive(Clone)]
pub struct DeviceCodeCredentialBuilder {
    credential: DeviceCodeCredential,
}

impl DeviceCodeCredentialBuilder {
    fn new(client_id: impl AsRef<str>) -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder {
            credential: DeviceCodeCredential {
                app_config: AppConfig::new(client_id.as_ref()),
                refresh_token: None,
                device_code: None,
                serializer: Default::default(),
                token_cache: Default::default(),
            },
        }
    }

    pub(crate) fn new_with_device_code(
        device_code: impl AsRef<str>,
        app_config: AppConfig,
    ) -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder {
            credential: DeviceCodeCredential {
                app_config,
                refresh_token: None,
                device_code: Some(device_code.as_ref().to_owned()),
                serializer: Default::default(),
                token_cache: Default::default(),
            },
        }
    }

    pub fn with_device_code<T: AsRef<str>>(&mut self, device_code: T) -> &mut Self {
        self.credential.device_code = Some(device_code.as_ref().to_owned());
        self.credential.refresh_token = None;
        self
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.credential.device_code = None;
        self.credential.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }
}

/*
impl From<&DeviceCode> for DeviceCodeCredentialBuilder {
    fn from(value: &DeviceCode) -> Self {
        DeviceCodeCredentialBuilder {
            credential: DeviceCodeCredential {
                app_config: AppConfig::new(),
                refresh_token: None,
                device_code: Some(value.device_code.clone()),
                scope: vec![],
                serializer: Default::default(),
                token_cache: Default::default(),
            },
        }
    }
}
 */

pub struct DeviceCodePollingExecutor {
    credential: DeviceCodeCredential,
}

impl DeviceCodePollingExecutor {
    pub(crate) fn new_with_app_config(app_config: AppConfig) -> DeviceCodePollingExecutor {
        DeviceCodePollingExecutor {
            credential: DeviceCodeCredential {
                app_config,
                refresh_token: None,
                device_code: None,
                serializer: Default::default(),
                token_cache: Default::default(),
            },
        }
    }

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.credential.app_config.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_tenant(&mut self, tenant_id: impl AsRef<str>) -> &mut Self {
        self.credential.app_config.tenant_id = Some(tenant_id.as_ref().to_owned());
        self
    }

    pub fn interactive_webview_auth(&mut self, options: Option<WebViewOptions>) {
        let executor_result = self.interactive_webview_authentication(options);
        if let Ok(mut executor) = executor_result {
            while let interactive_device_code_event = executor.recv().unwrap() {
                match interactive_device_code_event {
                    InteractiveDeviceCodeEvent::BeginAuth {
                        response,
                        device_code,
                    } => {
                        println!("{:#?}", response);
                        println!("{:#?}", device_code);
                    }
                    InteractiveDeviceCodeEvent::FailedAuth {
                        response,
                        device_code,
                    } => {
                        println!("{:#?}", response);
                        println!("{:#?}", device_code);
                    }
                    InteractiveDeviceCodeEvent::PollDeviceCode {
                        poll_device_code_event,
                        response,
                    } => {
                        println!("{:#?}", response);
                        println!("{:#?}", poll_device_code_event);
                    }
                    InteractiveDeviceCodeEvent::InteractiveAuthEvent(auth_event) => {
                        match auth_event {
                            InteractiveAuthEvent::InvalidRedirectUri(_) => {}
                            InteractiveAuthEvent::ReachedRedirectUri(uri) => {
                                println!("{:#?}", uri);
                            }
                            InteractiveAuthEvent::WindowClosed(window_closed) => {
                                match window_closed {
                                    WindowCloseReason::CloseRequested => {
                                        println!("close requested");
                                    }
                                    WindowCloseReason::InvalidWindowNavigation => {
                                        println!("invalid navigation");
                                    }
                                    WindowCloseReason::TimedOut {
                                        requested_resume,
                                        start,
                                    } => {
                                        println!("Timed Out");
                                    }
                                }
                            }
                        }
                    }
                    InteractiveDeviceCodeEvent::SuccessfulAuthEvent {
                        response,
                        public_application,
                    } => {
                        println!("{:#?}", response);
                        let json = response.json().unwrap();
                        let token: Token = serde_json::from_value(json).unwrap();
                        println!("{:#?}", token);
                    }
                }
            }
        } else if let Err(err) = executor_result {
            println!("{err:#?}");
        }
    }

    pub fn interactive_webview_authentication(
        &mut self,
        options: Option<WebViewOptions>,
    ) -> DeviceCodeWebViewResult<Receiver<InteractiveDeviceCodeEvent>> {
        let (sender, receiver) = std::sync::mpsc::channel();

        let mut credential = self.credential.clone();
        let response = credential.execute()?;

        let http_response = response.into_http_response()?;

        if !http_response.status().is_success() {
            sender
                .send(InteractiveDeviceCodeEvent::FailedAuth {
                    response: http_response,
                    device_code: None,
                })
                .unwrap();
            return Ok(receiver);
        }

        if let Some(json) = http_response.json() {
            let device_code_response: DeviceCode =
                serde_json::from_value(json).map_err(AuthExecutionError::from)?;

            if device_code_response.error.is_some() {
                sender
                    .send(InteractiveDeviceCodeEvent::FailedAuth {
                        response: http_response,
                        device_code: Some(device_code_response),
                    })
                    .unwrap();
                return Ok(receiver);
            } else {
                sender
                    .send(InteractiveDeviceCodeEvent::BeginAuth {
                        response: http_response,
                        device_code: Some(device_code_response.clone()),
                    })
                    .unwrap();
            }

            let device_code = device_code_response.device_code;
            let interval = Duration::from_secs(device_code_response.interval);
            credential.with_device_code(device_code);

            let sender2 = sender.clone();
            let _ = std::thread::spawn(move || {
                let mut should_slow_down = false;

                loop {
                    // Wait the amount of seconds that interval is.
                    if should_slow_down {
                        should_slow_down = false;
                        std::thread::sleep(interval.add(Duration::from_secs(5)));
                    } else {
                        std::thread::sleep(interval);
                    }

                    let response = credential.execute().unwrap();
                    let http_response = response.into_http_response()?;
                    let status = http_response.status();

                    if status.is_success() {
                        let json = http_response.json().unwrap();
                        let token: Token = serde_json::from_value(json)?;
                        let cache_id = credential.app_config.cache_id.clone();
                        credential.token_cache.store(cache_id, token);
                        sender2.send(InteractiveDeviceCodeEvent::SuccessfulAuthEvent {
                            response: http_response,
                            public_application: PublicClientApplication::from(credential),
                        })?;
                        break;
                    } else {
                        let json = http_response.json().unwrap();
                        let option_error = json["error"].as_str().map(|value| value.to_owned());

                        if let Some(error) = option_error {
                            match PollDeviceCodeEvent::from_str(error.as_str()) {
                                Ok(poll_device_code_type) => match poll_device_code_type {
                                    PollDeviceCodeEvent::AuthorizationPending => {
                                        sender2.send(
                                            InteractiveDeviceCodeEvent::PollDeviceCode {
                                                response: http_response,
                                                poll_device_code_event:
                                                    PollDeviceCodeEvent::AuthorizationPending,
                                            },
                                        )?;
                                        continue;
                                    }
                                    PollDeviceCodeEvent::AuthorizationDeclined => {
                                        sender2.send(
                                            InteractiveDeviceCodeEvent::PollDeviceCode {
                                                response: http_response,
                                                poll_device_code_event:
                                                    PollDeviceCodeEvent::AuthorizationDeclined,
                                            },
                                        )?;
                                        break;
                                    }
                                    PollDeviceCodeEvent::BadVerificationCode => {
                                        sender2.send(
                                            InteractiveDeviceCodeEvent::PollDeviceCode {
                                                response: http_response,
                                                poll_device_code_event:
                                                    PollDeviceCodeEvent::BadVerificationCode,
                                            },
                                        )?;
                                        continue;
                                    }
                                    PollDeviceCodeEvent::ExpiredToken => {
                                        sender2.send(
                                            InteractiveDeviceCodeEvent::PollDeviceCode {
                                                response: http_response,
                                                poll_device_code_event:
                                                    PollDeviceCodeEvent::ExpiredToken,
                                            },
                                        )?;
                                        break;
                                    }
                                    PollDeviceCodeEvent::AccessDenied => {
                                        sender2.send(
                                            InteractiveDeviceCodeEvent::PollDeviceCode {
                                                response: http_response,
                                                poll_device_code_event:
                                                    PollDeviceCodeEvent::AccessDenied,
                                            },
                                        )?;
                                        break;
                                    }
                                    PollDeviceCodeEvent::SlowDown => {
                                        sender2.send(
                                            InteractiveDeviceCodeEvent::PollDeviceCode {
                                                response: http_response,
                                                poll_device_code_event:
                                                    PollDeviceCodeEvent::SlowDown,
                                            },
                                        )?;

                                        should_slow_down = true;
                                        continue;
                                    }
                                },
                                Err(_) => break,
                            }
                        } else {
                            // Body should have error or we should bail.
                            break;
                        }
                    }
                }
                Ok::<(), anyhow::Error>(())
            });

            // Spawn thread for webview
            let sender3 = sender.clone();
            std::thread::spawn(move || {
                InteractiveWebView::device_code_interactive_authentication(
                    Url::parse(device_code_response.verification_uri.as_str()).unwrap(),
                    options.unwrap_or_default(),
                    sender3,
                )
                .unwrap();
            });
        } else {
            sender
                .send(InteractiveDeviceCodeEvent::FailedAuth {
                    response: http_response,
                    device_code: None,
                })
                .unwrap();
            return Ok(receiver);
        }

        Ok(receiver)
    }

    pub fn poll(&mut self) -> AuthExecutionResult<std::sync::mpsc::Receiver<JsonHttpResponse>> {
        let (sender, receiver) = std::sync::mpsc::channel();

        let mut credential = self.credential.clone();
        let response = credential.execute()?;

        let http_response = response.into_http_response()?;
        let json = http_response.json().unwrap();
        let device_code_response: DeviceCode = serde_json::from_value(json)?;

        sender.send(http_response).unwrap();

        let device_code = device_code_response.device_code;
        let interval = Duration::from_secs(device_code_response.interval);
        credential.with_device_code(device_code);

        let _ = std::thread::spawn(move || {
            let mut should_slow_down = false;

            loop {
                // Wait the amount of seconds that interval is.
                if should_slow_down {
                    should_slow_down = false;
                    std::thread::sleep(interval.add(Duration::from_secs(5)));
                } else {
                    std::thread::sleep(interval);
                }

                let response = credential.execute().unwrap();
                let http_response = response.into_http_response()?;
                let status = http_response.status();

                if status.is_success() {
                    sender.send(http_response)?;
                    break;
                } else {
                    let json = http_response.json().unwrap();
                    let option_error = json["error"].as_str().map(|value| value.to_owned());
                    sender.send(http_response)?;

                    if let Some(error) = option_error {
                        match PollDeviceCodeEvent::from_str(error.as_str()) {
                            Ok(poll_device_code_type) => match poll_device_code_type {
                                PollDeviceCodeEvent::AuthorizationPending => continue,
                                PollDeviceCodeEvent::AuthorizationDeclined => break,
                                PollDeviceCodeEvent::BadVerificationCode => continue,
                                PollDeviceCodeEvent::ExpiredToken => break,
                                PollDeviceCodeEvent::AccessDenied => break,
                                PollDeviceCodeEvent::SlowDown => {
                                    should_slow_down = true;
                                    continue;
                                }
                            },
                            Err(_) => break,
                        }
                    } else {
                        // Body should have error or we should bail.
                        break;
                    }
                }
            }
            Ok::<(), anyhow::Error>(())
        });

        Ok(receiver)
    }

    pub async fn poll_async(
        &mut self,
        buffer: Option<usize>,
    ) -> AuthTaskExecutionResult<tokio::sync::mpsc::Receiver<JsonHttpResponse>, JsonHttpResponse>
    {
        let (sender, receiver) = {
            if let Some(buffer) = buffer {
                tokio::sync::mpsc::channel(buffer)
            } else {
                tokio::sync::mpsc::channel(100)
            }
        };

        let mut credential = self.credential.clone();
        let response = credential.execute_async().await?;

        let http_response = response.into_http_response_async().await?;
        let json = http_response.json().unwrap();
        let device_code_response: DeviceCode =
            serde_json::from_value(json).map_err(AuthExecutionError::from)?;

        sender
            .send_timeout(http_response, Duration::from_secs(60))
            .await?;

        let device_code = device_code_response.device_code;
        let mut interval = Duration::from_secs(device_code_response.interval);
        credential.with_device_code(device_code);

        tokio::spawn(async move {
            let mut should_slow_down = false;

            loop {
                // Should slow down is part of the openid connect spec and means that
                // that we should wait longer between polling by the amount specified
                // in the interval field of the device code.
                if should_slow_down {
                    should_slow_down = false;
                    interval = interval.add(Duration::from_secs(5));
                }

                // Wait the amount of seconds that interval is.
                tokio::time::sleep(interval).await;

                let response = credential.execute_async().await?;
                let http_response = response.into_http_response_async().await?;
                let status = http_response.status();

                if status.is_success() {
                    sender
                        .send_timeout(http_response, Duration::from_secs(60))
                        .await?;
                    break;
                } else {
                    let json = http_response.json().unwrap();
                    let option_error = json["error"].as_str().map(|value| value.to_owned());
                    sender
                        .send_timeout(http_response, Duration::from_secs(60))
                        .await?;

                    if let Some(error) = option_error {
                        match PollDeviceCodeEvent::from_str(error.as_str()) {
                            Ok(poll_device_code_type) => match poll_device_code_type {
                                PollDeviceCodeEvent::AuthorizationPending => continue,
                                PollDeviceCodeEvent::AuthorizationDeclined => break,
                                PollDeviceCodeEvent::BadVerificationCode => continue,
                                PollDeviceCodeEvent::ExpiredToken => break,
                                PollDeviceCodeEvent::AccessDenied => break,
                                PollDeviceCodeEvent::SlowDown => {
                                    should_slow_down = true;
                                    continue;
                                }
                            },
                            Err(_) => break,
                        }
                    } else {
                        // Body should have error or we should bail.
                        break;
                    }
                }
            }
            Ok::<(), anyhow::Error>(())
        });

        Ok(receiver)
    }
}

// #[cfg(feature = "interactive-auth")]
pub(crate) mod web_view_authenticator {
    use crate::oauth::DeviceCodeCredential;
    use crate::web::{
        InteractiveAuthEvent, InteractiveAuthenticator, InteractiveWebView, WebViewOptions,
    };
    use graph_error::WebViewResult;
    use url::Url;

    impl InteractiveAuthenticator for DeviceCodeCredential {
        fn interactive_authentication(
            &self,
            interactive_web_view_options: Option<WebViewOptions>,
        ) -> WebViewResult<std::sync::mpsc::Receiver<InteractiveAuthEvent>> {
            let uri = self
                .app_config
                .azure_cloud_instance
                .auth_uri(&self.app_config.authority)
                .expect("Internal Error Please Report");
            let web_view_options = interactive_web_view_options.unwrap_or_default();
            let (sender, receiver) = std::sync::mpsc::channel();

            std::thread::spawn(move || {
                InteractiveWebView::interactive_authentication(
                    uri,
                    vec![Url::parse("http://localhost:8080").unwrap()],
                    web_view_options,
                    sender,
                )
                .unwrap();
            });

            Ok(receiver)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn no_scope() {
        let mut credential = DeviceCodeCredential::builder("CLIENT_ID").build();

        let _ = credential.form_urlencode().unwrap();
    }
}
