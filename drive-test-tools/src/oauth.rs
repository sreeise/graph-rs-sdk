use rust_onedrive::oauth::IntoEnumIterator;
use rust_onedrive::oauth::{GrantRequest, OAuth, OAuthCredential};
use std::borrow::Cow;
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OAuthTestTool;

impl OAuthTestTool {
    fn match_grant_credential(grant_request: GrantRequest) -> OAuthCredential {
        match grant_request {
            GrantRequest::Authorization => OAuthCredential::AuthorizeURL,
            GrantRequest::AccessToken => OAuthCredential::AccessTokenURL,
            GrantRequest::RefreshToken => OAuthCredential::RefreshTokenURL,
        }
    }

    pub fn oauth_query_uri_test(
        oauth: &mut OAuth,
        grant_request: GrantRequest,
        includes: Vec<OAuthCredential>,
    ) {
        let mut url = String::new();
        if grant_request.eq(&GrantRequest::AccessToken) {
            let mut atu = oauth.get(OAuthCredential::AccessTokenURL).unwrap();
            if !atu.ends_with('?') {
                atu.push('?')
            }
            url.push_str(atu.as_str());
        } else if grant_request.eq(&GrantRequest::RefreshToken) {
            let mut rtu = oauth.get(OAuthCredential::RefreshTokenURL).unwrap();
            if !rtu.ends_with('?') {
                rtu.push('?')
            }
            url.push_str(rtu.as_str());
        }
        url.push_str(oauth.encode_uri(grant_request).unwrap().as_str());
        let parsed_url = Url::parse(url.as_str()).unwrap();
        let mut cow_cred: Vec<(Cow<str>, Cow<str>)> = Vec::new();
        let mut cow_cred_false: Vec<(Cow<str>, Cow<str>)> = Vec::new();
        let not_includes = OAuthTestTool::credentials_not_including(&includes);

        for oac in OAuthCredential::iter() {
            if oauth.contains(oac) && includes.contains(&oac) && !not_includes.contains(&oac) {
                if oac.eq(&OAuthCredential::Scopes) {
                    let s = oauth.get_scopes(" ");
                    cow_cred.push((Cow::from(oac.alias()), Cow::from(s.to_owned())));
                } else if !oac.eq(&OAuthTestTool::match_grant_credential(grant_request)) {
                    let s = oauth.get(oac).unwrap();
                    cow_cred.push((Cow::from(oac.alias()), Cow::from(s.to_owned())));
                }
            } else if oauth.contains(oac) && not_includes.contains(&oac) {
                if oac.eq(&OAuthCredential::Scopes) {
                    let s = oauth.get_scopes(" ");
                    cow_cred.push((Cow::from(oac.alias()), Cow::from(s.to_owned())));
                } else if !oac.eq(&OAuthTestTool::match_grant_credential(grant_request)) {
                    let s = oauth.get(oac).unwrap();
                    cow_cred_false.push((Cow::from(oac.alias()), Cow::from(s.to_owned())));
                }
            }
        }

        let query = parsed_url.query().unwrap();
        let parse = url::form_urlencoded::parse(query.as_bytes());

        for query in parse.into_iter() {
            println!("{:#?}", &query);
            assert_eq!(cow_cred.contains(&query), true);
            assert_eq!(cow_cred_false.contains(&query), false);
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
            assert_eq!(oauth.contains(*oac), true);
        }
    }
}
