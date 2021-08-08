use std::collections::{VecDeque, BTreeMap};
use crate::api_types::RequestMetadata;
use from_as::*;
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestClientList {
    pub clients: BTreeMap<String, VecDeque<RequestMetadata>>
}

impl From<VecDeque<RequestMetadata>> for RequestClientList {
    fn from(metadata_vec: VecDeque<RequestMetadata>) -> Self {
        let mut clients: BTreeMap<String, VecDeque<RequestMetadata>> = BTreeMap::new();

        for metadata in metadata_vec.iter() {
            clients.entry(metadata.parent.to_string())
                .and_modify(|vec| {
                    vec.push_back(metadata.clone());
                }).or_insert_with(|| {
                let mut vec = VecDeque::new();
                vec.push_back(metadata.clone());
                vec
            });
        }

        RequestClientList {
            clients
        }
    }
}
