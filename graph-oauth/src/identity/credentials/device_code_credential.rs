use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use std::str::FromStr;
use std::time::Duration;

use graph_core::cache::{CacheStore, InMemoryCacheStore, TokenCache};
use graph_core::identity::ForceTokenRefresh;
use http::{HeaderMap, HeaderName, HeaderValue};
use tracing::error;
use url::Url;
use uuid::Uuid;

use crate::identity::{
    AppConfig, Authority, AzureCloudInstance, DeviceAuthorizationResponse, PollDeviceCodeEvent,
    PublicClientApplication, Token, TokenCredentialExecutor,
};
use crate::oauth_serializer::{AuthParameter, AuthSerializer};
use graph_core::http::{
    AsyncResponseConverterExt, HttpResponseExt, JsonHttpResponse, ResponseConverterExt,
};
use graph_error::{
    AuthExecutionError, AuthExecutionResult, AuthTaskExecutionResult, AuthorizationFailure,
    IdentityResult,
};

#[cfg(feature = "interactive-auth")]
use {
    crate::interactive::{HostOptions, UserEvents, WebViewAuth, WebViewOptions},
    crate::tracing_targets::INTERACTIVE_AUTH,
    graph_error::WebViewDeviceCodeError,
    tao::{event_loop::EventLoopProxy, window::Window},
    wry::{WebView, WebViewBuilder},
};

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

        if !response.status().is_success() {
            return Err(AuthExecutionError::silent_token_auth(
                response.into_http_response()?,
            ));
        }

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

        if !response.status().is_success() {
            return Err(AuthExecutionError::silent_token_auth(
                response.into_http_response_async().await?,
            ));
        }

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
                    self.with_force_token_refresh(ForceTokenRefresh::Never);
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
                    self.with_force_token_refresh(ForceTokenRefresh::Never);
                }
                token_result
            }
        }
    }

    fn with_force_token_refresh(&mut self, force_token_refresh: ForceTokenRefresh) {
        self.app_config.force_token_refresh = force_token_refresh;
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
        let mut serializer = AuthSerializer::new();
        let client_id = self.app_config.client_id.to_string();
        if client_id.is_empty() || self.app_config.client_id.is_nil() {
            return AuthorizationFailure::result(AuthParameter::ClientId.alias());
        }

        serializer
            .client_id(client_id.as_str())
            .set_scope(self.app_config.scope.clone());

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AuthorizationFailure::msg_result(
                    AuthParameter::RefreshToken.alias(),
                    "Found empty string for refresh token",
                );
            }

            serializer
                .grant_type("refresh_token")
                .device_code(refresh_token.as_ref());

            return serializer.as_credential_map(
                vec![],
                vec![
                    AuthParameter::ClientId,
                    AuthParameter::RefreshToken,
                    AuthParameter::Scope,
                    AuthParameter::GrantType,
                ],
            );
        } else if let Some(device_code) = self.device_code.as_ref() {
            if device_code.trim().is_empty() {
                return AuthorizationFailure::msg_result(
                    AuthParameter::DeviceCode.alias(),
                    "Found empty string for device code",
                );
            }

            serializer
                .grant_type(DEVICE_CODE_GRANT_TYPE)
                .device_code(device_code.as_ref());

            return serializer.as_credential_map(
                vec![],
                vec![
                    AuthParameter::ClientId,
                    AuthParameter::DeviceCode,
                    AuthParameter::Scope,
                    AuthParameter::GrantType,
                ],
            );
        }

        serializer.as_credential_map(vec![], vec![AuthParameter::ClientId, AuthParameter::Scope])
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
        let mut interval = Duration::from_secs(device_code_response.interval);
        credential.with_device_code(device_code);

        let _ = std::thread::spawn(move || {
            loop {
                // Wait the amount of seconds that interval is.
                std::thread::sleep(interval);

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
                                PollDeviceCodeEvent::AuthorizationPending
                                | PollDeviceCodeEvent::BadVerificationCode => continue,
                                PollDeviceCodeEvent::AuthorizationDeclined
                                | PollDeviceCodeEvent::ExpiredToken
                                | PollDeviceCodeEvent::AccessDenied => break,
                                PollDeviceCodeEvent::SlowDown => {
                                    interval = interval.add(Duration::from_secs(5));
                                    continue;
                                }
                            },
                            Err(_) => {
                                error!(
                                    target = "device_code_polling_executor",
                                    "invalid PollDeviceCodeEvent"
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
            loop {
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
                                    // Should slow down is part of the openid connect spec and means that
                                    // that we should wait longer between polling by the amount specified
                                    // in the interval field of the device code.
                                    interval = interval.add(Duration::from_secs(5));
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
    pub fn with_interactive_auth(
        &mut self,
        options: WebViewOptions,
    ) -> AuthExecutionResult<(DeviceAuthorizationResponse, DeviceCodeInteractiveAuth)> {
        let response = self.credential.execute()?;
        let device_authorization_response: DeviceAuthorizationResponse = response.json()?;
        self.credential
            .with_device_code(device_authorization_response.device_code.clone());

        Ok((
            device_authorization_response.clone(),
            DeviceCodeInteractiveAuth {
                credential: self.credential.clone(),
                interval: Duration::from_secs(device_authorization_response.interval),
                verification_uri: device_authorization_response.verification_uri.clone(),
                verification_uri_complete: device_authorization_response.verification_uri_complete,
                options,
            },
        ))
    }
}

#[cfg(feature = "interactive-auth")]
pub(crate) mod internal {
    use super::*;

    impl WebViewAuth for DeviceCodeCredential {
        fn webview(
            host_options: HostOptions,
            window: &Window,
            _proxy: EventLoopProxy<UserEvents>,
        ) -> anyhow::Result<WebView> {
            Ok(WebViewBuilder::new(window)
                .with_url(host_options.start_uri.as_ref())
                // Disables file drop
                .with_file_drop_handler(|_| true)
                .with_navigation_handler(move |uri| {
                    tracing::debug!(target: INTERACTIVE_AUTH, url = uri.as_str());
                    true
                })
                .build()?)
        }
    }
}

#[cfg(feature = "interactive-auth")]
#[derive(Debug)]
pub struct DeviceCodeInteractiveAuth {
    credential: DeviceCodeCredential,
    interval: Duration,
    verification_uri: String,
    verification_uri_complete: Option<String>,
    options: WebViewOptions,
}

#[allow(dead_code)]
#[cfg(feature = "interactive-auth")]
impl DeviceCodeInteractiveAuth {
    pub(crate) fn new(
        credential: DeviceCodeCredential,
        device_authorization_response: DeviceAuthorizationResponse,
        options: WebViewOptions,
    ) -> DeviceCodeInteractiveAuth {
        DeviceCodeInteractiveAuth {
            credential,
            interval: Duration::from_secs(device_authorization_response.interval),
            verification_uri: device_authorization_response.verification_uri.clone(),
            verification_uri_complete: device_authorization_response.verification_uri_complete,
            options,
        }
    }

    pub fn poll(
        &mut self,
    ) -> Result<PublicClientApplication<DeviceCodeCredential>, WebViewDeviceCodeError> {
        let url = {
            if let Some(url_complete) = self.verification_uri_complete.as_ref() {
                Url::parse(url_complete).map_err(AuthorizationFailure::from)?
            } else {
                Url::parse(self.verification_uri.as_str()).map_err(AuthorizationFailure::from)?
            }
        };

        let (sender, _receiver) = std::sync::mpsc::channel();

        let options = self.options.clone();
        std::thread::spawn(move || {
            DeviceCodeCredential::run(url, vec![], options, sender).unwrap();
        });

        let credential = self.credential.clone();
        let interval = self.interval;
        DeviceCodeInteractiveAuth::poll_internal(interval, credential)
    }

    pub(crate) fn poll_internal(
        mut interval: Duration,
        mut credential: DeviceCodeCredential,
    ) -> Result<PublicClientApplication<DeviceCodeCredential>, WebViewDeviceCodeError> {
        loop {
            // Wait the amount of seconds that interval is.
            std::thread::sleep(interval);

            let response = credential.execute().unwrap();
            let http_response = response.into_http_response().map_err(Box::new)?;
            let status = http_response.status();

            if status.is_success() {
                return if let Some(json) = http_response.json() {
                    let token: Token = serde_json::from_value(json)
                        .map_err(|err| Box::new(AuthExecutionError::from(err)))?;
                    let cache_id = credential.app_config.cache_id.clone();
                    credential.token_cache.store(cache_id, token);
                    Ok(PublicClientApplication::from(credential))
                } else {
                    Err(WebViewDeviceCodeError::DeviceCodePollingError(
                        http_response,
                    ))
                };
            } else {
                let json = http_response.json().unwrap();
                let option_error = json["error"].as_str().map(|value| value.to_owned());

                if let Some(error) = option_error {
                    match PollDeviceCodeEvent::from_str(error.as_str()) {
                        Ok(poll_device_code_type) => match poll_device_code_type {
                            PollDeviceCodeEvent::AuthorizationPending
                            | PollDeviceCodeEvent::BadVerificationCode => continue,
                            PollDeviceCodeEvent::SlowDown => {
                                interval = interval.add(Duration::from_secs(5));
                                continue;
                            }
                            PollDeviceCodeEvent::AuthorizationDeclined
                            | PollDeviceCodeEvent::ExpiredToken
                            | PollDeviceCodeEvent::AccessDenied => {
                                return Err(WebViewDeviceCodeError::DeviceCodePollingError(
                                    http_response,
                                ));
                            }
                        },
                        Err(_) => {
                            return Err(WebViewDeviceCodeError::DeviceCodePollingError(
                                http_response,
                            ));
                        }
                    }
                } else {
                    // Body should have error or we should bail.
                    return Err(WebViewDeviceCodeError::DeviceCodePollingError(
                        http_response,
                    ));
                }
            }
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
