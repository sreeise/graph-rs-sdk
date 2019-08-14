use std::io::Write;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Video {
    #[serde(rename = "audioBitsPerSample")]
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_bits_per_sample: Option<i64>,
    #[serde(rename = "audioChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_channels: Option<i64>,
    #[serde(rename = "audioFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_format: Option<String>,
    #[serde(rename = "audioSamplesPerSecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_samples_per_second: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bitrate: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<i64>,
    #[serde(rename = "fourCC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    four_cc: Option<String>,
    #[serde(rename = "frameRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    frame_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i64>,
}

impl Eq for Video {}
