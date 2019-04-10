use crate::oautherror::OAuthError;
use std::collections::HashMap;
use url::form_urlencoded;
use url::percent_encoding::{utf8_percent_encode, EncodeSet};

#[derive(Debug)]
pub struct Encoder {
    builder: form_urlencoded::Serializer<String>,
}

impl Encoder {
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

    #[allow(dead_code)]
    pub fn utf8_url_encode_pairs<T>(
        pairs: HashMap<String, String>,
        encode_set: T,
    ) -> Result<String, OAuthError>
    where
        T: EncodeSet,
    {
        let mut s = String::new();
        let to_pair = |key, value| {
            let v = vec!["&", key, "=", value];
            v.join("")
        };

        pairs.iter().for_each(|p| {
            let pair = to_pair(p.0.as_str(), p.1.as_str());
            s.push_str(pair.as_str());
        });

        Encoder::utf8_percent_encode(s.as_str(), encode_set);
        Ok(s)
    }
}
