use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{Authority, AzureCloudInstance, TokenCredentialExecutor};
use crate::oauth::{DeviceCode, PublicClientApplication};
use graph_error::{AuthorizationFailure, AuthorizationResult, AF, GraphFailure, GraphResult};
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::time::Duration;
use anyhow::anyhow;

use crate::identity::credentials::app_config::AppConfig;
use url::Url;

/*
fn response_to_http_response(response: reqwest::Response) -> anyhow::Result<http::Response<>> {
    let status = response.status();
    let url = response.url().clone();
    let headers = response.headers().clone();
    let version = response.version();

    let body: serde_json::Value = response.json().await?;
    let next_link = body.odata_next_link();
    let json = body.clone();
    let body_result: Result<T, ErrorMessage> = serde_json::from_value(body)
        .map_err(|_| serde_json::from_value(json.clone()).unwrap_or(ErrorMessage::default()));

    let mut builder = http::Response::builder()
        .url(url)
        .json(&json)
        .status(http::StatusCode::from(&status))
        .version(version);

    for builder_header in builder.headers_mut().iter_mut() {
        builder_header.extend(headers.clone());
    }

    Ok(builder.body(body_result))
}
 */

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

/*
    pub async fn poll_async(&mut self, buffer: Option<usize>) -> tokio::sync::mpsc::Receiver<anyhow::Result<http::Response<serde_json::Value>>> {
        let (sender, receiver) = {
            if let Some(buffer) = buffer {
                tokio::sync::mpsc::channel(buffer)
            }  else {
                tokio::sync::mpsc::channel(100)
            }
        };

        let mut credential = self.clone();
        let mut application = PublicClientApplication::from(self.clone());

        tokio::spawn(async move {
            let response = application.execute_async().await.map_err(|err| anyhow!(err));

            match response {
                Ok(response) => {
                    let status = response.status();

                    let body: serde_json::Value = response.json().await.unwrap();
                    println!("{body:#?}");

                    let device_code = body["device_code"].as_str().unwrap();
                    let interval = body["interval"].as_u64().unwrap();
                    let message = body["message"].as_str().unwrap();
                    credential.with_device_code(device_code);
                    let mut application = PublicClientApplication::from(credential);

                    if !status.is_success() {
                        loop {
                            // Wait the amount of seconds that interval is.
                            std::thread::sleep(Duration::from_secs(interval.clone()));

                            let response = application.execute_async().await.unwrap();

                            let status = response.status();
                            println!("{response:#?}");

                            let body: serde_json::Value = response.json().await.unwrap();
                            println!("{body:#?}");

                            if status.is_success() {
                                sender.send_timeout(Ok(body), Duration::from_secs(60));
                            } else {
                                let option_error = body["error"].as_str();

                                if let Some(error) = option_error {
                                    match error {
                                        "authorization_pending" => println!("Still waiting on user to sign in"),
                                        "authorization_declined" => panic!("user declined to sign in"),
                                        "bad_verification_code" => println!("Bad verification code. Message:\n{message:#?}"),
                                        "expired_token" => panic!("token has expired - user did not sign in"),
                                        _ => {
                                            panic!("This isn't the error we expected: {error:#?}");
                                        }
                                    }
                                } else {
                                    // Body should have error or we should bail.
                                    panic!("Crap hit the fan");
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    sender.send_timeout(Err(err), Duration::from_secs(60)).await;
                }
            }
        });

        return receiver;
    }
 */

    pub fn builder() -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder::new()
    }
}

impl TokenCredentialExecutor for DeviceCodeCredential {
    fn uri(&mut self, azure_authority_host: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.app_config.authority);

        if self.refresh_token.is_none() {
            let uri = self
                .serializer
                .get(OAuthParameter::TokenUrl)
                .ok_or(AF::msg_internal_err("access_token_url"))?;
            Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
        } else {
            let uri = self
                .serializer
                .get(OAuthParameter::RefreshTokenUrl)
                .ok_or(AF::msg_internal_err("refresh_token_url"))?;
            Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
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

        self.serializer.grant_type(DEVICE_CODE_GRANT_TYPE);

        self.serializer.as_credential_map(
            vec![],
            vec![
                OAuthParameter::ClientId,
                OAuthParameter::Scope,
                OAuthParameter::GrantType,
            ],
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
