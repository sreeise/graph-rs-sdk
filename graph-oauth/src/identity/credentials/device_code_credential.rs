use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::{
    Authority, AuthorizationSerializer, AzureAuthorityHost, TokenCredential, TokenCredentialOptions,
};
use crate::oauth::DeviceCode;
use graph_error::{AuthorizationFailure, AuthorizationResult};
use std::collections::HashMap;
use url::Url;

static DEVICE_CODE_GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:device_code";

/// Allows users to sign in to input-constrained devices such as a smart TV, IoT device,
/// or a printer. To enable this flow, the device has the user visit a webpage in a browser on
/// another device to sign in. Once the user signs in, the device is able to get access tokens
/// and refresh tokens as needed.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code
#[derive(Clone)]
pub struct DeviceCodeCredential {
    /// Required when requesting a new access token using a refresh token
    /// The refresh token needed to make an access token request using a refresh token.
    /// Do not include an authorization code when using a refresh token.
    pub(crate) refresh_token: Option<String>,
    /// Required.
    /// The Application (client) ID that the Azure portal - App registrations page assigned
    /// to your app
    pub(crate) client_id: String,
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
    /// The Azure Active Directory tenant (directory) Id of the service principal.
    pub(crate) authority: Authority,
    pub(crate) token_credential_options: TokenCredentialOptions,
    serializer: OAuthSerializer,
}

impl DeviceCodeCredential {
    pub fn new<T: AsRef<str>, U: ToString, I: IntoIterator<Item = U>>(
        client_id: T,
        device_code: T,
        scope: I,
    ) -> DeviceCodeCredential {
        DeviceCodeCredential {
            refresh_token: None,
            client_id: client_id.as_ref().to_owned(),
            device_code: Some(device_code.as_ref().to_owned()),
            scope: scope.into_iter().map(|s| s.to_string()).collect(),
            authority: Default::default(),
            token_credential_options: Default::default(),
            serializer: Default::default(),
        }
    }

    pub fn builder() -> DeviceCodeCredentialBuilder {
        DeviceCodeCredentialBuilder::new()
    }
}

impl AuthorizationSerializer for DeviceCodeCredential {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.authority);

        if self.refresh_token.is_none() {
            let uri = self.serializer.get(OAuthParameter::AccessTokenUrl).ok_or(
                AuthorizationFailure::msg_err("access_token_url", "Internal Error"),
            )?;
            Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
        } else {
            let uri = self.serializer.get(OAuthParameter::RefreshTokenUrl).ok_or(
                AuthorizationFailure::msg_err("refresh_token_url", "Internal Error"),
            )?;
            Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
        }
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        if self.device_code.is_some() && self.refresh_token.is_some() {
            return AuthorizationFailure::msg_result(
                format!(
                    "{} or {}",
                    OAuthParameter::DeviceCode.alias(),
                    OAuthParameter::RefreshToken.alias()
                ),
                "Device code and refresh token should not be set at the same time - Internal Error",
            );
        }

        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::result(OAuthParameter::ClientId.alias());
        }

        self.serializer
            .client_id(self.client_id.as_str())
            .extend_scopes(self.scope.clone());

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AuthorizationFailure::msg_result(
                    OAuthParameter::RefreshToken.alias(),
                    "Either device code or refresh token is required - found empty refresh token",
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
                    OAuthParameter::ClientSecret,
                    OAuthParameter::Scope,
                    OAuthParameter::GrantType,
                ],
            );
        }

        AuthorizationFailure::msg_result(
            format!(
                "{} or {}",
                OAuthParameter::DeviceCode.alias(),
                OAuthParameter::RefreshToken.alias()
            ),
            "Either device code or refresh token is required",
        )
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
                refresh_token: None,
                client_id: String::new(),
                device_code: None,
                scope: vec![],
                authority: Default::default(),
                token_credential_options: Default::default(),
                serializer: Default::default(),
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.credential.client_id = client_id.as_ref().to_owned();
        self
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

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.credential.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.credential.authority = authority.into();
        self
    }

    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.credential.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn with_token_credential_options(
        &mut self,
        token_credential_options: TokenCredentialOptions,
    ) {
        self.credential.token_credential_options = token_credential_options;
    }

    pub fn build(&self) -> DeviceCodeCredential {
        self.credential.clone()
    }
}

impl From<&DeviceCode> for DeviceCodeCredentialBuilder {
    fn from(value: &DeviceCode) -> Self {
        DeviceCodeCredentialBuilder {
            credential: DeviceCodeCredential {
                refresh_token: None,
                client_id: String::new(),
                device_code: Some(value.device_code.clone()),
                scope: vec![],
                authority: Default::default(),
                token_credential_options: Default::default(),
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
    fn no_device_code() {
        let mut credential = DeviceCodeCredential::builder()
            .with_client_id("CLIENT_ID")
            .with_scope(vec!["scope"])
            .build();

        let _ = credential.form_urlencode().unwrap();
    }
}
