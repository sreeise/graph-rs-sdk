use crate::auth::OAuthCredential;
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

pub struct EncodeBuilder {
    serializer: form_urlencoded::Serializer<String>,
    scopes: Vec<String>,
    url: Option<String>,
}

impl EncodeBuilder {
    pub fn new() -> EncodeBuilder {
        EncodeBuilder {
            serializer: form_urlencoded::Serializer::new(String::new()),
            scopes: Vec::new(),
            url: None,
        }
    }

    pub fn client_id(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::ClientId.alias(), value);
        }
        self
    }

    pub fn client_secret(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::ClientSecret.alias(), value);
        }
        self
    }

    pub fn authorize_url(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.url = Some(value.into());
        }
        self
    }

    pub fn access_token_url(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.url = Some(value.into());
        }
        self
    }

    pub fn refresh_token_url(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.url = Some(value.into());
        }
        self
    }

    pub fn redirect_uri(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::RedirectURI.alias(), value);
        }
        self
    }

    pub fn code(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::AccessCode.alias(), value);
        }
        self
    }

    pub fn access_token(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::AccessToken.alias(), value);
        }
        self
    }

    pub fn refresh_token(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::RefreshToken.alias(), value);
        }
        self
    }

    pub fn response_mode(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::ResponseMode.alias(), value);
        }
        self
    }

    pub fn state(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::State.alias(), value);
        }
        self
    }

    pub fn response_type(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::ResponseType.alias(), value);
        }
        self
    }

    pub fn grant_type(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::GrantType.alias(), value);
        }
        self
    }

    pub fn nonce(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::Nonce.alias(), value);
        }
        self
    }

    pub fn scope(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.scopes.push(value.into());
        }
        self
    }

    pub fn post_logout_redirect_uri(&mut self, value: &str) -> &mut Self {
        if !value.is_empty() {
            self.serializer
                .append_pair(OAuthCredential::PostLogoutRedirectURI.alias(), value);
        }
        self
    }

    pub fn logout_url(&mut self, value: &str) -> &mut Self {
        self.serializer
            .append_pair(OAuthCredential::LogoutURL.alias(), value);
        self
    }

    pub fn build(&mut self) -> String {
        if let Some(url) = self.url.as_ref() {
            let mut s = String::from(url.as_str());
            if !s.ends_with('?') {
                s.push('?');
            }
            s.push_str(self.build_body().as_str());
            s
        } else {
            self.build_body()
        }
    }

    pub fn build_body(&mut self) -> String {
        if !self.scopes.is_empty() {
            self.serializer.append_pair(
                OAuthCredential::Scopes.alias(),
                self.scopes.join(" ").as_str(),
            );
        }
        self.serializer.finish()
    }
}
