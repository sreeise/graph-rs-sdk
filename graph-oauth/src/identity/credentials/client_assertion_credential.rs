use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    Authority, AzureCloudInstance, TokenCredentialExecutor, CLIENT_ASSERTION_TYPE,
};
use crate::oauth::{ConfidentialClientApplication, OAuthParameter, OAuthSerializer};
use async_trait::async_trait;
use graph_error::{AuthorizationResult, AF};
use std::collections::HashMap;
use url::Url;

credential_builder!(
    ClientAssertionCredentialBuilder,
    ConfidentialClientApplication
);

#[derive(Clone)]
pub struct ClientAssertionCredential {
    /// The client (application) ID of the service principal
    pub(crate) app_config: AppConfig,
    /// The value passed for the scope parameter in this request should be the resource
    /// identifier (application ID URI) of the resource you want, affixed with the .default
    /// suffix. For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// Default is https://graph.microsoft.com/.default.
    pub(crate) scope: Vec<String>,
    pub(crate) client_assertion_type: String,
    pub(crate) client_assertion: String,
    pub(crate) refresh_token: Option<String>,
    serializer: OAuthSerializer,
}

impl ClientAssertionCredential {
    pub fn new(
        tenant_id: impl AsRef<str>,
        client_id: impl AsRef<str>,
    ) -> ClientAssertionCredential {
        ClientAssertionCredential {
            app_config: AppConfig::init(tenant_id, client_id),
            scope: vec!["https://graph.microsoft.com/.default".into()],
            client_assertion_type: CLIENT_ASSERTION_TYPE.to_owned(),
            client_assertion: Default::default(),
            refresh_token: None,
            serializer: Default::default(),
        }
    }
}

pub struct ClientAssertionCredentialBuilder {
    credential: ClientAssertionCredential,
}

impl ClientAssertionCredentialBuilder {
    pub(crate) fn new() -> ClientAssertionCredentialBuilder {
        ClientAssertionCredentialBuilder {
            credential: ClientAssertionCredential {
                app_config: Default::default(),
                scope: vec!["https://graph.microsoft.com/.default".into()],
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_string(),
                client_assertion: Default::default(),
                refresh_token: None,
                serializer: Default::default(),
            },
        }
    }

    pub(crate) fn new_with_signed_assertion(
        signed_assertion: String,
        app_config: AppConfig,
    ) -> ClientAssertionCredentialBuilder {
        ClientAssertionCredentialBuilder {
            credential: ClientAssertionCredential {
                app_config,
                scope: vec!["https://graph.microsoft.com/.default".into()],
                client_assertion_type: CLIENT_ASSERTION_TYPE.to_string(),
                client_assertion: signed_assertion,
                refresh_token: None,
                serializer: Default::default(),
            },
        }
    }

    pub fn with_client_assertion<T: AsRef<str>>(&mut self, client_assertion: T) -> &mut Self {
        self.credential.client_assertion = client_assertion.as_ref().to_owned();
        self
    }

    pub fn with_refresh_token<T: AsRef<str>>(&mut self, refresh_token: T) -> &mut Self {
        self.credential.refresh_token = Some(refresh_token.as_ref().to_owned());
        self
    }
}

#[async_trait]
impl TokenCredentialExecutor for ClientAssertionCredential {
    fn uri(&mut self, azure_authority_host: &AzureCloudInstance) -> AuthorizationResult<Url> {
        self.serializer
            .authority(azure_authority_host, &self.authority());

        let uri = self
            .serializer
            .get(OAuthParameter::TokenUrl)
            .ok_or(AF::msg_err("token_url", "Internal Error"))?;
        Url::parse(uri.as_str()).map_err(AF::from)
    }

    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>> {
        let client_id = self.client_id().clone();
        if client_id.trim().is_empty() {
            return AF::result(OAuthParameter::ClientId.alias());
        }

        if self.client_assertion.trim().is_empty() {
            return AF::result(OAuthParameter::ClientAssertion.alias());
        }

        if self.client_assertion_type.trim().is_empty() {
            self.client_assertion_type = CLIENT_ASSERTION_TYPE.to_owned();
        }

        self.serializer
            .client_id(client_id.as_str())
            .client_assertion(self.client_assertion.as_str())
            .client_assertion_type(self.client_assertion_type.as_str());

        if self.scope.is_empty() {
            self.serializer
                .add_scope("https://graph.microsoft.com/.default");
        }

        return if let Some(refresh_token) = self.refresh_token.as_ref() {
            if refresh_token.trim().is_empty() {
                return AF::msg_result(
                    OAuthParameter::RefreshToken.alias(),
                    "refresh_token is set but is empty",
                );
            }

            self.serializer
                .refresh_token(refresh_token.as_ref())
                .grant_type("refresh_token");

            self.serializer.as_credential_map(
                vec![OAuthParameter::Scope],
                vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::GrantType,
                    OAuthParameter::ClientAssertion,
                    OAuthParameter::ClientAssertionType,
                    OAuthParameter::RefreshToken,
                ],
            )
        } else {
            self.serializer.grant_type("client_credentials");

            self.serializer.as_credential_map(
                vec![OAuthParameter::Scope],
                vec![
                    OAuthParameter::ClientId,
                    OAuthParameter::GrantType,
                    OAuthParameter::ClientAssertion,
                    OAuthParameter::ClientAssertionType,
                ],
            )
        };
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
