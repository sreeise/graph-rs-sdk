mod auth_code_authorization_url;
mod authorization_code_certificate_credential;
mod authorization_code_credential;
mod client_certificate_credential;
mod client_credentials_authorization_url;
mod client_secret_credential;
mod confidential_client_application;
mod implicit_credential_authorization_url;
mod prompt;
mod proof_key_for_code_exchange;
mod public_client_application;
mod response_mode;
mod response_type;
mod token_credential;
mod token_request;

#[cfg(feature = "openssl")]
mod client_assertion;

pub use auth_code_authorization_url::*;
pub use authorization_code_certificate_credential::*;
pub use authorization_code_credential::*;
pub use client_certificate_credential::*;
pub use client_credentials_authorization_url::*;
pub use client_secret_credential::*;
pub use confidential_client_application::*;
pub use implicit_credential_authorization_url::*;
pub use prompt::*;
pub use proof_key_for_code_exchange::*;
pub use public_client_application::*;
pub use response_mode::*;
pub use response_type::*;
pub use token_credential::*;
pub use token_request::*;

#[cfg(feature = "openssl")]
pub use client_assertion::*;

// Powershell
// [System.Diagnostics.Tracing.EventSource]::new("graph-rs-sdk").Guid
pub static EVENT_TRACING_GUID: &str = "58c1e34e-8df1-5dfb-4a3c-6066550ab7f7";
