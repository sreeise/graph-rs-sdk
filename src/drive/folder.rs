use crate::drive::endpoint::DriveEndPoint;
use crate::drive::ep::Select;
use crate::drive::ep::EP;
use crate::drive::{UrlResult, AND_SELECT};

/*
The Folder and Special Folder structs

WORK IN PROGRESS: In the process of seeing if there is a way to coerce a response
from the graph api with trait objects cleanly
*/

#[allow(dead_code)]
#[derive(Debug)]
pub struct Folder {
    child_count: u64,
}

impl Folder {
    pub fn new() -> Folder {
        Folder { child_count: 0 }
    }
}

#[allow(dead_code)]
struct View {
    view_type: String,
    sort_by: String,
    sort_order: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialFolder {
    name: String,
}

impl SpecialFolder {
    pub fn new() -> SpecialFolder {
        SpecialFolder {
            name: String::from(""),
        }
    }

    pub fn new_sp(name: String) -> SpecialFolder {
        SpecialFolder { name }
    }
}

fn build_query(end_point_type: DriveEndPoint, params: Select) -> String {
    let mut ep = DriveEndPoint::build(end_point_type).expect("Could not build drive end point");
    ep.push_str(AND_SELECT);
    ep.push_str(params.as_str());
    ep
}

impl EP for Folder {
    fn ep(&self, end_point_type: DriveEndPoint) -> UrlResult {
        Ok(build_query(end_point_type, Select::Folder))
    }
}

impl EP for SpecialFolder {
    fn ep(&self, end_point_type: DriveEndPoint) -> UrlResult {
        Ok(build_query(end_point_type, Select::SpecialFolder))
    }
}
