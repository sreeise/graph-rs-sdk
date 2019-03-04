use crate::drive::base::driveitem::DriveItem;
use crate::drive::endpoint::DriveEndPoint;
use crate::drive::{Drive, ItemResult};
use graph_error::{GraphError, RequestError};
use reqwest::{header, Response};

pub trait Item: serde::Serialize + for<'de> serde::Deserialize<'de> {
    type DI: serde::Serialize + for<'de> serde::Deserialize<'de>;

    fn token(&self) -> &str;

    fn request(&mut self, end_point: DriveEndPoint) -> std::result::Result<Response, RequestError> {
        let client = reqwest::Client::builder().build()?;
        client
            .get(DriveEndPoint::build(end_point).as_str())
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .map_err(RequestError::from)
    }

    fn get(&self, url: &str) -> ItemResult<Self::DI> {
        let client = reqwest::Client::builder().build()?;
        let response = client
            .get(url)
            .header(header::AUTHORIZATION, self.token())
            .header(header::CONTENT_TYPE, "application/json")
            .send();

        match response {
            Ok(mut t) => {
                let status = t.status().as_u16();
                if GraphError::is_error(status) {
                    return Err(RequestError::from(GraphError::from(status)));
                }

                let keys: Self::DI = t.json()?;
                Ok(keys)
            },
            Err(e) => Err(RequestError::from(e)),
        }
    }

    fn drive_item(&mut self, end_point: DriveEndPoint) -> ItemResult<Self::DI> {
        let response = self.request(end_point);
        match response {
            Ok(mut t) => {
                let item: Self::DI = t.json()?;
                Ok(item)
            },
            Err(e) => Err(e),
        }
    }
}

impl Item for Drive {
    type DI = DriveItem;

    fn token(&self) -> &str {
        self.access_token.as_str()
    }
}