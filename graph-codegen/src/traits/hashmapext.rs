use crate::parser::RequestMap;
use std::collections::{BTreeSet, HashMap};

pub trait HashMapExt<T, RHS = Self> {
    fn entry_modify_insert(&mut self, s: String, value: T);
}

impl HashMapExt<RequestMap> for HashMap<String, Vec<RequestMap>> {
    fn entry_modify_insert(&mut self, s: String, request_map: RequestMap) {
        self.entry(s)
            .and_modify(|vec| {
                vec.push(request_map.clone());
            })
            .or_insert_with(|| vec![request_map.clone()]);
    }
}

impl HashMapExt<String> for HashMap<String, Vec<String>> {
    fn entry_modify_insert(&mut self, s: String, value: String) {
        self.entry(s)
            .and_modify(|vec| {
                vec.push(value.clone());
                vec.retain(|s| !s.is_empty());
            })
            .or_insert_with(|| {
                let mut vec: Vec<String> = vec![value.clone()];
                vec.retain(|s| !s.is_empty());
                vec
            });
    }
}

impl HashMapExt<String> for HashMap<String, BTreeSet<String>> {
    fn entry_modify_insert(&mut self, s: String, value: String) {
        self.entry(s)
            .and_modify(|set| {
                set.insert(value.clone());
            })
            .or_insert_with(|| {
                let mut names: BTreeSet<String> = BTreeSet::new();
                names.insert(value.clone());
                names
            });
    }
}
