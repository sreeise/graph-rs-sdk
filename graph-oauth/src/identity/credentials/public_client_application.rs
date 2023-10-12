use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::tls::Version;
use reqwest::{ClientBuilder, Response};
use url::Url;
use uuid::Uuid;

use graph_error::{AuthExecutionResult, IdentityResult};
use graph_extensions::token::ClientApplication;

use crate::identity::credentials::app_config::AppConfig;
use crate::identity::credentials::application_builder::PublicClientApplicationBuilder;
use crate::identity::{
    Authority, AzureCloudInstance, DeviceCodeCredential, ResourceOwnerPasswordCredential,
    TokenCredentialExecutor,
};

/// Clients incapable of maintaining the confidentiality of their credentials
/// (e.g., clients executing on the device used by the resource owner, such as an
/// installed native application or a web browser-based application), and incapable of
/// secure client authentication via any other means.
/// https://datatracker.ietf.org/doc/html/rfc6749#section-2.1
#[derive(Clone)]
pub struct PublicClientApplication {
    http_client: reqwest::Client,
    credential: Box<dyn TokenCredentialExecutor + Send>,
    //token_store: Arc<RwLock<dyn TokenStore + Send>>,
}

impl Debug for PublicClientApplication {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfidentialClientApplication")
            .field("credential", &self.credential)
            .finish()
    }
}

impl PublicClientApplication {
    pub fn new<T>(credential: T) -> PublicClientApplication
    where
        T: Into<PublicClientApplication>,
    {
        credential.into()
    }

    pub(crate) fn credential<T>(credential: T) -> PublicClientApplication
    where
        T: TokenCredentialExecutor + Send + 'static,
    {
        PublicClientApplication {
            http_client: ClientBuilder::new()
                .min_tls_version(Version::TLS_1_2)
                .https_only(true)
                .build()
                .unwrap(),
            credential: Box::new(credential),
            //token_store: Arc::new(RwLock::new(UnInitializedTokenStore)),
        }
    }

    pub fn builder(client_id: impl AsRef<str>) -> PublicClientApplicationBuilder {
        PublicClientApplicationBuilder::new(client_id.as_ref())
    }

    /*
     pub fn with_in_memory_token_store(&mut self) {
        self.token_store = Arc::new(RwLock::new(InMemoryCredentialStore::new(
            self.app_config().cache_id(),
            StoredToken::UnInitialized,
        )));
    }
     */
}

#[async_trait]
impl ClientApplication for PublicClientApplication {
    fn get_token_silent(&mut self) -> AuthExecutionResult<String> {
        todo!()
    }

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String> {
        todo!()
    }
}
/*
fn get_token_silent(&mut self) -> AuthExecutionResult<String> {
        let cache_id = self.app_config().cache_id();
        if self.is_store_and_token_initialized(cache_id.as_str()) {
            return Ok(self
                .get_bearer_token_from_store(cache_id.as_str())
                .ok_or(AF::unknown(
                    "Unknown error getting token from store - please report issue",
                ))?
                .clone());
        }

        if !self.is_token_store_initialized() {
            self.with_in_memory_token_store();
        }

        let response = self.execute()?;
        let msal_token: MsalToken = response.json()?;
        self.update_stored_token(cache_id.as_str(), StoredToken::MsalToken(msal_token));
        Ok(self
            .get_bearer_token_from_store(cache_id.as_str())
            .ok_or(AF::unknown(
                "Unknown error initializing token store - please report issue",
            ))?
            .clone())
    }

    async fn get_token_silent_async(&mut self) -> AuthExecutionResult<String> {
        let cache_id = self.app_config().cache_id();
        if self.is_store_and_token_initialized(cache_id.as_str()) {
            return Ok(self
                .get_bearer_token_from_store(cache_id.as_str())
                .ok_or(AF::unknown(
                    "Unknown error getting token from store - please report issue",
                ))?
                .clone());
        }

        if !self.is_token_store_initialized() {
            self.with_in_memory_token_store();
        }

        let response = self.execute_async().await?;
        let msal_token: MsalToken = response.json().await?;
        self.update_stored_token(cache_id.as_str(), StoredToken::MsalToken(msal_token));
        Ok(self
            .get_bearer_token_from_store(cache_id.as_str())
            .ok_or(AF::unknown(
                "Unknown error initializing token store - please report issue",
            ))?
            .clone())
    }

    fn get_stored_application_token(&mut self) -> Option<&StoredToken> {
        let cache_id = self.app_config().cache_id();
        if !self.is_store_and_token_initialized(cache_id.as_str()) {
            self.get_token_silent().ok()?;
        }

        self.token_store.get_stored_token(cache_id.as_str())
    }
 */

