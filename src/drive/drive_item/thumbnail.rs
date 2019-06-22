use std::io::Write;

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct Thumbnail {
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
    #[serde(rename = "sourceItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    source_item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct ThumbnailSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    large: Option<Thumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    medium: Option<Thumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    small: Option<Thumbnail>,
    #[serde(rename = "smallSquare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    small_square: Option<Thumbnail>,
    #[serde(rename = "mediumSquare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    medium_square: Option<Thumbnail>,
    #[serde(rename = "largeSquare")]
    #[serde(skip_serializing_if = "Option::is_none")]
    large_square: Option<Thumbnail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Thumbnail>,
}

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct ThumbnailCollection {
    #[serde(rename = "value")]
    thumbnails: Vec<ThumbnailSet>,
}
