use std::sync::LazyLock;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

static ID_VEC: LazyLock<Vec<String>> = LazyLock::new(|| TestTools::random_strings(4, 20));

#[test]
pub fn permission_grants() {
    let client = Graph::new("");
    assert_eq!(
        "/v1.0/permissionGrants/getByIds".to_string(),
        client
            .permission_grants()
            .get_by_ids(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/permissionGrants/{}/getMemberGroups", ID_VEC[0]),
        client
            .permission_grant(ID_VEC[0].as_str())
            .get_member_groups(&String::new())
            .url()
            .path()
    );
}
