use futures::stream::{self, StreamExt};
use graph_http::traits::ODataNextLink;
use graph_rs_sdk::*;
use serde::Deserialize;
use serde::Serialize;
use test_tools::oauth_request::{OAuthTestClient, ASYNC_THROTTLE_MUTEX};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>,
    value: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: Option<String>,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: Option<String>,
}

impl ODataNextLink for UserResponse {
    fn odata_next_link(&self) -> Option<String> {
        self.next_link.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseDetail {
    id: Option<String>,
    #[serde(rename = "skuId")]
    sku_id: Option<String>,
}

#[tokio::test]
async fn buffered_requests() {
    let _ = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let mut stream = client
            .users()
            .list_user()
            .select(&["id", "userPrincipalName"])
            .top("5")
            .paging()
            .stream::<UserResponse>()
            .unwrap();

        let mut users: Vec<String> = Vec::new();
        while let Some(Ok(user_response)) = stream.next().await {
            let body = user_response.into_body();

            users.extend(body.value.iter().flat_map(|user| user.id.clone()));
        }

        assert!(!users.is_empty());

        let mut stream = stream::iter(users)
            .map(|i| async {
                client
                    .users()
                    .id(i)
                    .license_details()
                    .list_license_details()
                    .paging()
                    .json::<LicenseDetail>()
                    .await
                    .unwrap()
            })
            .buffered(5);

        while let Some(vec) = stream.next().await {
            for response in vec {
                assert!(response.status().is_success());
            }
        }
    }
}
