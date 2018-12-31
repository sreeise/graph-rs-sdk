use crate::drive::endpoint::GRAPH_ENDPOINT;
use crate::drive::DriveResource;
use std::collections::HashMap;

pub struct DriveBuilder {
    url_params: HashMap<String, String>,
    root_set: bool,
}

impl DriveBuilder {
    pub fn new() -> DriveBuilder {
        DriveBuilder {
            url_params: HashMap::new(),
            root_set: false,
        }
    }

    pub fn reset(&mut self) {
        self.url_params.clear();
    }

    pub fn drive(&mut self) -> &mut DriveBuilder {
        if self.root_set {
            panic!("Drive item already set")
        }
        self.root_set = true;
        self.url_params
            .insert(String::from("DRIVE_ITEM"), DriveResource::Drives.as_str());
        self
    }

    pub fn group(&mut self) -> &mut DriveBuilder {
        if self.root_set {
            panic!("Drive item already set")
        }
        self.root_set = true;
        self.url_params
            .insert(String::from("DRIVE_ITEM"), DriveResource::Groups.as_str());
        self
    }

    pub fn sites(&mut self) -> &mut DriveBuilder {
        if self.root_set {
            panic!("Drive item already set")
        }

        self.root_set = true;
        self.url_params
            .insert(String::from("DRIVE_ITEM"), DriveResource::Sites.as_str());
        self
    }

    pub fn users(&mut self) -> &mut DriveBuilder {
        if self.root_set {
            panic!("Drive item already set")
        }
        self.root_set = true;
        self.url_params
            .insert(String::from("DRIVE_ITEM"), DriveResource::Users.as_str());
        self
    }

    pub fn drive_item(&mut self, drive_id: &str, item_id: &str) -> String {
        let mut str_builder = String::from(GRAPH_ENDPOINT);
        str_builder.push_str(drive_id);
        str_builder.push_str("/drive/items/");
        str_builder.push_str(item_id);
        str_builder
    }
}
