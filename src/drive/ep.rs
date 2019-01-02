use crate::drive::endpoint::DriveEndPoint;
use crate::drive::folder::Folder;
use crate::drive::folder::SpecialFolder;
use crate::drive::AND_SELECT;
use crate::drive::{Drive, DriveResponse, UrlResult};

/*
WORK IN PROGRESS: In the process of seeing if there is a way to coerce a response
from the graph api with trait objects cleanly
*/

#[allow(dead_code)]
pub enum Select {
    Folder,
    SpecialFolder,
}

impl Select {
    pub fn as_str(&self) -> &str {
        match *self {
            Select::Folder => "folder",
            Select::SpecialFolder => "specialFolder",
        }
    }
}

pub trait EP {
    fn ep(&self, end_point_type: DriveEndPoint) -> UrlResult;
}

#[allow(dead_code)]
pub struct EPRequest {
    pub resource: Box<dyn EP>,
}

impl EPRequest {
    #[allow(dead_code)]
    fn build_query(end_point_type: DriveEndPoint, params: Select) -> String {
        let mut ep = DriveEndPoint::build(end_point_type).expect("Could not build drive end point");
        ep.push_str(AND_SELECT);
        ep.push_str(params.as_str());
        ep
    }

    #[allow(dead_code)]
    pub fn new_ep(resource: Select) -> EPRequest {
        match resource {
            Select::Folder => EPRequest {
                resource: Box::new(Folder::new()),
            },
            Select::SpecialFolder => EPRequest {
                resource: Box::new(SpecialFolder::new()),
            },
        }
    }

    #[allow(dead_code)]
    pub fn ep(select: Select, end_point: DriveEndPoint) -> String {
        let ep = EPRequest::new_ep(select);
        let req_url = ep
            .to_string(end_point)
            .expect("could not get end point url");
        req_url
    }

    #[allow(dead_code)]
    pub fn ep_select_url(end_point: DriveEndPoint, select: Select) -> UrlResult {
        let ep = EPRequest::new_ep(select);
        ep.to_string(end_point)
    }

    #[allow(dead_code)]
    pub fn to_string(&self, end_point_type: DriveEndPoint) -> UrlResult {
        self.resource.ep(end_point_type)
    }

    #[allow(dead_code)]
    pub fn select_struct(
        &self,
        select: Select,
        end_point: DriveEndPoint,
        access_token: &str,
    ) -> DriveResponse {
        Drive::req_url(&EPRequest::ep(select, end_point), access_token)
    }
}
