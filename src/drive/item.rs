use crate::drive::endpoint::DriveEndPoint;
use crate::drive::ItemResult;
use reqwest::{header, Response};
use serde_json::Value;
use transform_request::RequestError;

pub trait Item<T>
where
    T: serde::Serialize + for<'de> serde::Deserialize<'de>,
{
    fn token(&self) -> &str;
    fn item(&self, r: &mut Response) -> ItemResult<T>;

    fn value(r: &mut Response) -> ItemResult<Value> {
        let v: Value = r.json()?;
        Ok(v)
    }

    fn end_point(&mut self, end_point: DriveEndPoint) -> Result<Response, RequestError> {
        let client = reqwest::Client::builder().build()?;
        client
            .get(DriveEndPoint::build(end_point).as_str())
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .map_err(RequestError::from)
    }

    fn get(&self, url: &str) -> ItemResult<T> {
        let client = reqwest::Client::builder().build()?;
        let mut response = client
            .get(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        self.item(&mut response)
    }

    fn post(&self, url: &str) -> ItemResult<T> {
        let client = reqwest::Client::builder().build()?;
        let mut response = client
            .post(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        self.item(&mut response)
    }

    fn put(&self, url: &str) -> ItemResult<T> {
        let client = reqwest::Client::builder().build()?;
        let mut response = client
            .put(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "text/plain")
            .send()?;

        self.item(&mut response)
    }

    fn patch(&self, url: &str) -> ItemResult<T> {
        let client = reqwest::Client::builder().build()?;
        let mut response = client
            .patch(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()?;

        self.item(&mut response)
    }

    fn drive_item(&mut self, end_point: DriveEndPoint) -> ItemResult<T> {
        let mut response: Response = self.end_point(end_point)?;
        let value: T = response.json()?;
        Ok(value)
    }
}
