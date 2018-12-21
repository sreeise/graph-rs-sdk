/*
Microsoft Graph and OneDrive API use OAuth 2.0 for authorization. By completing an OAuth flow,
your app receives an access token that provides access to the Microsoft Graph a particular
set of permissions for a user.

Your app provides the access token in each request, through an HTTP header:

Authorization: bearer {token}
*/

pub mod endpoint;

use crate::drive::endpoint::DriveEndPoint;
use reqwest::*;
use std;

pub trait DriveRequest {
    fn request(&mut self, end_point: DriveEndPoint) -> std::result::Result<Response, reqwest::Error>;
}

pub struct Drive {
    access_token: String,
}

impl Drive {
    pub fn new(access_code: &str) -> Drive {
        Drive {
            access_token: String::from(access_code),
        }
    }

    pub fn reset_access_token(&mut self, access_token: &str) {
        self.access_token = String::from(access_token);
    }
}

impl DriveRequest for Drive {
    /// A drive request can make a request to any of the end points on the DriveEndPoint enum
    fn request(&mut self, end_point: DriveEndPoint) -> std::result::Result<Response, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(end_point.as_str())
            .header(header::AUTHORIZATION, self.access_token.as_str())
            .header(header::CONTENT_TYPE, "application/json")
            .send().expect("Error with request to microsoft graph");
        Ok(res)
    }
}
