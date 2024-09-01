use std::env::VarError;
use std::fmt::{Debug, Formatter};

use crate::identity::{
    ClientSecretCredential, ConfidentialClientApplication, PublicClientApplication,
    ResourceOwnerPasswordCredential,
};

const AZURE_TENANT_ID: &str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID: &str = "AZURE_CLIENT_ID";
const AZURE_CLIENT_SECRET: &str = "AZURE_CLIENT_SECRET";
const AZURE_USERNAME: &str = "AZURE_USERNAME";
const AZURE_PASSWORD: &str = "AZURE_PASSWORD";

#[derive(Clone)]
pub struct EnvironmentCredential;

impl Debug for EnvironmentCredential {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EnvironmentCredential").finish()
    }
}

impl EnvironmentCredential {
    pub fn resource_owner_password_credential(
    ) -> Result<PublicClientApplication<ResourceOwnerPasswordCredential>, VarError> {
        match EnvironmentCredential::try_username_password_compile_time_env() {
            Ok(credential) => Ok(credential),
            Err(_) => EnvironmentCredential::try_username_password_runtime_env(),
        }
    }

    pub fn client_secret_credential(
    ) -> Result<ConfidentialClientApplication<ClientSecretCredential>, VarError> {
        match EnvironmentCredential::try_azure_client_secret_compile_time_env() {
            Ok(credential) => Ok(credential),
            Err(_) => EnvironmentCredential::try_azure_client_secret_runtime_env(),
        }
    }

    fn try_azure_client_secret_compile_time_env(
    ) -> Result<ConfidentialClientApplication<ClientSecretCredential>, VarError> {
        let tenant_id = option_env!("AZURE_TENANT_ID");
        let azure_client_id = option_env!("AZURE_CLIENT_ID").ok_or(VarError::NotPresent)?;
        let azure_client_secret = option_env!("AZURE_CLIENT_SECRET").ok_or(VarError::NotPresent)?;
        EnvironmentCredential::client_secret_env(
            tenant_id.map(|s| s.to_owned()),
            azure_client_id.to_owned(),
            azure_client_secret.to_owned(),
        )
    }

    fn try_azure_client_secret_runtime_env(
    ) -> Result<ConfidentialClientApplication<ClientSecretCredential>, VarError> {
        let tenant_id = std::env::var(AZURE_TENANT_ID).ok();
        let azure_client_id = std::env::var(AZURE_CLIENT_ID)?;
        let azure_client_secret = std::env::var(AZURE_CLIENT_SECRET)?;
        EnvironmentCredential::client_secret_env(tenant_id, azure_client_id, azure_client_secret)
    }

    fn client_secret_env(
        tenant_id: Option<String>,
        azure_client_id: String,
        azure_client_secret: String,
    ) -> Result<ConfidentialClientApplication<ClientSecretCredential>, VarError> {
        match tenant_id {
            Some(tenant_id) => Ok(ConfidentialClientApplication::credential(
                ClientSecretCredential::new_with_tenant(
                    tenant_id,
                    azure_client_id,
                    azure_client_secret,
                ),
            )),
            None => Ok(ConfidentialClientApplication::credential(
                ClientSecretCredential::new(azure_client_id, azure_client_secret),
            )),
        }
    }

    fn try_username_password_compile_time_env(
    ) -> Result<PublicClientApplication<ResourceOwnerPasswordCredential>, VarError> {
        let tenant_id = option_env!("AZURE_TENANT_ID");
        let azure_client_id = option_env!("AZURE_CLIENT_ID").ok_or(VarError::NotPresent)?;
        let azure_username = option_env!("AZURE_USERNAME").ok_or(VarError::NotPresent)?;
        let azure_password = option_env!("AZURE_PASSWORD").ok_or(VarError::NotPresent)?;
        Ok(EnvironmentCredential::username_password_env(
            tenant_id.map(|s| s.to_owned()),
            azure_client_id.to_owned(),
            azure_username.to_owned(),
            azure_password.to_owned(),
        ))
    }

    fn try_username_password_runtime_env(
    ) -> Result<PublicClientApplication<ResourceOwnerPasswordCredential>, VarError> {
        let tenant_id = std::env::var(AZURE_TENANT_ID).ok();
        let azure_client_id = std::env::var(AZURE_CLIENT_ID)?;
        let azure_username = std::env::var(AZURE_USERNAME)?;
        let azure_password = std::env::var(AZURE_PASSWORD)?;
        Ok(EnvironmentCredential::username_password_env(
            tenant_id,
            azure_client_id,
            azure_username,
            azure_password,
        ))
    }

    pub fn username_password_env(
        tenant_id: Option<String>,
        azure_client_id: String,
        azure_username: String,
        azure_password: String,
    ) -> PublicClientApplication<ResourceOwnerPasswordCredential> {
        match tenant_id {
            Some(tenant_id) => {
                PublicClientApplication::new(ResourceOwnerPasswordCredential::new_with_tenant(
                    tenant_id,
                    azure_client_id,
                    azure_username,
                    azure_password,
                ))
            }
            None => PublicClientApplication::new(ResourceOwnerPasswordCredential::new(
                azure_client_id,
                azure_username,
                azure_password,
            )),
        }
    }
}
