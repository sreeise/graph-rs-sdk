use std::io::Write;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct SharePointIds {
    #[serde(rename = "listId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    list_id: Option<String>,
    #[serde(rename = "listItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    list_item_id: Option<String>,
    #[serde(rename = "listItemUniqueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    list_item_unique_id: Option<String>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    site_id: Option<String>,
    #[serde(rename = "siteUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    site_url: Option<String>,
    #[serde(rename = "webId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_id: Option<String>,
}

impl SharePointIds {
    pub fn new(
        list_id: Option<String>,
        list_item_id: Option<String>,
        list_item_unique_id: Option<String>,
        site_id: Option<String>,
        site_url: Option<String>,
        web_id: Option<String>,
    ) -> Self {
        SharePointIds {
            list_id,
            list_item_id,
            list_item_unique_id,
            site_id,
            site_url,
            web_id,
        }
    }
}
