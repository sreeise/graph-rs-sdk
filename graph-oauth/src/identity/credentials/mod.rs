pub use app_config::*;
pub use application_builder::*;
pub use as_query::*;
pub use auth_code_authorization_url::*;
pub use authorization_code_assertion_credential::*;
pub use authorization_code_certificate_credential::*;
pub use authorization_code_credential::*;
pub use bearer_token_credential::*;
pub use client_assertion_credential::*;
pub use client_builder_impl::*;
pub use client_certificate_credential::*;
pub use client_credentials_authorization_url::*;
pub use client_secret_credential::*;
pub use confidential_client_application::*;
pub use device_code_credential::*;
pub use display::*;
pub use environment_credential::*;
pub use open_id_authorization_url::*;
pub use open_id_credential::*;
pub use prompt::*;
pub use public_client_application::*;
pub use resource_owner_password_credential::*;
pub use response_mode::*;
pub use response_type::*;
pub use token_credential_executor::*;
#[cfg(feature = "openssl")]
pub use x509_certificate::*;

#[macro_use]
mod client_builder_impl;

pub mod legacy;

mod app_config;
mod application_builder;
mod as_query;
mod auth_code_authorization_url;
mod authorization_code_assertion_credential;
mod authorization_code_certificate_credential;
mod authorization_code_credential;
mod bearer_token_credential;
mod client_assertion_credential;
mod client_certificate_credential;
mod client_credentials_authorization_url;
mod client_secret_credential;
mod confidential_client_application;
mod device_code_credential;
mod display;
mod environment_credential;
mod open_id_authorization_url;
mod open_id_credential;
mod prompt;
mod public_client_application;
mod resource_owner_password_credential;
mod response_mode;
mod response_type;
mod token_credential_executor;

#[cfg(feature = "openssl")]
mod x509_certificate;

pub(crate) mod tracing_targets {
    pub const CREDENTIAL_EXECUTOR: &str = "graph_rs_sdk::credential_executor";
    pub const INTERACTIVE_AUTH: &str = "graph_rs_sdk::interactive_auth";
}
