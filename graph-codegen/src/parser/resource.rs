use crate::parser::RequestSet;
use from_as::*;
use inflector::Inflector;
use std::collections::{BTreeSet, HashMap, VecDeque};

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ResourceNames {
    pub names: BTreeSet<String>,
}

impl ResourceNames {
    pub fn new(names: BTreeSet<String>) -> ResourceNames {
        ResourceNames { names }
    }

    pub fn sort(&mut self) {
        let mut v: Vec<String> = self.names.clone().into_iter().collect();
        v = v.iter().map(|s| s.to_pascal_case()).collect();
        v.sort();
        self.names = v.into_iter().collect();
    }
}

impl From<RequestSet> for ResourceNames {
    fn from(request_set: RequestSet) -> Self {
        ResourceNames::from(&request_set)
    }
}

impl From<&RequestSet> for ResourceNames {
    fn from(request_set: &RequestSet) -> Self {
        let mut resource = ResourceNames::new(BTreeSet::new());
        let mut names: Vec<String> = Vec::new();

        for request_map in request_set.set.iter() {
            let mut vec: VecDeque<&str> = request_map.path.split('/').collect();
            vec.retain(|s| !s.is_empty());
            if let Some(name) = vec.pop_front() {
                if !name.is_empty() {
                    names.push(name.to_pascal_case());
                }
            }
        }

        names.sort();
        for name in names.iter() {
            resource.names.insert(name.to_string());
        }

        resource
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ResourceNameMapping {
    pub map: HashMap<String, BTreeSet<String>>,
}

impl ResourceNameMapping {
    pub fn new(map: HashMap<String, BTreeSet<String>>) -> ResourceNameMapping {
        ResourceNameMapping { map }
    }
}
