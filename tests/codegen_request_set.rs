use from_as::*;
use graph_codegen::parser::RequestSet;
use std::collections::{HashMap, HashSet};

#[test]
fn gen_links() {
    let request_set: RequestSet =
        RequestSet::from_file("./test_files/codegen/auditLogsRequestSet.json").unwrap();
    let mut links_set: HashSet<String> = HashSet::new();
    links_set.insert("auditLogRoot".into());
    links_set.insert("auditLogs".into());
    links_set.insert("fakeLog".into());
    links_set.insert("innerFakeLog".into());

    let mut client_links_map: HashMap<String, Vec<String>> = HashMap::new();
    client_links_map.insert(
        "auditLogs".into(),
        vec!["auditLogRoot".into(), "fakeLog".into()],
    );
    client_links_map.insert("fakeLog".into(), vec!["innerFakeLog".into()]);

    let (links, client_links) = request_set.method_links();
    assert_eq!(links_set, links);
    assert_eq!(client_links_map, client_links);
}
