use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::form_credential::SerializerField;
use crate::identity::{Authority, AuthorizationSerializer, AzureAuthorityHost};
use graph_error::{AuthorizationFailure, AuthorizationResult};
use std::collections::HashMap;
use url::Url;

/// Legacy sign in for personal microsoft accounts to get access tokens for OneDrive
/// Not recommended - Instead use Microsoft Identity Platform OAuth 2.0 and OpenId Connect.
/// https://learn.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online#code-flow
#[derive(Clone, Eq, PartialEq)]
pub struct CodeFlowCredential {
    /// Required unless requesting a refresh token
    /// The authorization code obtained from a call to authorize.
    /// The code should be obtained with all required scopes.
    pub(crate) authorization_code: Option<String>,
    /// Required when requesting a new access token using a refresh token
    /// The refresh token needed to make an access token request using a refresh token.
    /// Do not include an authorization code when using a refresh token.
    pub(crate) refresh_token: Option<String>,
    /// Required.
    /// The Application (client) ID that the Azure portal - App registrations page assigned
    /// to your app
    pub(crate) client_id: String,
    /// Required
    /// The application secret that you created in the app registration portal for your app.
    /// Don't use the application secret in a native app or single page app because a
    /// client_secret can't be reliably stored on devices or web pages. It's required for web
    /// apps and web APIs, which can store the client_secret securely on the server side. Like
    /// all parameters here, the client secret must be URL-encoded before being sent. This step
    /// is done by the SDK. For more information on URI encoding, see the URI Generic Syntax
    /// specification. The Basic auth pattern of instead providing credentials in the Authorization
    /// header, per RFC 6749 is also supported.
    pub(crate) client_secret: String,
    /// The same redirect_uri value that was used to acquire the authorization_code.
    pub(crate) redirect_uri: String,
    serializer: OAuthSerializer,
}

impl CodeFlowCredential {
    pub fn new<T: AsRef<str>>(
        client_id: T,
        client_secret: T,
        authorization_code: T,
        redirect_uri: T,
    ) -> CodeFlowCredential {
        CodeFlowCredential {
            authorization_code: Some(authorization_code.as_ref().to_owned()),
            refresh_token: None,
            client_id: client_id.as_ref().to_owned(),
            client_secret: client_secret.as_ref().to_owned(),
            redirect_uri: redirect_uri.as_ref().to_owned(),
            serializer: OAuthSerializer::new(),
        }
    }

    pub fn builder() -> CodeFlowCredentialBuilder {
        CodeFlowCredentialBuilder::new()
    }
}

impl AuthorizationSerializer for CodeFlowCredential {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url> {
        if azure_authority_host.ne(&AzureAuthorityHost::OneDriveAndSharePoint) {
            return AuthorizationFailure::required_value_msg_result(
                "uri",
                Some("Code flow can only be used with AzureAuthorityHost::OneDriveAndSharePoint"),
            );
        }

        self.serializer
            .authority(azure_authority_host, &Authority::Common);

        if self.refresh_token.is_none() {
            let uri = self.serializer.get(OAuthParameter::AccessTokenUrl).ok_or(
                AuthorizationFailure::required_value_msg(
                    "access_token_url",
                    Some("Internal Error"),
                ),
            )?;
            Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
        } else {
            let uri = self.serializer.get(OAuthParameter::RefreshTokenUrl).ok_or(
                AuthorizationFailure::required_value_msg(
                    "refresh_token_url",
                    Some("Internal Error"),
                ),
            )?;
            Url::parse(uri.as_str()).map_err(AuthorizationFailure::from)
        }
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        if self.authorization_code.is_some() && self.refresh_token.is_some() {
            return AuthorizationFailure::required_value_msg_result(
                &format!(
                    "{} or {}",
                    OAuthParameter::AuthorizationCode.alias(),
                    OAuthParameter::RefreshToken.alias()
                ),
                Some("Authorization code and refresh token should not be set at the same time - Internal Error"),
            );
        }

        if self.client_id.trim().is_empty() {
            return AuthorizationFailure::required_value_result(OAuthParameter::ClientId.alias());
        }

        if self.client_secret.trim().is_empty() {
            return AuthorizationFailure::required_value_result(
                OAuthParameter::ClientSecret.alias(),
            );
        }

        if self.redirect_uri.trim().is_empty() {
            return AuthorizationFailure::required_value_result(OAuthParameter::RedirectUri);
        }

        self.serializer
            .client_id(self.client_id.as_str())
            .client_secret(self.client_secret.as_str())
            .redirect_uri(self.redirect_uri.as_str())
            .legacy_authority();

        if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AuthorizationFailure::required_value_msg_result(
                    OAuthParameter::RefreshToken.alias(),
                    Some("Either authorization code or refresh token is required"),
                );
            }

            self.serializer.refresh_token(refresh_token.as_ref());

            return self.serializer.authorization_form(vec![
                SerializerField::Required(OAuthParameter::ClientId),
                SerializerField::Required(OAuthParameter::ClientSecret),
                SerializerField::Required(OAuthParameter::RefreshToken),
                SerializerField::Required(OAuthParameter::RedirectUri),
            ]);
        } else if let Some(authorization_code) = self.authorization_code.as_ref() {
            if authorization_code.trim().is_empty() {
                return AuthorizationFailure::required_value_msg_result(
                    OAuthParameter::RefreshToken.alias(),
                    Some("Either authorization code or refresh token is required"),
                );
            }

            self.serializer
                .authorization_code(authorization_code.as_ref());

            return self.serializer.authorization_form(vec![
                SerializerField::Required(OAuthParameter::ClientId),
                SerializerField::Required(OAuthParameter::ClientSecret),
                SerializerField::Required(OAuthParameter::RedirectUri),
                SerializerField::Required(OAuthParameter::AuthorizationCode),
            ]);
        }

        AuthorizationFailure::required_value_msg_result(
            &format!(
                "{} or {}",
                OAuthParameter::AuthorizationCode.alias(),
                OAuthParameter::RefreshToken.alias()
            ),
            Some("Either authorization code or refresh token is required"),
        )
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct CodeFlowCredentialBuilder {
    credential: CodeFlowCredential,
}

impl CodeFlowCredentialBuilder {
    fn new() -> CodeFlowCredentialBuilder {
        CodeFlowCredentialBuilder {
            credential: CodeFlowCredential {
                authorization_code: None,
                refresh_token: None,
                client_id: String::new(),
                client_secret: String::new(),
                redirect_uri: String::new(),
                serializer: Default::default(),
            },
        }
    }

    pub fn with_authorization_code<T: AsRef<str>>(&mut self, authorization_code: T) -> &mut Self {
        self.credential.refresh_token = None;
        self.credential.authorization_code = Some(authorization_code.as_ref().to_owned());
        self
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.credential.authorization_code = None;
        self.credential.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }

    pub fn with_redirect_uri<T: AsRef<str>>(&mut self, redirect_uri: T) -> &mut Self {
        self.credential.redirect_uri = redirect_uri.as_ref().to_owned();
        self
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.credential.client_id = client_id.as_ref().to_owned();
        self
    }

    pub fn with_client_secret<T: AsRef<str>>(&mut self, client_secret: T) -> &mut Self {
        self.credential.client_secret = client_secret.as_ref().to_owned();
        self
    }

    pub fn build(&self) -> CodeFlowCredential {
        self.credential.clone()
    }
}

impl Default for CodeFlowCredentialBuilder {
    fn default() -> Self {
        CodeFlowCredentialBuilder::new()
    }
}
