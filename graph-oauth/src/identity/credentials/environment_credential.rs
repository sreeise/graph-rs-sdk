use crate::identity::{AuthorizationSerializer, ClientSecretCredential};
use crate::oauth::{
    ConfidentialClientApplication, PublicClientApplication, ResourceOwnerPasswordCredential,
};
use graph_error::{AuthorizationResult, GraphFailure};
use std::env::VarError;

const AZURE_TENANT_ID: &'static str = "AZURE_TENANT_ID";
const AZURE_CLIENT_ID: &str = "AZURE_CLIENT_ID";
const AZURE_CLIENT_SECRET: &str = "AZURE_CLIENT_SECRET";
const AZURE_USERNAME: &str = "AZURE_USERNAME";
const AZURE_PASSWORD: &str = "AZURE_PASSWORD";

pub struct EnvironmentCredential {
    pub credential: Box<dyn AuthorizationSerializer + Send>,
}

impl EnvironmentCredential {
    pub fn new() -> Result<EnvironmentCredential, VarError> {
        match EnvironmentCredential::compile_time_environment_credential() {
            Some(credential) => Ok(credential),
            None => EnvironmentCredential::runtime_environment_credential(),
        }
    }

    fn compile_time_environment_credential() -> Option<EnvironmentCredential> {
        fn try_azure_client_secret() -> Option<EnvironmentCredential> {
            let tenant_id_option = option_env!("AZURE_TENANT_ID");
            let azure_client_id = option_env!("AZURE_CLIENT_ID")?;
            let azure_client_secret = option_env!("AZURE_CLIENT_SECRET")?;

            if let Some(tenant_id) = tenant_id_option {
                Some(EnvironmentCredential::from(
                    ConfidentialClientApplication::new(
                        ClientSecretCredential::new_with_tenant(
                            tenant_id,
                            azure_client_id,
                            azure_client_secret,
                        ),
                        Default::default(),
                    )
                    .ok()?,
                ))
            } else {
                Some(EnvironmentCredential::from(
                    ConfidentialClientApplication::new(
                        ClientSecretCredential::new(azure_client_id, azure_client_secret),
                        Default::default(),
                    )
                    .ok()?,
                ))
            }
        }

        fn try_username_password() -> Option<EnvironmentCredential> {
            let tenant_id_option = option_env!("AZURE_TENANT_ID");
            let azure_client_id = option_env!("AZURE_CLIENT_ID")?;
            let azure_username = option_env!("AZURE_USERNAME")?;
            let azure_password = option_env!("AZURE_PASSWORD")?;

            match tenant_id_option {
                Some(tenant_id) => Some(EnvironmentCredential::from(
                    PublicClientApplication::new(
                        ResourceOwnerPasswordCredential::new_with_tenant(
                            tenant_id,
                            azure_client_id,
                            azure_username,
                            azure_password,
                        ),
                        Default::default(),
                    )
                    .ok()?,
                )),
                None => Some(EnvironmentCredential::from(
                    PublicClientApplication::new(
                        ResourceOwnerPasswordCredential::new(
                            azure_client_id,
                            azure_username,
                            azure_password,
                        ),
                        Default::default(),
                    )
                    .ok()?,
                )),
            }
        }

        match try_azure_client_secret() {
            Some(credential) => Some(credential),
            None => try_username_password(),
        }
    }

    fn runtime_environment_credential() -> Result<EnvironmentCredential, VarError> {
        fn try_azure_client_secret() -> Result<EnvironmentCredential, VarError> {
            let tenant_id_result = std::env::var(AZURE_TENANT_ID);
            let azure_client_id = std::env::var(AZURE_CLIENT_ID)?;
            let azure_client_secret = std::env::var(AZURE_CLIENT_SECRET)?;

            if let Ok(tenant_id) = tenant_id_result {
                Ok(EnvironmentCredential::from(
                    ConfidentialClientApplication::new(
                        ClientSecretCredential::new_with_tenant(
                            tenant_id,
                            azure_client_id,
                            azure_client_secret,
                        ),
                        Default::default(),
                    )
                    .map_err(|_| VarError::NotPresent)?,
                ))
            } else {
                Ok(EnvironmentCredential::from(
                    ConfidentialClientApplication::new(
                        ClientSecretCredential::new(azure_client_id, azure_client_secret),
                        Default::default(),
                    )
                    .map_err(|_| VarError::NotPresent)?,
                ))
            }
        }

        fn try_username_password() -> Result<EnvironmentCredential, VarError> {
            let tenant_id_result = std::env::var(AZURE_TENANT_ID);
            let azure_client_id = std::env::var(AZURE_CLIENT_ID)?;
            let azure_username = std::env::var(AZURE_USERNAME)?;
            let azure_password = std::env::var(AZURE_PASSWORD)?;

            match tenant_id_result {
                Ok(tenant_id) => Ok(EnvironmentCredential::from(
                    PublicClientApplication::new(
                        ResourceOwnerPasswordCredential::new_with_tenant(
                            tenant_id,
                            azure_client_id,
                            azure_username,
                            azure_password,
                        ),
                        Default::default(),
                    )
                    .map_err(|_| VarError::NotPresent)?,
                )),
                Err(_) => Ok(EnvironmentCredential::from(
                    PublicClientApplication::new(
                        ResourceOwnerPasswordCredential::new(
                            azure_client_id,
                            azure_username,
                            azure_password,
                        ),
                        Default::default(),
                    )
                    .map_err(|_| VarError::NotPresent)?,
                )),
            }
        }

        match try_azure_client_secret() {
            Ok(credential) => Ok(credential),
            Err(_) => try_username_password(),
        }
    }
}

impl From<ClientSecretCredential> for EnvironmentCredential {
    fn from(value: ClientSecretCredential) -> Self {
        EnvironmentCredential {
            credential: Box::new(value),
        }
    }
}

impl From<ResourceOwnerPasswordCredential> for EnvironmentCredential {
    fn from(value: ResourceOwnerPasswordCredential) -> Self {
        EnvironmentCredential {
            credential: Box::new(value),
        }
    }
}

impl From<ConfidentialClientApplication> for EnvironmentCredential {
    fn from(value: ConfidentialClientApplication) -> Self {
        EnvironmentCredential {
            credential: Box::new(value),
        }
    }
}

impl From<PublicClientApplication> for EnvironmentCredential {
    fn from(value: PublicClientApplication) -> Self {
        EnvironmentCredential {
            credential: Box::new(value),
        }
    }
}
