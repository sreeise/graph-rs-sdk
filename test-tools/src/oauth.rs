use graph_rs_sdk::oauth::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OAuthTestTool;

impl OAuthTestTool {
    pub fn oauth_contains_credentials(oauth: &mut OAuthSerializer, credentials: &[OAuthParameter]) {
        for oac in credentials.iter() {
            assert!(oauth.contains(*oac));
        }
    }

    pub fn for_each_scope(s: &[String]) {
        OAuthTestTool::for_each_fn_scope(OAuthTestTool::join_scopes, s);
        OAuthTestTool::for_each_fn_scope(OAuthTestTool::contains_scopes, s);
        OAuthTestTool::for_each_fn_scope(OAuthTestTool::remove_scopes, s);
        OAuthTestTool::for_each_fn_scope(OAuthTestTool::get_scopes, s);
        OAuthTestTool::for_each_fn_scope(OAuthTestTool::clear_scopes, s);
        OAuthTestTool::for_each_fn_scope(OAuthTestTool::distinct_scopes, s);
    }

    pub fn for_each_fn_scope<F>(mut func: F, scopes: &[String])
    where
        F: FnMut(&mut OAuthSerializer, &[String]),
    {
        let mut oauth = OAuthSerializer::new();
        oauth.extend_scopes(scopes);
        func(&mut oauth, scopes)
    }

    pub fn join_scopes(oauth: &mut OAuthSerializer, s: &[String]) {
        assert_eq!(s.join(" "), oauth.join_scopes(" "));
    }

    pub fn contains_scopes(oauth: &mut OAuthSerializer, s: &[String]) {
        for string in s {
            assert!(oauth.contains_scope(string.as_str()));
        }
    }

    pub fn remove_scopes(oauth: &mut OAuthSerializer, s: &[String]) {
        for string in s {
            oauth.remove_scope(string.as_str());
            assert!(!oauth.contains_scope(string));
        }
    }

    pub fn get_scopes(oauth: &mut OAuthSerializer, s: &[String]) {
        assert_eq!(
            s,
            oauth
                .get_scopes()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .as_slice()
        )
    }

    pub fn clear_scopes(oauth: &mut OAuthSerializer, s: &[String]) {
        OAuthTestTool::join_scopes(oauth, s);
        assert!(!oauth.get_scopes().is_empty());
        oauth.clear_scopes();
        assert!(oauth.get_scopes().is_empty())
    }

    pub fn distinct_scopes(oauth: &mut OAuthSerializer, s: &[String]) {
        assert_eq!(s.len(), oauth.get_scopes().len());
        let s0 = &s[0];
        oauth.add_scope(s0.as_str());
        assert_eq!(s.len(), oauth.get_scopes().len());
        oauth.extend_scopes(s);
        assert_eq!(s.len(), oauth.get_scopes().len());
    }
}
