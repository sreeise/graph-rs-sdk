use crate::drive;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::driveaction::DownloadFormat;
use crate::drive::ItemResult;
use graph_error::GraphError;
use reqwest::{header, Client, RequestBuilder, Response};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use transform_request::RequestError;

pub trait Item {
    /// The token() method should return a bearer token to make
    /// authenticated calls to the OneDrive API.
    fn token(&self) -> &str;

    fn item<T>(&self, r: &mut Response) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        if GraphError::is_error(r.status().as_u16()) {
            return Err(RequestError::from(
                GraphError::try_from(r).unwrap_or_default(),
            ));
        }
        let item: T = r.json()?;
        Ok(item)
    }

    /// Converts response to a drive item. Checks for errors
    /// for errors returned from the OneDrive API and if found
    /// returns the error.
    fn drive_item(&self, r: &mut Response) -> ItemResult<DriveItem> {
        if GraphError::is_error(r.status().as_u16()) {
            return Err(RequestError::from(
                GraphError::try_from(r).unwrap_or_default(),
            ));
        }
        let drive_item = DriveItem::try_from(r)?;
        Ok(drive_item)
    }

    /// Converts serde_json Value to a drive item. Checks for errors
    /// for errors returned from the OneDrive API and if found
    /// returns the error.
    fn value(r: &mut Response) -> ItemResult<serde_json::Value> {
        if GraphError::is_error(r.status().as_u16()) {
            return Err(RequestError::from(
                GraphError::try_from(r).unwrap_or_default(),
            ));
        }
        let v: serde_json::Value = r.json()?;
        Ok(v)
    }

    fn client(&self) -> Result<Client, RequestError> {
        reqwest::Client::builder()
            .build()
            .map_err(RequestError::from)
    }

    /// A request builder for REST requests with the authorization header
    /// included.
    fn request_builder(&self, url: &str) -> ItemResult<RequestBuilder> {
        Ok(self.client()?.get(url).bearer_auth(self.token()))
    }

    fn get<T>(&self, url: &str) -> ItemResult<T>
    where
        T: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let mut response = self
            .client()?
            .get(url)
            .bearer_auth(self.token())
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
            .bearer_auth(self.token())
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
            .bearer_auth(self.token())
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
            .bearer_auth(self.token())
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
            .bearer_auth(self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        self.item(&mut response)
    }
}

pub trait Download: Item {
    // https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content?view=odsp-graph-online
    fn download<P: AsRef<Path>>(
        &self,
        directory: P,
        value: &mut drive::value::Value,
    ) -> ItemResult<PathBuf>;

    // https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content_format?view=odsp-graph-online
    fn download_format<P: AsRef<Path>>(
        &self,
        directory: P,
        value: &mut drive::value::Value,
        format: DownloadFormat,
    ) -> ItemResult<PathBuf>;
}
