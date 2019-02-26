use url::percent_encoding::{utf8_percent_encode, EncodeSet};

#[derive(Debug)]
pub struct PercentEncode;

impl PercentEncode {
    #[allow(dead_code)]
    pub fn utf8_percent_encode<T>(s: &str, encode_set: T) -> String
    where
        T: EncodeSet,
    {
        utf8_percent_encode(s, encode_set).collect::<String>()
    }

    #[allow(dead_code)]
    pub fn utf8_percent_encode_string<T>(s: String, encode_set: T) -> String
    where
        T: EncodeSet,
    {
        utf8_percent_encode(s.as_str(), encode_set).collect::<String>()
    }

    #[allow(dead_code)]
    pub fn ut8_percent_encode_url<T>(domain: &str, s: &str, encode_set: T) -> String
    where
        T: EncodeSet,
    {
        let resource = utf8_percent_encode(s, encode_set).collect::<String>();
        let mut url = String::from(domain);
        url.push_str(resource.as_str());
        url
    }

    #[allow(dead_code)]
    pub fn ut8_percent_encode_url_string<T>(domain: &str, s: String, encode_set: T) -> String
    where
        T: EncodeSet,
    {
        let resource = utf8_percent_encode(s.as_str(), encode_set).collect::<String>();
        let mut url = String::from(domain);
        url.push_str(resource.as_str());
        url
    }
}
