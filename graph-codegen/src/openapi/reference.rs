use crate::serde::de::Error;
use either::Either;
use from_as::*;
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use std::{
    collections::{HashMap, VecDeque},
    convert::TryFrom,
    io::{Read, Write},
};

/// [Reference Object](https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#referenceObject)
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile)]
pub struct Reference {
    /// REQUIRED. The reference identifier. This MUST be in the form of a URI.
    #[serde(rename = "$ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_: Option<String>,

    /// A short summary which by default SHOULD override that of the referenced
    /// component. If the referenced object-type does not allow a summary
    /// field, then this field has no effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// A description which by default SHOULD override that of the referenced
    /// component. CommonMark syntax MAY be used for rich text
    /// representation. If the referenced object-type does not allow a
    /// description field, then this field has no effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub fn either_t_or_reference<'de, D, T: DeserializeOwned>(
    deserializer: D,
) -> Result<Option<Either<T, Reference>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    let map = value.as_object().ok_or_else(|| {
        D::Error::custom(&format!(
            "Expected struct matching either `Parameter` or `Reference`. Found the following \
             serde_json::Value:\n{:#?}\n",
            value
        ))
    })?;

    if map.contains_key("$ref") {
        let reference: Reference = serde_json::from_value(value.clone()).unwrap();
        Ok(Some(Either::Right(reference)))
    } else {
        let t: T = serde_json::from_value(value.clone()).unwrap();
        Ok(Some(Either::Left(t)))
    }
}

pub fn either_t_map_right_or_reference<'de, D, T: DeserializeOwned>(
    deserializer: D,
) -> Result<Option<HashMap<String, Either<T, Reference>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    let map = value.as_object().ok_or_else(|| {
        D::Error::custom(&format!(
            "Expected struct matching either `Map<String, T>` or `Map<String, Reference>`. Found \
             the following serde_json::Value:\n{:#?}\n",
            value
        ))
    })?;
    let mut either_map = HashMap::new();

    for (key, inner_value) in map {
        let inner_map = inner_value.as_object().ok_or_else(|| {
            D::Error::custom(&format!(
                "Expected struct matching either `T` or `Reference`. Found the following \
                 serde_json::Value:\n{:#?}\n",
                value
            ))
        })?;
        if inner_map.contains_key("$ref") {
            let reference: Reference = serde_json::from_value(inner_value.clone()).unwrap();
            either_map.insert(key.clone(), Either::Right(reference));
        } else {
            let t: T = serde_json::from_value(inner_value.clone()).unwrap();
            either_map.insert(key.clone(), Either::Left(t));
        }
    }

    Ok(Some(either_map))
}

pub fn either_vec_t_or_reference<'de, D, T: DeserializeOwned>(
    deserializer: D,
) -> Result<VecDeque<Either<T, Reference>>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: serde_json::Value = Deserialize::deserialize(deserializer)?;
    let arr = value.as_array().ok_or_else(|| {
        D::Error::custom(&format!(
            "Value must be an array. Found the following serde_json::Value:\n{:#?}\n",
            value
        ))
    })?;

    let mut vec = VecDeque::new();
    for v in arr {
        let map = v.as_object().ok_or_else(|| {
            D::Error::custom(&format!(
                "Expected struct matching either `T` or `Reference`. Found the following \
                 serde_json::Value:\n{:#?}\n",
                v
            ))
        })?;

        if map.contains_key("$ref") {
            let reference: Reference = serde_json::from_value(v.clone()).unwrap();
            vec.push_back(Either::Right(reference));
        } else {
            let t: T = serde_json::from_value(v.clone()).unwrap();
            vec.push_back(Either::Left(t));
        }
    }
    Ok(vec)
}
