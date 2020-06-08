use reqwest::header::HeaderMap;
use reqwest::{header, redirect::Policy, IntoUrl, Method};
use std::future::Future;
use handlebars::Handlebars;
use reqwest::Url;

static V1_URL: &str = "https://graph.microsoft.com/v1.0";
static BETA_URL: &str = "https://graph.microsoft.com/beta";

pub struct Render<'a> {
    host: String,
    handlebars: Handlebars<'a>,
}

impl<'a> Render<'a> {
    pub fn new(host: &str) -> Render<'a> {
        Render {
            host: host.into(),
            handlebars: Handlebars::new(),
        }
    }

    pub fn new_v1() -> Render<'a> {
        Render {
            host: V1_URL.into(),
            handlebars: Handlebars::new(),
        }
    }

    pub fn new_beta() -> Render<'a> {
        Render {
            host: BETA_URL.into(),
            handlebars: Handlebars::new(),
        }
    }

    pub fn v1(&mut self) {
        self.host = V1_URL.into()
    }

    pub fn beta(&mut self) {
        self.host = BETA_URL.into();
    }

    pub fn url(&self, path: &str, params: &serde_json::Value) -> Url {
        let mut url = Url::parse(&self.host).unwrap();
        let p = self.handlebars.render_template(path, params).unwrap();
        url.set_path(&p);
        url
    }
}

static JSON_TYPE_HEADER: &str = "application/json";

pub trait RequestClient<RHS = Self> {
    type Output;
    type Body;

    fn with_headers(&mut self, headers: HeaderMap) -> &mut Self;
    fn set_body<B: Into<Self::Body>>(&mut self, body: B) -> &mut Self;
    fn get<U: IntoUrl>(&mut self, url: U) -> Self::Output;
    fn post<U: IntoUrl, B: serde::Serialize + Sync>(&mut self, url: U, body: &B) -> Self::Output;
    fn patch<U: IntoUrl, B: serde::Serialize + Sync>(&mut self, url: U, body: &B) -> Self::Output;
    fn put<U: IntoUrl, B: serde::Serialize + Sync>(&mut self, url: U, body: &B) -> Self::Output;
    fn delete<U: IntoUrl>(&mut self, url: U) -> Self::Output;
}

macro_rules! request_client_impl {
    ( $client_type:ty, $output:ty, $body:ty ) => {
        impl RequestClient for $client_type {
            type Output = $output;
            type Body = $body;

            fn with_headers(&mut self, headers: HeaderMap) -> &mut Self {
                self.next_request_headers = Some(headers);
                self
            }

            fn set_body<B: Into<Self::Body>>(&mut self, body: B) -> &mut Self {
                    self.body = Some(body.into());
                    self
            }

            fn get<U: IntoUrl>(&mut self, url: U) -> Self::Output {
                if self.next_request_headers.is_some() {
                    self.client.get(url).headers(self.next_request_headers.take().unwrap()).send()
                } else {
                    self.client.get(url).send()
                }
            }

            fn post<U: IntoUrl, B: serde::Serialize + Sync>(
                &mut self,
                url: U,
                body: &B,
            ) -> Self::Output {
                let body = serde_json::to_string(body).unwrap();

                if self.next_request_headers.is_some() {
                    let mut headers = self.next_request_headers.take().unwrap();
                    headers.insert(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str(JSON_TYPE_HEADER).unwrap(),
                    );
                    self.client.post(url).headers(headers).body(body).send()
                } else {
                    self.client
                        .post(url)
                        .header(header::CONTENT_TYPE, JSON_TYPE_HEADER)
                        .body(body)
                        .send()
                }
            }

            fn patch<U: IntoUrl, B: serde::Serialize + Sync>(
                &mut self,
                url: U,
                body: &B,
            ) -> Self::Output {
                let body = serde_json::to_string(body).unwrap();

                if self.next_request_headers.is_some() {
                    let mut headers = self.next_request_headers.take().unwrap();
                    headers.insert(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str(JSON_TYPE_HEADER).unwrap(),
                    );
                    self.client.patch(url).headers(headers).body(body).send()
                } else {
                    self.client
                        .patch(url)
                        .header(header::CONTENT_TYPE, JSON_TYPE_HEADER)
                        .body(body)
                        .send()
                }
            }

            fn put<U: IntoUrl, B: serde::Serialize + Sync>(
                &mut self,
                url: U,
                body: &B,
            ) -> Self::Output {
                let body = serde_json::to_string(body).unwrap();

                if self.next_request_headers.is_some() {
                    let mut headers = self.next_request_headers.take().unwrap();
                    headers.insert(
                        header::CONTENT_TYPE,
                        header::HeaderValue::from_str(JSON_TYPE_HEADER).unwrap(),
                    );
                    self.client.put(url).headers(headers).body(body).send()
                } else {
                    self.client
                        .put(url)
                        .header(header::CONTENT_TYPE, JSON_TYPE_HEADER)
                        .body(body)
                        .send()
                }
            }

            fn delete<U: IntoUrl>(&mut self, url: U) -> Self::Output {
                if self.next_request_headers.is_some() {
                    self.client.delete(url).headers(self.next_request_headers.take().unwrap()).send()
                } else {
                    self.client.delete(url).send()
                }
            }
        }
    };
}

