use crate::GraphResponse;
use graph_error::GraphFailure;
use std::marker::PhantomData;

#[allow(clippy::large_enum_variant)]
pub enum Delta<T> {
    Next(GraphResponse<T>),
    Done(Option<GraphFailure>),
}

impl<T> std::fmt::Debug for Delta<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Delta::Next(response) => {
                f.debug_struct("GraphResponse")
                    .field("status", &response.status())
                    .field("headers", &response.headers())
                    .field("body", &"[REDACTED]")
                    .finish()
            },
            Delta::Done(err) => {
                if let Some(err) = err {
                    err.fmt(f)
                } else {
                    f.write_str("Error: None")
                }
            }
        }
    }
}

#[derive(Default)]
pub struct DeltaPhantom<T> {
    phantom: PhantomData<T>,
}
