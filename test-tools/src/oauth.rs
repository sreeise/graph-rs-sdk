use graph_rs_sdk::oauth::*;
use std::borrow::Cow;
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OAuthTestTool;

impl OAuthTestTool {
    fn match_grant_credential(grant_request: GrantRequest) -> OAuthCredential {
        match grant_request {
            GrantRequest::Authorization => OAuthCredential::AuthorizationUrl,
            GrantRequest::AccessToken => OAuthCredential::AccessTokenUrl,
            GrantRequest::RefreshToken => OAuthCredential::RefreshTokenUrl,
        }
    }

    pub fn oauth_query_uri_test(
        oauth: &mut OAuth,
        grant_type: GrantType,
        grant_request: GrantRequest,
        includes: Vec<OAuthCredential>,
    ) {
        let mut url = String::new();
        if grant_request.eq(&GrantRequest::AccessToken) {
            let mut atu = oauth.get(OAuthCredential::AccessTokenUrl).unwrap();
            if !atu.ends_with('?') {
                atu.push('?');
            }
            url.push_str(atu.as_str());
        } else if grant_request.eq(&GrantRequest::RefreshToken) {
            let mut rtu = oauth.get(OAuthCredential::RefreshTokenUrl).unwrap();
            if !rtu.ends_with('?') {
                rtu.push('?');
            }
            url.push_str(rtu.as_str());
        }
        url.push_str(
            oauth
                .encode_uri(grant_type, grant_request)
                .unwrap()
                .as_str(),
        );
        let parsed_url = Url::parse(url.as_str()).unwrap();
        let mut cow_cred: Vec<(Cow<str>, Cow<str>)> = Vec::new();
        let mut cow_cred_false: Vec<(Cow<str>, Cow<str>)> = Vec::new();
        let not_includes = OAuthTestTool::credentials_not_including(&includes);

        for oac in OAuthCredential::iter() {
            if oauth.contains(oac) && includes.contains(&oac) && !not_includes.contains(&oac) {
                if oac.eq(&OAuthCredential::Scopes) {
                    let s = oauth.join_scopes(" ");
                    cow_cred.push((Cow::from(oac.alias()), Cow::from(s.to_owned())));
                } else if !oac.eq(&OAuthTestTool::match_grant_credential(grant_request)) {
                    let s = oauth.get(oac).unwrap();
                    cow_cred.push((Cow::from(oac.alias()), Cow::from(s.to_owned())));
                }
            } else if oauth.contains(oac) && not_includes.contains(&oac) {
                if oac.eq(&OAuthCredential::Scopes) {
                    let s = oauth.join_scopes(" ");
                    cow_cred.push((Cow::from(oac.alias()), Cow::from(s.to_owned())));
                } else if !oac.eq(&OAuthTestTool::match_grant_credential(grant_request)) {
                    let s = oauth.get(oac).unwrap();
                    cow_cred_false.push((Cow::from(oac.alias()), Cow::from(s.to_owned())));
                }
            }
        }

        let query = parsed_url.query().unwrap();
        let parse = url::form_urlencoded::parse(query.as_bytes());

        for query in parse {
            assert!(cow_cred.contains(&query));
            assert!(!cow_cred_false.contains(&query));
        }
    }

    fn credentials_not_including(included: &[OAuthCredential]) -> Vec<OAuthCredential> {
        let mut vec = Vec::new();
        for oac in OAuthCredential::iter() {
            if !included.contains(&oac) {
                vec.push(oac);
            }
        }

        vec
    }

    pub fn oauth_contains_credentials(oauth: &mut OAuth, credentials: &[OAuthCredential]) {
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
        F: FnMut(&mut OAuth, &[String]),
    {
        let mut oauth = OAuth::new();
        oauth.extend_scopes(scopes);
        func(&mut oauth, scopes)
    }

    pub fn join_scopes(oauth: &mut OAuth, s: &[String]) {
        assert_eq!(s.join(" "), oauth.join_scopes(" "));
    }

    pub fn contains_scopes(oauth: &mut OAuth, s: &[String]) {
        for string in s {
            assert!(oauth.contains_scope(string.as_str()));
        }
    }

    pub fn remove_scopes(oauth: &mut OAuth, s: &[String]) {
        for string in s {
            oauth.remove_scope(string.as_str());
            assert!(!oauth.contains_scope(string));
        }
    }

    pub fn get_scopes(oauth: &mut OAuth, s: &[String]) {
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

    pub fn clear_scopes(oauth: &mut OAuth, s: &[String]) {
        OAuthTestTool::join_scopes(oauth, s);
        assert!(!oauth.get_scopes().is_empty());
        oauth.clear_scopes();
        assert!(oauth.get_scopes().is_empty())
    }

    pub fn distinct_scopes(oauth: &mut OAuth, s: &[String]) {
        assert_eq!(s.len(), oauth.get_scopes().len());
        let s0 = &s[0];
        oauth.add_scope(s0.as_str());
        assert_eq!(s.len(), oauth.get_scopes().len());
        oauth.extend_scopes(s);
        assert_eq!(s.len(), oauth.get_scopes().len());
    }
}
