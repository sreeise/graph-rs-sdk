use graph_oauth::oauth::AccessToken;
use graph_oauth::oauth::Credential;
use graph_oauth::oauth::OAuth;
use jsonfile::JsonFile;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
struct CleanUp {
    // As in rm -r file for Linux or in other words remove the file.
    rm_f: Vec<String>,
}

impl CleanUp {
    pub fn new<F>(f: F) -> CleanUp
    where
        F: Fn(),
    {
        f();
        CleanUp::default()
    }

    fn rm_files(&mut self, s: String) {
        self.rm_f.push(s);
    }
}

impl Drop for CleanUp {
    fn drop(&mut self) {
        for s in &self.rm_f {
            if Path::new(s.as_str()).exists() {
                fs::remove_file(Path::new(s.as_str())).unwrap();
            }
        }
    }
}

#[test]
fn get_method() {
    let mut access_token = AccessToken::default();
    access_token
        .expires_in(3600)
        .token_type("bearer")
        .access_token("ASODFIUJ34KJ;LADSK")
        .scope("offline")
        .refresh_token(Some("eyJh...9323"));
    assert_eq!(access_token.get_expires_in(), 3600);
    assert_eq!(access_token.get_token_type(), "bearer");
    assert_eq!(access_token.get_access_token(), "ASODFIUJ34KJ;LADSK");
    assert_eq!(access_token.get_scopes(), "offline");
    assert_eq!(access_token.get_refresh_token(), Some("eyJh...9323".into()));
}

#[test]
fn access_token_field_encoding() {
    // Internally this is base64.
    let mut access_token = AccessToken::default();
    access_token.access_token("ASDFJ;34LIUASDOFI NASDOFIUY OP");
    assert_eq!(
        "ASDFJ;34LIUASDOFI NASDOFIUY OP",
        access_token.get_access_token()
    );
}

#[test]
fn oauth_json_file() {
    let file_location = "./test_files/test_file.json";
    let mut clean_up = CleanUp::new(|| {
        if Path::new(file_location).exists() {
            fs::remove_file(Path::new(file_location)).unwrap();
        }
    });

    clean_up.rm_files(file_location.into());

    let mut oauth = OAuth::new();
    oauth
        .client_id("bb301aaa-1201-4259-a230923fds32")
        .redirect_url("http://localhost:8888/redirect")
        .authorize_url("https://example.com/oauth2/v2.0/authorize");

    let mut builder = AccessToken::default();
    builder
        .token_type("token")
        .access_token("access_token")
        .expires_in(3600)
        .scope("scope")
        .refresh_token(None)
        .user_id(None)
        .id_token(None);

    JsonFile::json_file(&file_location, &oauth).unwrap();

    let metadata = fs::metadata(&file_location)
        .expect("Could not get metadata for auth_configs/test_file.json");
    let file_type = metadata.file_type();
    assert_eq!(file_type.is_file(), true);

    let oauth_from_file: OAuth = match JsonFile::from_file(&file_location) {
        Ok(t) => t,
        Err(e) => panic!("Could not get OAuth from file. Error: {:#?}", e),
    };

    assert_eq!(&oauth, &oauth_from_file);
    assert_eq!(
        oauth_from_file.get(Credential::ClientId),
        Some("bb301aaa-1201-4259-a230923fds32".into())
    );
    assert_eq!(
        oauth_from_file.get(Credential::RedirectURI),
        Some("http://localhost:8888/redirect".into())
    );
    assert_eq!(
        oauth_from_file.get(Credential::AuthorizeURL),
        Some("https://example.com/oauth2/v2.0/authorize".into())
    );
}
