use std::collections::{BTreeSet, HashMap};
use from_as::*;
use inflector::Inflector;

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ResourceNames {
    names: BTreeSet<String>,
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

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct ResourceNameMapping {
    map: HashMap<String, BTreeSet<String>>,
}

impl ResourceNameMapping {
    pub fn new(map: HashMap<String, BTreeSet<String>>) -> ResourceNameMapping {
        ResourceNameMapping { map }
    }
}
