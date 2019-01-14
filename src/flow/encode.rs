use std::collections::HashMap;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

/* Builder for Microsoft authorization url and encoding.

Microsoft uses RFC 3986.
https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/addressing-driveitems?view=odsp-graph-online

NOTE: This may change to a general encoding and decoding module for Graph and/or OneDrive
*/

define_encode_set! {
    /// This encode set is used in the URL parser for query strings.
    pub ONEDRIVE_AUTH_RESERVED_ENCODE_SET = [DEFAULT_ENCODE_SET] | {'/', '*', '<', '>', '?', ':', '|', ' ', '#'}
}

pub fn encode_url(domain: &str, url: &str) -> String {
    let resource = utf8_percent_encode(url, ONEDRIVE_AUTH_RESERVED_ENCODE_SET).collect::<String>();
    let mut complete_url = String::from(domain);
    complete_url.push_str(resource.as_str());
    complete_url
}

#[derive(Debug)]
pub struct OauthUrlBuilder {
    url_params: HashMap<String, String>,
    encoded: bool,
}

impl OauthUrlBuilder {
    pub fn new(encode: bool) -> OauthUrlBuilder {
        OauthUrlBuilder {
            url_params: HashMap::new(),
            encoded: encode,
        }
    }

    pub fn scheme(&mut self, scheme: &str) -> &mut OauthUrlBuilder {
        self.url_params
            .insert(String::from("SCHEME"), String::from(scheme));
        self
    }

    pub fn host(&mut self, host: &str) -> &mut OauthUrlBuilder {
        self.url_params
            .insert(String::from("HOST"), String::from(host));
        self
    }

    pub fn path(&mut self, path: &str) -> &mut OauthUrlBuilder {
        self.url_params
            .insert(String::from("PATH"), String::from(path));
        self
    }

    pub fn query(&mut self, query: &str) -> &mut OauthUrlBuilder {
        self.url_params
            .insert(String::from("QUERY"), String::from(query));
        self
    }

    pub fn build(&mut self) -> String {
        if self.encoded {
            let mut url_string = String::from("");
            url_string.push_str(self.url_params.get("PATH").expect("Path not set"));
            url_string.push_str(self.url_params.get("QUERY").expect("Query not set"));

            let mut domain = String::from(
                self.url_params
                    .get("SCHEME")
                    .expect("Scheme not set")
                    .as_str(),
            );
            domain.push_str(self.url_params.get("HOST").expect("Host not set").as_str());
            let complete_url = encode_url(domain.as_str(), url_string.as_str());
            String::from(complete_url)
        } else {
            let mut url_string = String::from(
                self.url_params
                    .get("SCHEME")
                    .expect("Scheme not set")
                    .as_str(),
            );
            url_string.push_str(self.url_params.get("HOST").expect("Host not set").as_str());
            url_string.push_str(self.url_params.get("PATH").expect("Path not set").as_str());
            url_string.push_str(
                self.url_params
                    .get("QUERY")
                    .expect("Query not set")
                    .as_str(),
            );
            String::from(url_string)
        }
    }
}
