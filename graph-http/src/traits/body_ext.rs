use crate::api_impl::{BodyRead, FileConfig};
use graph_error::GraphResult;

pub trait BodyExt<RHS = Self> {
    fn into_body(self) -> GraphResult<BodyRead>;
}

impl<U> BodyExt for &U
where
    U: serde::Serialize,
{
    fn into_body(self) -> GraphResult<BodyRead> {
        BodyRead::from_serialize(self)
    }
}

impl BodyExt for &FileConfig {
    fn into_body(self) -> GraphResult<BodyRead> {
        BodyRead::try_from(self)
    }
}

impl BodyExt for reqwest::Body {
    fn into_body(self) -> GraphResult<BodyRead> {
        Ok(BodyRead::from(self))
    }
}

impl BodyExt for reqwest::blocking::Body {
    fn into_body(self) -> GraphResult<BodyRead> {
        Ok(BodyRead::from(self))
    }
}
