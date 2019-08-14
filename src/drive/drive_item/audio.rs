use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/audio?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct Audio {
    #[serde(skip_serializing_if = "Option::is_none")]
    album: Option<String>,
    #[serde(rename = "albumArtist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    album_artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    artist: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    composers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    copyright: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disc: Option<i64>,
    #[serde(rename = "discCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    disc_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    genre: Option<String>,
    #[serde(rename = "hasDrm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    has_drm: Option<bool>,
    #[serde(rename = "isVariableBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    is_variable_bitrate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    track: Option<i64>,
    #[serde(rename = "trackCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    track_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    year: Option<i64>,
}
