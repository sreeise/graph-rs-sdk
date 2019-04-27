use crate::drive::ItemResult;
use reqwest::{header, Client, Response};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use transform_request::RequestError;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::transform::*;
use crate::drive;

pub trait Item {
    fn token(&self) -> &str;

    fn item<T>(&self, r: &mut Response) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let item: T = r.json()?;
        Ok(item)
    }

    fn drive_item(&self, r: &mut Response) -> ItemResult<DriveItem> {
        let drive_item = DriveItem::transform(r)?;
        Ok(drive_item)
    }

    fn value(r: &mut Response) -> ItemResult<Value> {
        let v: Value = r.json()?;
        Ok(v)
    }

    fn client(&self) -> Result<Client, RequestError> {
        reqwest::Client::builder()
            .build()
            .map_err(RequestError::from)
    }

    fn get<T>(&self, url: &str) -> ItemResult<T>
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

    fn get_with_body<T>(&self, url: &str, body: &HashMap<String, String>) -> ItemResult<T>
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

    fn post<T>(&self, url: &str, body: &'static str) -> ItemResult<T>
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

    fn put<T>(&self, url: &str, body: &'static str) -> ItemResult<T>
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

    fn patch<T>(&self, url: &str) -> ItemResult<T>
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

pub trait Download: Item {
    fn download<P: AsRef<Path>>(
        &self,
        directory: P,
        value: &mut drive::value::Value,
    ) -> ItemResult<PathBuf>;
}