/*
impl TokenStore for PublicClientApplication {
    fn token_store_provider(&self) -> TokenStoreProvider {
        self.token_store.read().unwrap().token_store_provider()
    }

    fn is_stored_token_initialized(&self, id: &str) -> bool {
        self.token_store.read().unwrap().is_stored_token_initialized(id)
    }

    fn get_stored_token(&self, id: &str) -> Option<&StoredToken> {
        self.token_store.read().unwrap().get_stored_token(id)
    }

    fn update_stored_token(&mut self, id: &str, stored_token: StoredToken) -> Option<StoredToken> {
        *self.token_store.write().unwrap().update_stored_token(id, stored_token)
    }

    fn get_bearer_token_from_store(&self, id: &str) -> Option<&String> {
        self.token_store.read().unwrap().get_bearer_token_from_store(id)
    }

    fn get_refresh_token_from_store(&self, id: &str) -> Option<&String> {
        self.token_store.read().unwrap().get_refresh_token_from_store(id)
    }
}
*/
#[async_trait]
impl TokenCredentialExecutor for PublicClientApplication {
    fn uri(&mut self) -> IdentityResult<Url> {
        self.credential.uri()
    }

    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>> {
        self.credential.form_urlencode()
    }

    fn client_id(&self) -> &Uuid {
        self.credential.client_id()
    }

    fn authority(&self) -> Authority {
        self.credential.authority()
    }

    fn azure_cloud_instance(&self) -> AzureCloudInstance {
        self.credential.azure_cloud_instance()
    }

    fn app_config(&self) -> &AppConfig {
        self.credential.app_config()
    }

    fn execute(&mut self) -> AuthExecutionResult<reqwest::blocking::Response> {
        let uri = self.credential.uri()?;

        let form = self.credential.form_urlencode()?;
        let http_client = reqwest::blocking::ClientBuilder::new()
            .min_tls_version(Version::TLS_1_2)
            .https_only(true)
            .build()?;
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let basic_auth = self.credential.basic_auth();
        if let Some((client_identifier, secret)) = basic_auth {
            Ok(http_client
                .post(uri)
                .basic_auth(client_identifier, Some(secret))
                .headers(headers)
                .form(&form)
                .send()?)
        } else {
            Ok(http_client.post(uri).headers(headers).form(&form).send()?)
        }
    }

    async fn execute_async(&mut self) -> AuthExecutionResult<Response> {
        let uri = self.credential.uri()?;

        let form = self.credential.form_urlencode()?;
        let basic_auth = self.credential.basic_auth();
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        if let Some((client_identifier, secret)) = basic_auth {
            Ok(self
                .http_client
                .post(uri)
                // https://datatracker.ietf.org/doc/html/rfc6749#section-2.3.1
                .basic_auth(client_identifier, Some(secret))
                .headers(headers)
                .form(&form)
                .send()
                .await?)
        } else {
            Ok(self
                .http_client
                .post(uri)
                .headers(headers)
                .form(&form)
                .send()
                .await?)
        }
    }
}

impl From<ResourceOwnerPasswordCredential> for PublicClientApplication {
    fn from(value: ResourceOwnerPasswordCredential) -> Self {
        PublicClientApplication::credential(value)
    }
}

impl From<DeviceCodeCredential> for PublicClientApplication {
    fn from(value: DeviceCodeCredential) -> Self {
        PublicClientApplication::credential(value)
    }
}
