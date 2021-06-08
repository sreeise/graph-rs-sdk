use serde::{Deserialize, Serialize};

mod test_server;

pub use test_server::TestServer;

#[derive(Debug, Serialize, Deserialize)]
pub struct RedirectQuery {
    pub code: String,
}
