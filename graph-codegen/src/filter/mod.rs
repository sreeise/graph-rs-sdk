use from_as::*;
use std::io::{Read, Write};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub enum FilterIgnore {
    PathContains(String),
    PathContainsMulti(Vec<String>),
    PathStartsWith(String),
    PathEquals(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub enum Filter {
    None,
    PathStartsWith(String),
    PathStartsWithMulti(Vec<String>),
    PathEquals(String),
    PathContains(String),
    Regex(String),
    IgnoreIf(FilterIgnore),
    MultiFilter(Vec<Filter>),
}
