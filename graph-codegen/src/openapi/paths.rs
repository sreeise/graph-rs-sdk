use crate::openapi::PathItem;
use std::collections::BTreeMap;

/// [Paths Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#pathsObject)
pub type Paths = BTreeMap<String, PathItem>;
