use graph_oauth::oauth::OAuth;
use graph_oauth::scopes::{FileScope, OfflineAccessScope};

#[test]
pub fn file_scope_enum() {
    let mut oauth = OAuth::token_flow();
    oauth
        .add_scope(FileScope::Read)
        .add_scope(FileScope::ReadAll)
        .add_scope(FileScope::ReadWrite)
        .add_scope(FileScope::ReadWriteAll)
        .add_scope(FileScope::ReadWriteAppFolder)
        .add_scope(FileScope::ReadSelected)
        .add_scope(FileScope::ReadWriteSelected);
    let test_scopes = vec![
        "Files.Read",
        "Files.Read.All",
        "Files.ReadWrite",
        "Files.ReadWrite.All",
        "Files.ReadWrite.AppFolder",
        "Files.Read.Selected",
        "Files.ReadWrite.Selected",
    ];
    assert_eq!(oauth.get_scopes(" "), test_scopes.join(" "));

    oauth.add_scope(OfflineAccessScope::WlOfflineAccess);
    assert!(oauth.contains_scope(OfflineAccessScope::WlOfflineAccess));
}

#[test]
pub fn scope_extend() {
    let mut oauth = OAuth::token_flow();
    let test_vec1 = vec![FileScope::Read, FileScope::ReadAll];
    oauth.extend_scopes(&test_vec1);

    let test_scopes = "Files.Read Files.Read.All";
    assert_eq!(oauth.get_scopes(" "), test_scopes);

    let test_vec2 = vec!["Files.ReadWrite", "Files.ReadWrite.All"];
    oauth.extend_scopes(&test_vec2);

    let test_scopes2 = "Files.Read Files.Read.All Files.ReadWrite Files.ReadWrite.All";
    assert_eq!(oauth.get_scopes(" "), test_scopes2);
}

#[test]
pub fn contains_scope() {
    let mut oauth = OAuth::token_flow();
    assert!(!oauth.contains_scope(FileScope::Read));
    oauth.add_scope(FileScope::Read);
    assert!(oauth.contains_scope(FileScope::Read.as_ref()));

    assert!(!oauth.contains_scope(FileScope::ReadWrite));
    oauth.add_scope(FileScope::ReadWrite);
    assert!(oauth.contains_scope(FileScope::ReadWrite));
}

#[test]
pub fn remove_scope() {
    let mut oauth = OAuth::token_flow();
    oauth.add_scope(FileScope::ReadWrite);
    assert_eq!(oauth.get_scopes(" "), FileScope::ReadWrite.as_ref());
    assert!(oauth.contains_scope(FileScope::ReadWrite));

    oauth.remove_scope(FileScope::ReadWrite);
    assert!(!oauth.contains_scope(FileScope::ReadWrite));
}
