#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
pub fn sites_lists() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/sites/{}/lists/{}", ID_VEC[0], ID_VEC[1]),
        client
            .site(ID_VEC[0].as_str())
            .list(ID_VEC[1].as_str())
            .get_lists()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/sites/{}/lists/{}/items/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2]
        ),
        client
            .site(ID_VEC[0].as_str())
            .list(ID_VEC[1].as_str())
            .item(ID_VEC[2].as_str())
            .get_items()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/lists", ID_VEC[0]),
        client
            .site(ID_VEC[0].as_str())
            .lists()
            .create_lists(&String::new())
            .url()
            .path()
    );
}

#[test]
pub fn term_store() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/sites/{}/termStore", ID_VEC[0]),
        client
            .site(ID_VEC[0].as_str())
            .term_store()
            .get_term_store()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/termStore/groups/{}", ID_VEC[0], ID_VEC[1]),
        client
            .site(ID_VEC[0].as_str())
            .term_store()
            .group(ID_VEC[1].as_str())
            .get_groups()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/termStore/groups", ID_VEC[0]),
        client
            .site(ID_VEC[0].as_str())
            .term_store()
            .groups()
            .list_groups()
            .url()
            .path()
    );
}

#[test]
pub fn term_stores() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/sites/{}/termStores/$count", ID_VEC[0]),
        client
            .site(ID_VEC[0].as_str())
            .term_stores()
            .get_term_stores_count()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/termStores/{}", ID_VEC[0], ID_VEC[1]),
        client
            .site(ID_VEC[0].as_str())
            .term_stores_id(ID_VEC[1].as_str())
            .get_term_stores()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/sites/{}/termStores/{}/sets/{}/terms",
            ID_VEC[0],
            ID_VEC[1].as_str(),
            ID_VEC[2].as_str()
        ),
        client
            .site(ID_VEC[0].as_str())
            .term_stores_id(ID_VEC[1].as_str())
            .set(ID_VEC[2].as_str())
            .terms()
            .list_terms()
            .url()
            .path()
    );
}

#[test]
fn term_store_sets() {
    let client = Graph::new("");

    assert_eq!(
        format!(
            "/v1.0/sites/{}/termStore/sets/{}/terms",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .site(ID_VEC[0].as_str())
            .term_store()
            .set(ID_VEC[1].as_str())
            .terms()
            .list_terms()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/termStore/sets/{}", ID_VEC[0], ID_VEC[1]),
        client
            .site(ID_VEC[0].as_str())
            .term_store()
            .set(ID_VEC[1].as_str())
            .get_sets()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/sites/{}/termStore/sets/{}/terms/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2]
        ),
        client
            .site(ID_VEC[0].as_str())
            .term_store()
            .set(ID_VEC[1].as_str())
            .term(ID_VEC[2].as_str())
            .get_terms()
            .url()
            .path()
    );
}

#[test]
fn term_store_sets_parent_group() {
    let client = Graph::new("");

    assert_eq!(
        format!(
            "/v1.0/sites/{}/termStore/sets/{}/parentGroup",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .site(ID_VEC[0].as_str())
            .term_store()
            .set(ID_VEC[1].as_str())
            .parent_group()
            .get_parent_group()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/sites/{}/termStore/sets/{}/parentGroup/sets/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2]
        ),
        client
            .site(ID_VEC[0].as_str())
            .term_store()
            .set(ID_VEC[1].as_str())
            .parent_group()
            .set(ID_VEC[2].as_str())
            .get_sets()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/sites/{}/termStore/sets/{}/parentGroup/sets/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2]
        ),
        client
            .site(ID_VEC[0].as_str())
            .term_store()
            .set(ID_VEC[1].as_str())
            .parent_group()
            .update_sets(ID_VEC[2].as_str(), &String::new())
            .url()
            .path()
    );
}

#[test]
fn sites_items() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/sites/{}/items", ID_VEC[0]),
        client
            .site(ID_VEC[0].as_str())
            .items()
            .list_items()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/sites/{}/lists/{}/items/{}/versions",
            ID_VEC[0], ID_VEC[1], ID_VEC[2]
        ),
        client
            .site(ID_VEC[0].as_str())
            .list(ID_VEC[1].as_str())
            .item(ID_VEC[2].as_str())
            .versions()
            .list_versions()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/sites/{}/lists/{}/items/{}/versions/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2], ID_VEC[3]
        ),
        client
            .site(ID_VEC[0].as_str())
            .list(ID_VEC[1].as_str())
            .item(ID_VEC[2].as_str())
            .version(ID_VEC[3].as_str())
            .get_versions()
            .url()
            .path()
    );
}
