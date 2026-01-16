use std::sync::LazyLock;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

static ID_VEC: LazyLock<Vec<String>> = LazyLock::new(|| TestTools::random_strings(2, 20));

#[test]
fn terms_of_use_url() {
    let client = Graph::new("");

    assert_eq!(
        format!(
            "/v1.0/directory/administrativeUnits/{}/members/$ref",
            ID_VEC[0]
        ),
        client
            .directory()
            .administrative_unit(ID_VEC[0].as_str())
            .members()
            .create_ref_members(&String::new())
            .url()
            .path()
    );
}
