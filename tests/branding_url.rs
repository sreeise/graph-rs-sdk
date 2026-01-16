use std::sync::LazyLock;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

static ID_VEC: LazyLock<Vec<String>> = LazyLock::new(|| TestTools::random_strings(2, 20));

#[test]
fn audit_logs_url() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/branding/localizations/$count".to_string(),
        client.branding().get_localizations_count().url().path()
    );
}
