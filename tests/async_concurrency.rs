use futures::stream::{self, StreamExt};
use graph_rs_sdk::http::GraphResponse;
use serde::Deserialize;
use serde::Serialize;
use test_tools::oauthrequest::OAuthTestClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: Option<String>,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub(crate) value: Vec<User>,
    #[serde(rename = "@odata.nextLink")]
    pub(crate) next_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseDetail {
    id: Option<String>,
    #[serde(rename = "skuId")]
    sku_id: Option<String>,
}

#[tokio::test]
async fn buffered_requests() {
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let users_resp: GraphResponse<Users> = client
            .v1()
            .users()
            .list_user()
            .select(&["id", "userPrincipalName"])
            .top("5")
            .json()
            .await
            .unwrap();

        let users: Vec<String> = users_resp
            .into_body()
            .unwrap()
            .value
            .iter()
            .filter_map(|user| user.id.clone())
            .collect();
        assert!(!users.is_empty());

        let mut stream = stream::iter(users)
            .map(|i| async {
                let license_details: GraphResponse<LicenseDetail> = client
                    .v1()
                    .users()
                    .id(i)
                    .list_license_details()
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
