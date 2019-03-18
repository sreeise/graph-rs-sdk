use crate::drive;
use crate::drive::ItemResult;
use reqwest::{header, Client, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::io::ErrorKind;
use transform_request::RequestError;

pub trait Item<T> {
    fn token(&self) -> &str;
    fn item(&self, r: &mut Response) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>;

    fn value(r: &mut Response) -> ItemResult<Value> {
        let v: Value = r.json()?;
        Ok(v)
    }

    fn client(&self) -> Result<Client, RequestError> {
        reqwest::Client::builder()
            .build()
            .map_err(RequestError::from)
    }

    fn get(&self, url: &str) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let mut response = self
            .client()?
            .get(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        self.item(&mut response)
    }

    fn get_with_body(&self, url: &str, body: &HashMap<String, String>) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let mut response = self
            .client()?
            .get(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .form(body)
            .send()?;

        self.item(&mut response)
    }

    fn post(&self, url: &str, body: &'static str) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let mut response = self
            .client()?
            .post(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .send()?;

        self.item(&mut response)
    }

    fn put(&self, url: &str, body: &'static str) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let mut response = self
            .client()?
            .put(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "text/plain")
            .body(body)
            .send()?;

        self.item(&mut response)
    }

    fn patch(&self, url: &str) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let mut response = self
            .client()?
            .patch(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        self.item(&mut response)
    }
}

// TODO: Traits for common drive actions and testing.

pub trait Download {
    type Err: Error;
    fn download<T: Item<T>>(&self) -> Result<T, Self::Err>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>;
}

// TODO: This needs to be an actual downloaded file not a
// Rust struct.
impl Download for drive::value::Value {
    type Err = RequestError;
    fn download<T: Item<T>>(&self) -> Result<T, Self::Err>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        if let Some(url) = self.microsoft_graph_download_url() {
            let client = reqwest::Client::builder()
                .build()
                .map_err(RequestError::from)
                .unwrap();

            let mut s = client.get(url.as_str()).send().unwrap();
            let value: T = s.json()?;
            return Ok(value);
        }
        Err(RequestError::error_kind(
            ErrorKind::InvalidData,
            "Missing microsoft_graph_download_url",
        ))
    }
}
