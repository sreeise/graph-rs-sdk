extern crate serde_derive;

mod error;
mod transform;

pub use crate::error::RequestError;
pub use crate::transform::FromFile;
pub use crate::transform::FromYamlFile;
pub use crate::transform::ToFile;
pub use crate::transform::ToYamlFile;
pub use crate::transform::Transform;

pub mod prelude {
    pub use crate::error::RequestError;
    pub use crate::transform::FromFile;
    pub use crate::transform::FromYamlFile;
    pub use crate::transform::ToFile;
    pub use crate::transform::ToYamlFile;
    pub use crate::transform::Transform;
    pub use std::io::Write;
}
