use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::str::FromStr;
use std::time::Duration;

use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use http::{HeaderMap, HeaderName, HeaderValue};
use tracing::error;
use url::Url;
use uuid::Uuid;

use graph_core::http::{
    AsyncResponseConverterExt, HttpResponseExt, JsonHttpResponse, ResponseConverterExt,
};
use graph_error::{
    AuthExecutionError, AuthExecutionResult, AuthTaskExecutionResult, AuthorizationFailure,
    IdentityResult,
};

use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    Authority, AzureCloudInstance, DeviceAuthorizationResponse, ForceTokenRefresh,
    PollDeviceCodeEvent, PublicClientApplication, Token, TokenCredentialExecutor,
};

#[cfg(feature = "interactive-auth")]
use crate::oauth::InteractiveDeviceCodeEvent;

#[cfg(feature = "interactive-auth")]
use graph_error::WebViewResult;

#[cfg(feature = "interactive-auth")]
use crate::web::{InteractiveWebView, WebViewOptions, WindowCloseReason};

#[cfg(feature = "interactive-auth")]
use std::sync::mpsc::{Receiver, Sender};

const DEVICE_CODE_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:device_code";

credential_builder!(
    DeviceCodeCredentialBuilder,
    PublicClientApplication<DeviceCodeCredential>
);

/// The device authorization grant: allows users to sign in to input-constrained devices
/// such as a smart TV, IoT device, or a printer. To enable this flow, the device has the
/// user visit a webpage in a browser on another device to sign in. Once the user signs in,
/// the device is able to get access tokens and refresh tokens as needed.
///
/// For more info on the protocol supported by the Microsoft Identity Platform see the
/// [Microsoft identity platform and the OAuth 2.0 device authorization grant flow](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code)
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

#[cfg(feature = "interactive-auth")]
#[derive(Debug)]
pub enum DeviceCodeInteractiveEvent {
    DeviceAuthorization(DeviceAuthorizationResponse),
    Failed(JsonHttpResponse),
    WindowClosed(WindowCloseReason),
    Success(PublicClientApplication<DeviceCodeCredential>),
}

