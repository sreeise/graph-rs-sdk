#[macro_use]
mod credential_builder;

pub mod legacy;

mod as_query;
mod auth_code_authorization_url_parameters;
mod authorization_code_certificate_credential;
mod authorization_code_credential;
mod client_application;
mod client_certificate_credential;
mod client_credentials_authorization_url;
mod client_secret_credential;
mod confidential_client_application;
mod crypto;
mod device_code_credential;
mod display;
mod environment_credential;
mod implicit_credential;
mod open_id_authorization_url;
mod open_id_credential;
mod prompt;
mod proof_key_for_code_exchange;
mod public_client_application;
mod public_client_application_builder;
mod resource_owner_password_credential;
mod response_mode;
mod response_type;
mod token_credential;
mod token_credential_options;
mod token_request;

#[cfg(feature = "openssl")]
mod x509_certificate;

pub use as_query::*;
pub use auth_code_authorization_url_parameters::*;
pub use authorization_code_certificate_credential::*;
pub use authorization_code_credential::*;
pub use client_application::*;
pub use client_certificate_credential::*;
pub use client_credentials_authorization_url::*;
pub use client_secret_credential::*;
pub use confidential_client_application::*;
pub use credential_builder::*;
pub(crate) use crypto::*;
pub use device_code_credential::*;
pub use display::*;
pub use environment_credential::*;
pub use implicit_credential::*;
pub use open_id_authorization_url::*;
pub use open_id_credential::*;
pub use prompt::*;
pub use proof_key_for_code_exchange::*;
pub use public_client_application::*;
pub use public_client_application_builder::*;
pub use resource_owner_password_credential::*;
pub use response_mode::*;
pub use response_type::*;
pub use token_credential::*;
pub use token_credential_options::*;
pub use token_request::*;

#[cfg(feature = "openssl")]
pub use x509_certificate::*;
