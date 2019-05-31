use drive_test_tools::oauth::OAuthTestTool;
use rust_onedrive::oauth::scope;

#[test]
fn scopes() {
    let s = vec![
        scope::FILES_READ.to_string(),
        scope::FILES_READ_WRITE.to_string(),
        scope::FILES_READ_WRITE_SELECTED.to_string(),
    ];
    // Runs the method for each type of OAuth (token_flow, code_flow, etc.)
    OAuthTestTool::for_each_scope(&s);
}