#[derive(Debug)]
pub struct AsyncClient {
    pub client: reqwest::Client,
    token: String,
    next_request_headers: Option<HeaderMap>,
    body: Option<reqwest::Body>,
}

impl AsyncClient {
    pub fn new(token: &str) -> AsyncClient {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(token).unwrap(),
        );
        AsyncClient {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .redirect(Policy::limited(1))
                .build()
                .unwrap(),
            token: token.into(),
            next_request_headers: None,
            body: None,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl Default for AsyncClient {
    fn default() -> Self {
        AsyncClient {
            client: reqwest::Client::builder()
                .redirect(Policy::limited(1))
                .build()
                .unwrap(),
            token: String::new(),
            next_request_headers: None,
            body: None,
        }
    }
}

request_client_impl!(
    AsyncClient,
    impl Future<Output = Result<reqwest::Response, reqwest::Error>>,
    reqwest::Body
);

#[derive(Debug)]
pub struct BlockingClient {
    pub client: reqwest::blocking::Client,
    token: String,
    next_request_headers: Option<HeaderMap>,
    body: Option<reqwest::blocking::Body>
}

impl BlockingClient {
    pub fn new(token: &str) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(token).unwrap(),
        );
        BlockingClient {
            client: reqwest::blocking::Client::builder()
                .default_headers(headers)
                .redirect(Policy::limited(2))
                .build()
                .unwrap(),
            token: token.into(),
            next_request_headers: None,
            body: None,
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl Default for BlockingClient {
    fn default() -> Self {
        BlockingClient {
            client: reqwest::blocking::Client::builder()
                .redirect(Policy::limited(2))
                .build()
                .unwrap(),
            token: String::new(),
            next_request_headers: None,
            body: None,
        }
    }
}

request_client_impl!(BlockingClient, Result<reqwest::blocking::Response, reqwest::Error>, reqwest::blocking::Body);

pub struct HttpClient<'a, T: RequestClient> {
    pub request: T,
    pub render: Render<'a>,
}

impl<'a, T: RequestClient> HttpClient<'a, T> {
    pub fn v1(&mut self) -> &mut Self {
        self.render.v1();
        self
    }

    pub fn beta(&mut self) -> &mut Self {
        self.render.beta();
        self
    }
}

impl<'a> HttpClient<'a, AsyncClient> {
    pub fn new_default_async() -> HttpClient<'a, AsyncClient> {
        HttpClient {
            request: AsyncClient::default(),
            render: Render::new_v1(),
        }
    }

    pub fn new_async(token: &str) -> HttpClient<'a, AsyncClient> {
        HttpClient {
            request: AsyncClient::new(token),
            render: Render::new_v1(),
        }
    }
}

impl<'a> HttpClient<'a, BlockingClient> {
    pub fn new_default_blocking() -> HttpClient<'a, BlockingClient> {
        HttpClient {
            request: BlockingClient::default(),
            render: Render::new_v1(),
        }
    }

    pub fn new_blocking(token: &str) -> HttpClient<'a, BlockingClient> {
        HttpClient {
            request: BlockingClient::new(token),
            render: Render::new_v1(),
        }
    }
}
