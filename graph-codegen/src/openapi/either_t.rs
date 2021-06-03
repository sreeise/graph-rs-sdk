use crate::{
    openapi::Reference,
    serde::{
        de::{DeserializeOwned, Error},
        Deserialize, Deserializer,
    },
};
use either::Either;
use from_as::*;
use std::{
    convert::TryFrom,
    io::{Read, Write},
};

#[derive(Debug, Clone, Serialize, FromFile, AsFile)]
pub struct EitherT<T, U> {
    either: Either<T, U>,
}

impl<T, U> EitherT<T, U> {
    pub fn left(t: T) -> EitherT<T, U> {
        EitherT {
            either: Either::Left(t),
        }
    }

    pub fn right(u: U) -> EitherT<T, U> {
        EitherT {
            either: Either::Right(u),
        }
    }

    pub fn is_left(&self) -> bool {
        self.either.is_left()
    }

    pub fn is_right(&self) -> bool {
        self.either.is_right()
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for EitherT<T, Reference> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
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
            Ok(EitherT::right(reference))
        } else {
            let t: T = serde_json::from_value(value.clone()).unwrap();
            Ok(EitherT::left(t))
        }
    }
}

/*
impl<'de, T: DeserializeOwned, U: DeserializeOwned> Deserialize<'de> for EitherT<T, U> {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        let value: serde_json::Value = Deserialize::deserialize(deserializer)?;


        let t: T = serde_json::from_value(value.clone());
        let u: U = serde_json::from_value(value.clone());

        if let Ok(u) = u {
            return Ok(EitherT::right(u));
        }

        if let Ok(t) = t {
            return Ok(EitherT::left(t));
        }

        return D::Error::custom("Could not deserialize T or U from EitherT<T, U>");
    }
}

 */
