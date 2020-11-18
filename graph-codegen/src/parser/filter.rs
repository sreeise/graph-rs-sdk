use crate::parser::error::ParserError;
use crate::parser::{Request, RequestMap, RequestSet};
use crate::traits::{Modify, INTERNAL_PATH_ID};
use from_as::*;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::collections::{HashMap, VecDeque};
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub enum FilterIgnore<'a> {
    PathContains(&'a str),
    PathContainsMulti(Vec<&'a str>),
    PathStartsWith(&'a str),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub enum Filter<'a> {
    None,
    PathStartsWith(&'a str),
    PathStartsWithMulti(Vec<&'a str>),
    PathEquals(&'a str),
    PathContains(&'a str),
    Regex(&'a str),
    IgnoreIf(FilterIgnore<'a>),
    MultiFilter(Vec<Filter<'a>>),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub enum SerializedFilter {
    None,
    PathStartsWith,
    PathEquals,
    Regex,
    Ignore,
    Multi,
    Modify,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct StoredFilter {
    filter: SerializedFilter,
    value: String,
}

impl StoredFilter {
    pub fn new(filter: SerializedFilter, value: &str) -> StoredFilter {
        StoredFilter {
            filter,
            value: value.into(),
        }
    }
}

impl From<Filter<'_>> for StoredFilter {
    fn from(filter: Filter<'_>) -> Self {
        match filter {
            Filter::None => StoredFilter::new(SerializedFilter::None, ""),
            Filter::PathStartsWith(s) => StoredFilter::new(SerializedFilter::PathStartsWith, s),
            Filter::PathEquals(s) => StoredFilter::new(SerializedFilter::PathEquals, s),
            Filter::Regex(s) => StoredFilter::new(SerializedFilter::Regex, s),
            Filter::IgnoreIf(filter_ignore) => match filter_ignore {
                FilterIgnore::PathStartsWith(s) => StoredFilter::new(SerializedFilter::Ignore, s),
                FilterIgnore::PathContains(s) => StoredFilter::new(SerializedFilter::Ignore, s),
                _ => StoredFilter::new(SerializedFilter::None, ""),
            },
            _ => StoredFilter::new(SerializedFilter::None, ""),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub enum UrlMatchTarget {
    // Modifies the paths that start with a resource and id by replacing
    // that part of the path for the client sdk generation. An example
    // would be where we have the path /drives/{{id}}/items. Passing the
    // value of 'drives' to this modifier would change the path to
    // /drives/{{{RID}}/items
    ResourceId(String, String, String),
}

impl UrlMatchTarget {
    pub fn resource_id(name: &str, replacement: &str) -> UrlMatchTarget {
        UrlMatchTarget::ResourceId(
            format!("/{}/{{{{id}}}}", name),
            replacement.to_string(),
            name.to_string(),
        )
    }

    pub fn matches(&self, request_map: &RequestMap) -> bool {
        match self {
            UrlMatchTarget::ResourceId(s, _replacement, _name) => {
                if request_map.path.starts_with(s.as_str()) {
                    return true;
                }
            },
        }
        false
    }

    pub fn matches_resource_id(&self, request_map: &RequestMap) -> bool {
        match self {
            UrlMatchTarget::ResourceId(s, _replacement, _name) => {
                if request_map.path.starts_with(&s.replace("id", "RID")) {
                    return true;
                }
            },
        }
        false
    }
}

impl Modify<RequestMap> for UrlMatchTarget {
    fn modify(&self, value: &mut RequestMap) {
        let path = value
            .path
            .replace("{{id}}", "{{RID}}")
            .replace("{{id2}}", "{{id}}")
            .replace("{{id3}}", "{{id2}}")
            .replace("{{id4}}", "{{id3}}");
        value.path = path.clone();
        for request in value.requests.iter_mut() {
            request.path = path.clone();
            request.has_rid = true;
            if request.param_size > 0 {
                request.param_size -= 1;
            }
        }
    }
}

impl Modify<RequestSet> for UrlMatchTarget {
    fn modify(&self, value: &mut RequestSet) {
        let mut matches = false;
        for request_map in value.set.iter() {
            if self.matches_resource_id(request_map) {
                matches = true;
                break;
            }
        }

        if matches {
            let (mut rid_request_set, non_rid_request_set) = value.split_on_resource_id();
            match self {
                UrlMatchTarget::ResourceId(_s, replacement, _name) => {
                    let mut request_map_vec: Vec<RequestMap> =
                        non_rid_request_set.set.into_iter().collect();
                    for request_map in request_map_vec.iter_mut() {
                        let param_size = INTERNAL_PATH_ID.captures_iter(&request_map.path).count();
                        for request in request_map.requests.iter_mut() {
                            if let Some(index) = request.operation_mapping.find('.') {
                                request
                                    .operation_mapping
                                    .replace_range(0..index, replacement);
                            } else {
                                request.operation_mapping = replacement.to_string();
                            }

                            request.param_size = param_size;
                            if request.path.contains("RID") && request.param_size > 0 {
                                request.param_size -= 1;
                            }
                        }
                    }

                    rid_request_set.set.extend(request_map_vec);
                    value.set = rid_request_set.set;
                },
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct SecondaryTarget {
    match_target: MatchTarget,
    pat: String,
}

impl SecondaryTarget {
    pub fn new(pat: &str, match_target: MatchTarget) -> SecondaryTarget {
        SecondaryTarget {
            match_target,
            pat: pat.to_string(),
        }
    }

    pub fn modify(&self, request: &mut Request) {
        if self.match_target.contains(request) {
            self.match_target.modify_contains(&self.pat, request);
        }
    }
}

impl From<MatchTarget> for SecondaryTarget {
    fn from(match_target: MatchTarget) -> Self {
        SecondaryTarget::new("", match_target)
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct SecondaryModifierMap {
    pub secondary_targets: VecDeque<SecondaryTarget>,
}

impl SecondaryModifierMap {
    pub fn with_capacity(capacity: usize) -> SecondaryModifierMap {
        SecondaryModifierMap {
            secondary_targets: VecDeque::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, pat: &str, match_target: MatchTarget) {
        self.secondary_targets
            .push_back(SecondaryTarget::new(pat, match_target));
    }

    pub fn modify(&self, request: &mut Request) {
        for target in self.secondary_targets.iter() {
            target.modify(request);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub enum MatchTarget {
    Tag(String),
    OperationId(String),
    OperationMap(String),
    TagAndOperationMap(String),
    TagOrOperationMap(String),
}

impl MatchTarget {
    pub fn matches(&self, request: &Request) -> bool {
        match self {
            MatchTarget::OperationMap(s) => {
                if request.operation_mapping.eq(s.as_str()) {
                    return true;
                }
            },
            MatchTarget::Tag(s) => {
                if request.tag.eq(s.as_str()) {
                    return true;
                }
            },
            MatchTarget::TagAndOperationMap(s) => {
                if request.tag.eq(s.as_str()) && request.operation_mapping.eq(s.as_str()) {
                    return true;
                }
            },
            MatchTarget::TagOrOperationMap(s) => {
                if request.tag.eq(s.as_str()) || request.operation_mapping.eq(s.as_str()) {
                    return true;
                }
            },
            MatchTarget::OperationId(s) => {
                if request.operation_id.eq(s.as_str()) {
                    return true;
                }
            },
        }
        false
    }

    pub fn modify(&self, request: &mut Request) {
        match self {
            MatchTarget::OperationMap(s) => {
                request.operation_mapping = s.to_string();
            },
            MatchTarget::Tag(s) => {
                request.tag = s.to_string();
            },
            MatchTarget::TagAndOperationMap(s) => {
                request.tag = s.to_string();
                request.operation_mapping = s.to_string();
            },
            MatchTarget::TagOrOperationMap(s) => {
                request.tag = s.to_string();
                request.operation_mapping = s.to_string();
            },
            MatchTarget::OperationId(s) => {
                request.operation_id = s.to_string();
            },
        }
    }

    pub fn contains(&self, request: &Request) -> bool {
        match self {
            MatchTarget::Tag(s) => request.tag.contains(s.as_str()),
            MatchTarget::OperationId(s) => request.operation_id.contains(s.as_str()),
            MatchTarget::OperationMap(s) => request.operation_mapping.contains(s.as_str()),
            MatchTarget::TagAndOperationMap(s) => {
                request.tag.contains(s.as_str()) && request.operation_mapping.contains(s.as_str())
            },
            MatchTarget::TagOrOperationMap(s) => {
                request.tag.contains(s.as_str()) || request.operation_mapping.contains(s.as_str())
            },
        }
    }

    fn modify_contains(&self, pat: &str, request: &mut Request) {
        match self {
            MatchTarget::OperationMap(s) => {
                request.operation_mapping = request.operation_mapping.replace(pat, s);
            },
            MatchTarget::Tag(s) => {
                request.tag = request.tag.replace(pat, s);
            },
            MatchTarget::TagAndOperationMap(s) => {
                request.tag = request.tag.replace(pat, s);
                request.operation_mapping = request.operation_mapping.replace(pat, s);
            },
            MatchTarget::TagOrOperationMap(s) => {
                request.tag = request.tag.replace(pat, s);
                request.operation_mapping = request.operation_mapping.replace(pat, s);
            },
            MatchTarget::OperationId(s) => {
                request.operation_id = request.operation_id.replace(pat, s);
            },
        }
        request.param_size = INTERNAL_PATH_ID.captures_iter(&request.path).count();
        if request.path.contains("RID") && request.param_size > 0 {
            request.param_size -= 1;
        }
    }
}

impl ToString for MatchTarget {
    fn to_string(&self) -> String {
        match self {
            MatchTarget::Tag(s) => format!("Tag:{}", s),
            MatchTarget::OperationMap(s) => format!("OperationMap:{}", s),
            MatchTarget::TagAndOperationMap(s) => format!("TagAndOperationMap:{}", s),
            MatchTarget::TagOrOperationMap(s) => format!("TagAndOperationMap:{}", s),
            MatchTarget::OperationId(s) => format!("OperationId:{}", s),
        }
    }
}

impl TryFrom<String> for MatchTarget {
    type Error = ParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut vec: VecDeque<&str> = value.split(':').collect();
        vec.retain(|s| !s.is_empty());
        if vec.len() == 2 {
            let key = vec.pop_front().unwrap();
            let value = vec.pop_front().unwrap();
            match key {
                "Tag" => Ok(MatchTarget::Tag(value.to_string())),
                "OperationMap" => Ok(MatchTarget::OperationMap(value.to_string())),
                "TagAndOperationMap" => Ok(MatchTarget::TagAndOperationMap(value.to_string())),
                "TagOrOperationMap" => Ok(MatchTarget::TagOrOperationMap(value.to_string())),
                "OperationId" => Ok(MatchTarget::OperationId(value.to_string())),
                _ => Err(ParserError::DeserializeMatchTarget),
            }
        } else if vec.len() == 1 {
            let key = vec.pop_front().unwrap();
            match key {
                "Tag" => Ok(MatchTarget::Tag(String::new())),
                "OperationMap" => Ok(MatchTarget::OperationMap(String::new())),
                "TagAndOperationMap" => Ok(MatchTarget::TagAndOperationMap(String::new())),
                "TagOrOperationMap" => Ok(MatchTarget::TagOrOperationMap(String::new())),
                "OperationId" => Ok(MatchTarget::OperationId(String::new())),
                _ => Err(ParserError::DeserializeMatchTarget),
            }
        } else {
            Ok(MatchTarget::Tag(String::new()))
        }
    }
}

impl FromStr for MatchTarget {
    type Err = ParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MatchTarget::try_from(s.to_string())
    }
}

#[derive(Default, Debug, Clone, FromFile, AsFile)]
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

struct ModifierMapVisitor {
    marker: PhantomData<fn() -> ModifierMap>,
}

impl ModifierMapVisitor {
    fn new() -> Self {
        ModifierMapVisitor {
            marker: PhantomData,
        }
    }
}

impl<'de> Visitor<'de> for ModifierMapVisitor {
    type Value = ModifierMap;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(
            "a HashMap<String, String> where the key is in the \
        format key:value and key is the MatchTarget name and the value is the enum's value",
        )
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut map = ModifierMap::with_capacity(access.size_hint().unwrap_or(0));

        while let Some((key, value)) = access.next_entry::<String, Vec<MatchTarget>>()? {
            let mt = MatchTarget::from_str(key.as_str()).unwrap();
            map.map.insert(mt, value);
        }

        Ok(map)
    }
}

impl Serialize for ModifierMap {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (k, v) in &self.map {
            map.serialize_entry(&k.to_string(), &v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for ModifierMap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ModifierMapVisitor::new())
    }
}
