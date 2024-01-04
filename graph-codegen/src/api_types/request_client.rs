use crate::api_types::RequestMetadata;
use crate::traits::RequestParser;
use from_as::*;
use std::collections::{BTreeMap, VecDeque};
use std::convert::TryFrom;
use std::io::{Read, Write};

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestClient {
    pub name: String,
    pub requests: VecDeque<RequestMetadata>,
    pub links: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct RequestClientList {
    pub clients: BTreeMap<String, VecDeque<RequestMetadata>>,
    pub client_list: VecDeque<RequestClient>,
}

impl RequestClientList {
    pub fn client_links(&self) -> BTreeMap<String, Vec<String>> {
        let mut links_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (_name, metadata) in self.clients.iter() {
            if let Some(m) = metadata.front() {
                let links = m.operation_mapping.struct_links();
                links_map.extend(links);
            }
        }
        links_map
    }
}

impl From<VecDeque<RequestMetadata>> for RequestClientList {
    fn from(metadata_vec: VecDeque<RequestMetadata>) -> Self {
        let mut clients: BTreeMap<String, VecDeque<RequestMetadata>> = BTreeMap::new();

        for metadata in metadata_vec.iter() {
            clients
                .entry(metadata.parent.to_string())
                .and_modify(|vec| {
                    vec.push_back(metadata.clone());
                })
                .or_insert_with(|| {
                    let mut vec = VecDeque::new();
                    vec.push_back(metadata.clone());
                    vec
                });
        }

        let mut links_map: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (_name, metadata) in clients.iter() {
            if let Some(m) = metadata.front() {
                let links = m.operation_mapping.struct_links();
                links_map.extend(links);
            }
        }

        let mut client_list: VecDeque<RequestClient> = VecDeque::new();
        for (name, request_metadata) in clients.iter() {
            let mut links: Vec<String> = Vec::new();
            if let Some(l) = links_map.get(name.as_str()) {
                links = l.clone();
            }

            client_list.push_back(RequestClient {
                name: name.to_string(),
                requests: request_metadata.clone(),
                links,
            });
        }

        RequestClientList {
            clients,
            client_list,
        }
    }
}