#[derive(Debug)]
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

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(mut self, scope: I) -> Self {
        self.credential.app_config.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_tenant(mut self, tenant_id: impl AsRef<str>) -> Self {
        self.credential.app_config.tenant_id = Some(tenant_id.as_ref().to_owned());
        self
    }

    pub fn poll(&mut self) -> AuthExecutionResult<std::sync::mpsc::Receiver<JsonHttpResponse>> {
        let (sender, receiver) = std::sync::mpsc::channel();

        let mut credential = self.credential.clone();
        let response = credential.execute()?;

        let http_response = response.into_http_response()?;
        let json = http_response.json().unwrap();
        let device_code_response: DeviceAuthorizationResponse = serde_json::from_value(json)?;

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
                            Err(_) => {
                                error!(
                                    target = "device_code_polling_executor",
                                    "Invalid PollDeviceCodeEvent"
                                );
                                break;
                            }
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
        let device_code_response: DeviceAuthorizationResponse =
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

    #[cfg(feature = "interactive-auth")]
    pub fn execute_interactive_authentication(
        &mut self,
    ) -> AuthExecutionResult<DeviceCodeInteractiveAuth> {
        let response = self.credential.execute()?;
        let device_authorization_response: DeviceAuthorizationResponse = response.json()?;
        Ok(DeviceCodeInteractiveAuth {
            credential: self.credential.clone(),
            device_authorization_response,
        })
    }
}

#[cfg(feature = "interactive-auth")]
#[derive(Debug)]
pub struct DeviceCodeInteractiveAuth {
    credential: DeviceCodeCredential,
    pub device_authorization_response: DeviceAuthorizationResponse,
}

#[allow(dead_code)]
#[cfg(feature = "interactive-auth")]
impl DeviceCodeInteractiveAuth {
    pub(crate) fn new(
        credential: DeviceCodeCredential,
        device_authorization_response: DeviceAuthorizationResponse,
    ) -> DeviceCodeInteractiveAuth {
        DeviceCodeInteractiveAuth {
            credential,
            device_authorization_response,
        }
    }

    pub fn begin(
        &mut self,
        options: Option<WebViewOptions>,
    ) -> WebViewResult<Receiver<DeviceCodeInteractiveEvent>> {
        let executor = self.interactive_webview_authentication(options)?;
        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            DeviceCodeInteractiveAuth::execute_interactive_loop(sender, executor);
        });

        Ok(receiver)
    }

    #[tracing::instrument]
    fn execute_interactive_loop(
        sender: Sender<DeviceCodeInteractiveEvent>,
        executor: Receiver<InteractiveDeviceCodeEvent>,
    ) {
        loop {
            match executor.recv() {
                Ok(interactive_device_code_event) => match interactive_device_code_event {
                    InteractiveDeviceCodeEvent::PollDeviceCode {
                        poll_device_code_event,
                        response,
                    } => {
                        let res = response.json().unwrap_or_default().to_string();
                        tracing::debug!(target: "device_code_polling_executor", poll_device_code = poll_device_code_event.as_str(), http_response = res);

                        match poll_device_code_event {
                            PollDeviceCodeEvent::AuthorizationPending
                            | PollDeviceCodeEvent::SlowDown => continue,
                            PollDeviceCodeEvent::AuthorizationDeclined
                            | PollDeviceCodeEvent::BadVerificationCode
                            | PollDeviceCodeEvent::ExpiredToken
                            | PollDeviceCodeEvent::AccessDenied => {
                                sender
                                    .send(DeviceCodeInteractiveEvent::Failed(response))
                                    .unwrap_or_default();
                                break;
                            }
                        }
                    }
                    InteractiveDeviceCodeEvent::WindowClosed(window_closed) => {
                        sender
                            .send(DeviceCodeInteractiveEvent::WindowClosed(window_closed))
                            .unwrap_or_default();
                        break;
                    }
                    InteractiveDeviceCodeEvent::SuccessfulAuthEvent {
                        response: _,
                        public_application,
                    } => {
                        tracing::debug!(target: "device_code_polling_executor", "PublicApplication: {public_application:#?}");
                        sender
                            .send(DeviceCodeInteractiveEvent::Success(public_application))
                            .unwrap_or_default();
                    }
                    _ => {}
                },
                Err(err) => panic!("{}", err),
            }
        }
    }

    #[tracing::instrument]
    pub fn interactive_webview_authentication(
        &mut self,
        options: Option<WebViewOptions>,
    ) -> WebViewResult<Receiver<InteractiveDeviceCodeEvent>> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut credential = self.credential.clone();
        let device_authorization_response = self.device_authorization_response.clone();

        // Spawn thread for webview
        let sender3 = sender.clone();
        std::thread::spawn(move || {
            let url = {
                if let Some(url_complete) = device_authorization_response
                    .verification_uri_complete
                    .as_ref()
                {
                    Url::parse(url_complete).unwrap()
                } else {
                    Url::parse(device_authorization_response.verification_uri.as_str()).unwrap()
                }
            };

            InteractiveWebView::device_code_interactive_authentication(
                url,
                options.unwrap_or_default(),
                sender3,
            )
            .unwrap();
        });

        let device_code = device_authorization_response.device_code;
        let interval = Duration::from_secs(device_authorization_response.interval);
        credential.with_device_code(device_code);

        let sender2 = sender;
        std::thread::spawn(move || {
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
                tracing::debug!(target: "device_code_polling_executor", "{response:#?}");
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
                                    sender2.send(InteractiveDeviceCodeEvent::PollDeviceCode {
                                        response: http_response,
                                        poll_device_code_event:
                                            PollDeviceCodeEvent::AuthorizationPending,
                                    })?;
                                    continue;
                                }
                                PollDeviceCodeEvent::AuthorizationDeclined => {
                                    sender2.send(InteractiveDeviceCodeEvent::PollDeviceCode {
                                        response: http_response,
                                        poll_device_code_event:
                                            PollDeviceCodeEvent::AuthorizationDeclined,
                                    })?;
                                    break;
                                }
                                PollDeviceCodeEvent::BadVerificationCode => {
                                    sender2.send(InteractiveDeviceCodeEvent::PollDeviceCode {
                                        response: http_response,
                                        poll_device_code_event:
                                            PollDeviceCodeEvent::BadVerificationCode,
                                    })?;
                                    continue;
                                }
                                PollDeviceCodeEvent::ExpiredToken => {
                                    sender2.send(InteractiveDeviceCodeEvent::PollDeviceCode {
                                        response: http_response,
                                        poll_device_code_event: PollDeviceCodeEvent::ExpiredToken,
                                    })?;
                                    break;
                                }
                                PollDeviceCodeEvent::AccessDenied => {
                                    sender2.send(InteractiveDeviceCodeEvent::PollDeviceCode {
                                        response: http_response,
                                        poll_device_code_event: PollDeviceCodeEvent::AccessDenied,
                                    })?;
                                    break;
                                }
                                PollDeviceCodeEvent::SlowDown => {
                                    sender2.send(InteractiveDeviceCodeEvent::PollDeviceCode {
                                        response: http_response,
                                        poll_device_code_event: PollDeviceCodeEvent::SlowDown,
                                    })?;

                                    should_slow_down = true;
                                    continue;
                                }
                            },
                            Err(err) => {
                                tracing::trace!(target: "device_code_polling_executor", "Error occurred while polling device code: {err:#?}");
                                sender2.send(InteractiveDeviceCodeEvent::PollDeviceCode {
                                    response: http_response,
                                    poll_device_code_event: PollDeviceCodeEvent::AccessDenied,
                                })?;
                                break;
                            }
                        }
                    } else {
                        sender2.send(InteractiveDeviceCodeEvent::PollDeviceCode {
                            response: http_response,
                            poll_device_code_event: PollDeviceCodeEvent::AccessDenied,
                        })?;
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
