use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{Authority, AzureCloudInstance, TokenCredentialExecutor};
use crate::oauth::{DeviceCode, PollDeviceCodeType, PublicClientApplication};

use graph_error::{
    AuthExecutionError, AuthExecutionResult, AuthorizationFailure, AuthorizationResult, AF,
};
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::ops::Add;
use std::str::FromStr;
use std::time::Duration;

use crate::identity::credentials::app_config::AppConfig;

use url::Url;

const DEVICE_CODE_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:device_code";

credential_builder!(DeviceCodeCredentialBuilder, PublicClientApplication);

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
    /// A space-separated list of scopes. The scopes must all be from a single resource,
    /// along with OIDC scopes (profile, openid, email). For more information, see Permissions
    /// and consent in the Microsoft identity platform. This parameter is a Microsoft extension
    /// to the authorization code flow, intended to allow apps to declare the resource they want
    /// the token for during token redemption.
    pub(crate) scope: Vec<String>,
    serializer: OAuthSerializer,
}

impl DeviceCodeCredential {
    pub fn new<T: AsRef<str>, U: ToString, I: IntoIterator<Item = U>>(
        client_id: T,
        device_code: T,
        scope: I,
    ) -> DeviceCodeCredential {
        DeviceCodeCredential {
            app_config: AppConfig::new_with_client_id(client_id),
            refresh_token: None,
            device_code: Some(device_code.as_ref().to_owned()),
            scope: scope.into_iter().map(|s| s.to_string()).collect(),
            serializer: Default::default(),
        }
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.device_code = None;
        self.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }

    pub fn with_device_code<T: AsRef<str>>(&mut self, device_code: T) -> &mut Self {
        self.refresh_token = None;
        self.device_code = Some(device_code.as_ref().to_owned());
        self
    }

    pub async fn poll_async(
        &mut self,
        buffer: Option<usize>,
    ) -> AuthExecutionResult<tokio::sync::mpsc::Receiver<http::Response<serde_json::Value>>> {
        let (sender, receiver) = {
            if let Some(buffer) = buffer {
                tokio::sync::mpsc::channel(buffer)
            } else {
                tokio::sync::mpsc::channel(100)
            }
        };

        let mut credential = self.clone();
        let result = credential
            .execute_async()
            .await
            .map_err(AuthExecutionError::from);

        match result {
            Ok(response) => {
                let device_code_response: DeviceCode = response.json().await.unwrap();
                println!("{:#?}", device_code_response);

                let device_code = device_code_response.device_code;
                let interval = device_code_response.interval;
                let _message = device_code_response.message;
                credential.with_device_code(device_code);

                tokio::spawn(async move {
                    let mut should_slow_down = false;

                    loop {
                        // Wait the amount of seconds that interval is.
                        if should_slow_down {
                            std::thread::sleep(interval.add(Duration::from_secs(5)));
                        } else {
                            std::thread::sleep(interval);
                        }

                        let response = credential.execute_async().await.unwrap();

                        let status = response.status();
                        println!("{response:#?}");

                        let body: serde_json::Value = response.json().await.unwrap();
                        println!("{body:#?}");

                        if status.is_success() {
                            sender
                                .send_timeout(
                                    http::Response::builder().status(status).body(body).unwrap(),
                                    Duration::from_secs(60),
                                )
                                .await
                                .unwrap();
                        } else {
                            let option_error = body["error"].as_str().map(|value| value.to_owned());
                            sender
                                .send_timeout(
                                    http::Response::builder().status(status).body(body).unwrap(),
                                    Duration::from_secs(60),
                                )
                                .await
                                .unwrap();

                            if let Some(error) = option_error {
                                match PollDeviceCodeType::from_str(error.as_str()) {
                                    Ok(poll_device_code_type) => match poll_device_code_type {
                                        PollDeviceCodeType::AuthorizationPending => continue,
                                        PollDeviceCodeType::AuthorizationDeclined => break,
                                        PollDeviceCodeType::BadVerificationCode => continue,
                                        PollDeviceCodeType::ExpiredToken => break,
                                        PollDeviceCodeType::InvalidType => break,
                                        PollDeviceCodeType::AccessDenied => break,
                                        PollDeviceCodeType::SlowDown => {
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
                });
            }
            Err(err) => return Err(err),
        }

        Ok(receiver)
    }

    pub fn builder() -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder::new()
    }
}

impl TokenCredentialExecutor for DeviceCodeCredential {
    fn uri(&mut self, azure_cloud_instance: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.serializer
            .authority_device_code(azure_cloud_instance, &self.app_config.authority);

        if self.device_code.is_none() && self.refresh_token.is_none() {
            let uri = self
                .serializer
                .get(OAuthParameter::AuthorizationUrl)
                .ok_or(AF::msg_internal_err("authorization_url"))?;
            Url::parse(uri.as_str()).map_err(|_err| AF::msg_internal_err("authorization_url"))
        } else {
            let uri = self
                .serializer
                .get(OAuthParameter::TokenUrl)
                .ok_or(AF::msg_internal_err("token_url"))?;
            Url::parse(uri.as_str()).map_err(|_err| AF::msg_internal_err("token_url"))
        }
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        let client_id = self.app_config.client_id.trim();
        if client_id.is_empty() {
            return AuthorizationFailure::result(OAuthParameter::ClientId.alias());
        }

        self.serializer
            .client_id(client_id)
            .extend_scopes(self.scope.clone());

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AuthorizationFailure::msg_result(
                    OAuthParameter::RefreshToken.alias(),
                    "Refresh token string is empty - Either device code or refresh token is required",
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
                    "Either device code or refresh token is required - found empty device code",
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

    fn client_id(&self) -> &String {
        &self.app_config.client_id
    }

    fn authority(&self) -> Authority {
        self.app_config.authority.clone()
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
    fn new() -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder {
            credential: DeviceCodeCredential {
                app_config: Default::default(),
                refresh_token: None,
                device_code: None,
                scope: vec![],
                serializer: Default::default(),
            },
        }
    }

    pub(crate) fn new_with_app_config(app_config: AppConfig) -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder {
            credential: DeviceCodeCredential {
                app_config,
                refresh_token: None,
                device_code: None,
                scope: vec![],
                serializer: Default::default(),
            },
        }
    }

    pub(crate) fn new_with_device_code<T: AsRef<str>>(
        device_code: T,
        app_config: AppConfig,
    ) -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder {
            credential: DeviceCodeCredential {
                app_config,
                refresh_token: None,
                device_code: Some(device_code.as_ref().to_owned()),
                scope: vec![],
                serializer: Default::default(),
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

    /*
     pub fn build(&self) -> PublicClientApplication {
        PublicClientApplication::from(self.credential.clone())
    }
     */
}

impl From<&DeviceCode> for DeviceCodeCredentialBuilder {
    fn from(value: &DeviceCode) -> Self {
        DeviceCodeCredentialBuilder {
            credential: DeviceCodeCredential {
                app_config: AppConfig::new(),
                refresh_token: None,
                device_code: Some(value.device_code.clone()),
                scope: vec![],
                serializer: Default::default(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn no_scope() {
        let mut credential = DeviceCodeCredential::builder()
            .with_client_id("CLIENT_ID")
            .build();

        let _ = credential.form_urlencode().unwrap();
    }
}
