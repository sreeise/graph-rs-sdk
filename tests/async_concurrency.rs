use futures::stream::{self, StreamExt};
use graph_http::traits::ODataNextLink;
use graph_rs_sdk::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use test_tools::oauthrequest::OAuthTestClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "@odata.nextLink")]
    next_link: Option<String>,
    value: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    std::env::set_var("GRAPH_TEST_ENV", "true");
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let mut stream = client
            .users()
            .list_user()
            .select(&["id", "userPrincipalName"])
            .top("5")
            .stream_next_links::<UserResponse>()
            .unwrap();

        let mut users1: Vec<UserResponse> = Vec::new();
        while let Some(Ok(body)) = stream.next().await {
            users1.push(body);
        }
        dbg!(&users1);
        /*
        let users: Vec<String> = users1
            .iter()
            .filter_map(|user| user.id.clone())
            .collect();
         */

        assert!(!users1.is_empty());

        let mut stream = stream::iter(users)
            .map(|i| async {
                let license_details: Vec<LicenseDetail> = client
                    .v1()
                    .users()
                    .id(i)
                    .list_license_details()
                    .paging()
                    .json()
                    .await
                    .unwrap();

                license_details
            })
            .buffered(5);

        while let Some(license_detail) = stream.next().await {
            assert_eq!(license_detail.status(), 200);
        }
    }
}
