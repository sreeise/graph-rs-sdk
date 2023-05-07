use graph_oauth::oauth::EnvironmentCredential;
use std::env::VarError;

// EnvironmentCredential will first look for compile time environment variables
// and then runtime environment variables.

// You can create a resource owner password credential or a client secret credential
// depending on the environment variables you set.

// Resource Owner Password Credential Environment Variables:
// "AZURE_TENANT_ID" (Optional - puts the tenant id in the authorization url)
// "AZURE_CLIENT_ID" (Required)
// "AZURE_USERNAME" (Required)
// "AZURE_PASSWORD"  (Required)
pub fn username_password() -> Result<(), VarError> {
    let public_client_application = EnvironmentCredential::resource_owner_password_credential()?;
    Ok(())
}

// Client Secret Credentials Environment Variables:
// "AZURE_TENANT_ID" (Optional - puts the tenant id in the authorization url)
// "AZURE_CLIENT_ID" (Required)
// "AZURE_CLIENT_SECRET" (Required)
pub fn client_secret_credential() -> Result<(), VarError> {
    let confidential_client_application = EnvironmentCredential::client_secret_credential()?;
    Ok(())
}
