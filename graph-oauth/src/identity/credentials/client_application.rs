use crate::identity::{CredentialStoreType, TokenCredentialExecutor};
use crate::oauth::MsalTokenResponse;
use async_trait::async_trait;

#[async_trait]
pub trait ClientApplication: TokenCredentialExecutor {
    fn get_credential_from_store(&self) -> &CredentialStoreType;

    fn update_token_credential_store(&mut self, credential_store_type: CredentialStoreType);

    fn get_token_credential(&mut self) -> anyhow::Result<CredentialStoreType> {
        debug!("get_token_credential");
        let credential_from_store = self.get_credential_from_store();
        if !credential_from_store.eq(&CredentialStoreType::UnInitialized) {
            Ok(credential_from_store.clone())
        } else {
            let response = self.execute()?;
            let token_value: serde_json::Value = response.json()?;
            let bearer = token_value.to_string();
            let access_token_result: serde_json::Result<MsalTokenResponse> =
                serde_json::from_value(token_value);
            match access_token_result {
                Ok(access_token) => {
                    let credential_store_type = CredentialStoreType::AccessToken(access_token);
                    self.update_token_credential_store(credential_store_type.clone());
                    Ok(credential_store_type)
                }
                Err(_) => {
                    let credential_store_type = CredentialStoreType::Bearer(bearer);
                    self.update_token_credential_store(credential_store_type.clone());
                    Ok(credential_store_type)
                }
            }
        }
    }

    async fn get_token_credential_async(&mut self) -> anyhow::Result<CredentialStoreType> {
        debug!("get_token_credential");
        let credential_from_store = self.get_credential_from_store();
        if !credential_from_store.eq(&CredentialStoreType::UnInitialized) {
            Ok(credential_from_store.clone())
        } else {
            let response = self.execute_async().await?;
            let token_value: serde_json::Value = response.json().await?;
            let bearer = token_value.to_string();
            let access_token_result: serde_json::Result<MsalTokenResponse> =
                serde_json::from_value(token_value);
            match access_token_result {
                Ok(access_token) => {
                    let credential_store_type = CredentialStoreType::AccessToken(access_token);
                    self.update_token_credential_store(credential_store_type.clone());
                    Ok(credential_store_type)
                }
                Err(_) => {
                    let credential_store_type = CredentialStoreType::Bearer(bearer);
                    self.update_token_credential_store(credential_store_type.clone());
                    Ok(credential_store_type)
                }
            }
        }
    }
}
