use crate::parser::error::ParserError;
use crate::parser::Request;
use from_as::*;
use serde::de::{Deserialize, Deserializer, MapAccess, Visitor};

use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};
use std::collections::{HashMap, VecDeque};

use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub enum Filter<'a> {
    None,
    PathStartsWith(&'a str),
    PathEquals(&'a str),
    Regex(&'a str),
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub enum SerializedFilter {
    None,
    PathStartsWith,
    PathEquals,
    Regex,
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Eq, PartialEq, Hash)]
pub enum MatchTarget {
    Tag(String),
    OperationMap(String),
}

impl MatchTarget {
    pub fn matches(&self, request: &mut Request) -> bool {
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
        }
    }
}

impl ToString for MatchTarget {
    fn to_string(&self) -> String {
        match self {
            MatchTarget::Tag(s) => format!("Tag:{}", s),
            MatchTarget::OperationMap(s) => format!("OperationMap:{}", s),
        }
    }
}

impl TryFrom<String> for MatchTarget {
    type Error = ParserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut vec: VecDeque<&str> = value.split(':').collect();
        vec.retain(|s| !s.is_empty());
        let key = vec.pop_front().unwrap();
        let value = vec.pop_front().unwrap();
        match key {
            "Tag" => Ok(MatchTarget::Tag(value.to_string())),
            "OperationMap" => Ok(MatchTarget::OperationMap(value.to_string())),
            _ => Err(ParserError::DeserializeMatchTarget),
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
