#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
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

impl SharePointIds {
    pub fn list_id(&self) -> Option<String> {
        self.list_id.clone()
    }

    pub fn list_item_id(&self) -> Option<String> {
        self.list_item_id.clone()
    }

    pub fn list_item_unique_id(&self) -> Option<String> {
        self.list_item_unique_id.clone()
    }

    pub fn site_id(&self) -> Option<String> {
        self.site_id.clone()
    }

    pub fn site_url(&self) -> Option<String> {
        self.site_url.clone()
    }

    pub fn web_id(&self) -> Option<String> {
        self.web_id.clone()
    }
}
