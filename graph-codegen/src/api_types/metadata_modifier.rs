use crate::api_types::RequestMetadata;
use crate::parser::Request;
use crate::traits::INTERNAL_PATH_ID;
use from_as::*;
use std::collections::HashMap;
use std::io::{Read, Write};

pub enum UpdateOperationMap {
    PathStartsWith(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub enum MatchTarget {
    OperationId(String),
    OperationMap(String),
}

impl MatchTarget {
    pub fn matches(&self, request: &Request) -> bool {
        match self {
            MatchTarget::OperationMap(s) => {
                if request.operation_mapping.eq(s.as_str()) {
                    return true;
                }
            }
            MatchTarget::OperationId(s) => {
                if request.operation_id.eq(s.as_str()) {
                    return true;
                }
            }
        }
        false
    }

    pub fn modify(&self, request: &mut Request) {
        match self {
            MatchTarget::OperationMap(s) => {
                request.operation_mapping = s.to_string();
            }
            MatchTarget::OperationId(s) => {
                request.operation_id = s.to_string();
            }
        }
    }

    pub fn contains(&self, request: &Request) -> bool {
        match self {
            MatchTarget::OperationId(s) => request.operation_id.contains(s.as_str()),
            MatchTarget::OperationMap(s) => request.operation_mapping.contains(s.as_str()),
        }
    }

    #[allow(dead_code)]
    fn modify_contains(&self, pat: &str, request: &mut Request) {
        match self {
            MatchTarget::OperationMap(s) => {
                request.operation_mapping = request.operation_mapping.replace(pat, s);
            }
            MatchTarget::OperationId(s) => {
                request.operation_id = request.operation_id.replace(pat, s);
            }
        }
        request.param_size = INTERNAL_PATH_ID.captures_iter(&request.path).count();
        if request.path.contains("RID") && request.param_size > 0 {
            request.param_size -= 1;
        }
    }

    pub fn replace_match(&self, pat: &str, metadata: &mut RequestMetadata) {
        match self {
            MatchTarget::OperationId(replacement) => {
                metadata.operation_id = metadata.operation_id.replace(pat, replacement.as_str());
            }
            MatchTarget::OperationMap(replacement) => {
                metadata.operation_mapping = metadata
                    .operation_mapping
                    .replace(pat, replacement.as_str());
            }
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ModifierMap {
    pub map: HashMap<MatchTarget, Vec<MatchTarget>>,
}

impl ModifierMap {
    pub fn with_capacity(size: usize) -> ModifierMap {
        ModifierMap {
            map: HashMap::with_capacity(size),
        }
    }

    pub fn insert(&mut self, from: MatchTarget, to: Vec<MatchTarget>) {
        let to_clone = to.clone();
        self.map
            .entry(from)
            .and_modify(|vec| {
                vec.extend(to_clone);
            })
            .or_insert(to);
    }

    pub fn multi_operation_map(&mut self, to: &str, from: &[&'static str]) {
        for operation_map in from {
            self.insert(
                MatchTarget::OperationMap(operation_map.to_string()),
                vec![MatchTarget::OperationMap(to.to_string())],
            );
        }
    }

    pub fn operation_map(&mut self, from: &str, to: &str) {
        self.insert(
            MatchTarget::OperationMap(from.to_string()),
            vec![MatchTarget::OperationMap(to.to_string())],
        );
    }

    pub fn operation_id(&mut self, from: &str, to: &str) {
        self.insert(
            MatchTarget::OperationId(from.to_string()),
            vec![MatchTarget::OperationId(to.to_string())],
        );
    }
}

pub trait MetadataModifier {
    fn replace_operation_mapping(&mut self, replacement: &str);
    fn replace_operation_id(&mut self, replacement: &str);
    fn replace_operation_mapping_n(&mut self, pat: &str, replacement: &str, count: usize);
    fn replace_operation_id_n(&mut self, pat: &str, replacement: &str, count: usize);
    fn operation_mapping(&self) -> String;
    fn operation_id(&self) -> String;

    fn contains_match_target(&self, match_target: &MatchTarget) -> bool {
        match match_target {
            MatchTarget::OperationId(id) => {
                if self.operation_id().contains(id.as_str()) {
                    return true;
                }
            }
            MatchTarget::OperationMap(mapping) => {
                if self.operation_mapping().contains(mapping.as_str()) {
                    return true;
                }
            }
        }
        false
    }

    fn apply_match_target(&mut self, match_target: &MatchTarget) {
        match match_target {
            MatchTarget::OperationId(replacement) => {
                self.replace_operation_id(replacement.as_ref());
            }
            MatchTarget::OperationMap(replacement) => {
                self.replace_operation_mapping(replacement.as_ref());
            }
        }
    }

    fn apply_match_targets(&mut self, match_targets: &[MatchTarget]) {
        for mat_target in match_targets {
            self.apply_match_target(mat_target);
        }
    }

    fn force_update_targets(&mut self, modifier_map: &ModifierMap) {
        for (_match_target, match_target_vec) in modifier_map.map.iter() {
            self.apply_match_targets(match_target_vec);
        }
    }

    fn update_targets(&mut self, modifier_map: &ModifierMap) {
        for (match_target, match_target_vec) in modifier_map.map.iter() {
            for mat_target in match_target_vec.iter() {
                match match_target {
                    MatchTarget::OperationId(id) => {
                        if self.operation_id().eq(id.as_str()) {
                            self.apply_match_target(mat_target);
                        }
                    }
                    MatchTarget::OperationMap(mapping) => {
                        if self.operation_mapping().eq(mapping.as_str()) {
                            self.apply_match_target(mat_target);
                        }
                    }
                }
            }
        }
    }
}
