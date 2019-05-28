#[derive(Serialize, Deserialize, Getters, Setters)]
#[set = "pub set"]
struct Video {
    #[serde(rename = "audioBitsPerSample")]
    audio_bits_per_sample: i64,
    #[serde(rename = "audioChannels")]
    audio_channels: i64,
    #[serde(rename = "audioFormat")]
    audio_format: String,
    #[serde(rename = "audioSamplesPerSecond")]
    audio_samples_per_second: i64,
    bitrate: i64,
    duration: i64,
    #[serde(rename = "fourCC")]
    four_cc: String,
    #[serde(rename = "frameRate")]
    frame_rate: f64,
    height: i64,
    width: i64,
}
