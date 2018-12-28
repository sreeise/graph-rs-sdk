/*
OAuth2 for Microsoft Authorization for using the Graph API V1.

    client_id:
        Type:  string
        Description:  The client ID value created for your application.
    redirect_uri
        Type: string
        Description: The redirect URL that the browser is sent to when authentication is complete.
    response_type
        Type: string
        Description: The type of response expected from the authorization flow.
        Flow Type:
            1. Code: Value must be 'code'
            2. Token: Value must be 'token'
    scope
        Type: string
        Description: A space-separated list of scopes your application requires.
    refresh_token
        Type: string
        Description: The refresh token you received previously. If the flow is a
                    code flow and offline access is requested, then the response
                    from the access token (2nd step) request will have a refresh code.

TOKEN FLOW
    GET https://login.microsoftonline.com/common/oauth2/v2.0/authorize?
    client_id={client_id}
    &scope={scope}
    &response_type=code
    &redirect_uri={redirect_uri}

CODE FLOW
    Step 1:
        GET https://login.microsoftonline.com/common/oauth2/v2.0/authorize?
        client_id={client_id}
        &scope={scope}
        &response_type=token
        &redirect_uri={redirect_uri}

    Step 2:
        POST https://login.live.com/oauth20_token.srf
        Content-Type: application/x-www-form-urlencoded

        client_id={client_id}&redirect_uri={redirect_uri}&client_secret={client_secret}
        &code={code}&grant_type=authorization_code
*/

use core::fmt;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::path::Path;
use std::process::Command;
use std::result;

use crate::flow::accesstoken::AccessToken;
use reqwest::{header, Response};
use serde_json::{Deserializer, Error, Value};
use std::thread;
use std::time::Duration;
use url::form_urlencoded;

#[derive(Debug, Copy, Clone)]
pub enum FlowType {
    AuthorizeTokenFlow,
    AuthorizeCodeFlow,
    GrantTypeAuthCode,
    GrantTypeRefreshToken,
}

impl FlowType {
    fn as_str(&self) -> &'static str {
        match *self {
            FlowType::AuthorizeTokenFlow => "token",
            FlowType::AuthorizeCodeFlow => "code",
            FlowType::GrantTypeRefreshToken => "refresh_token",
            FlowType::GrantTypeAuthCode => "authorization_code",
        }
    }
}

impl fmt::Display for FlowType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FlowType::AuthorizeTokenFlow => write!(f, "{:#?}", "token"),
            FlowType::AuthorizeCodeFlow => write!(f, "{:#?}", "code"),
            FlowType::GrantTypeAuthCode => write!(f, "{:#?}", "authorization_code"),
            FlowType::GrantTypeRefreshToken => write!(f, "{:#?}", "refresh_token"),
        }
    }
}

/// Builder for the OAuth2 specification for Microsoft Graph and Authorization
///
/// # Example
///
/// Creating an AuthFlow uses the builder pattern:
/// ```
/// use rust_onedrive::flow::v1::AuthFlow;
///
///
/// let mut auth_flow = AuthFlow::new(true);
///     auth_flow.set_client_id("client_id")
///        .set_auth_url("https://example.com/authorize")
///        .set_client_secret("client_secret")
///        .set_token_url("https://example.com/token");
/// ```
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthFlow {
    config_name: String,
    scopes: Vec<String>,
    params: HashMap<String, String>,
    allow_reset: bool,
    default_scope: bool,
    access_token: Option<AccessToken>,
}

impl fmt::Display for AuthFlow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#?}\n{:#?}\n{:#?}\n{:#?}\n{:#?}",
            self.config_name, self.scopes, self.params, self.allow_reset, self.default_scope
        )
    }
}

impl AuthFlow {
    pub fn new(default: bool) -> AuthFlow {
        AuthFlow {
            config_name: String::from("AuthFlow"),
            scopes: Vec::new(),
            params: HashMap::new(),
            allow_reset: false,
            default_scope: default,
            access_token: None,
        }
    }

    /// Set the client id of a request
    ///Set the client id of an OAuth URL.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::flow::v1::AuthFlow;
    ///
    /// let mut auth_flow = AuthFlow::new(true);
    /// auth_flow.set_client_id("client_id");
    /// ```
    pub fn set_client_id(&mut self, client_id: &str) -> &mut AuthFlow {
        self.set_config("CLIENT_ID", client_id)
    }

    /// Set the client secret of an OAuth URL.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::flow::v1::AuthFlow;
    ///
    /// let mut auth_flow = AuthFlow::new(true);
    /// auth_flow.set_client_secret("client_secret");
    /// ```
    pub fn set_client_secret(&mut self, client_secret: &str) -> &mut AuthFlow {
        self.set_config("CLIENT_SECRET", client_secret)
    }

    /// Set the auth url of a request
    /// Set the authorization URL for OAuth.
    ///
    /// # Example
    /// ```
    ///  use rust_onedrive::flow::v1::AuthFlow;
    ///
    /// let mut auth_flow = AuthFlow::new(true);
    /// auth_flow.set_auth_url("https://example.com/authorize");
    /// ```
    pub fn set_auth_url(&mut self, auth_url: &str) -> &mut AuthFlow {
        self.set_config("AUTH_URL", auth_url)
    }

    /// Set the token url of a request for OAuth
    ///
    /// # Example
    /// ```
    ///  use rust_onedrive::flow::v1::AuthFlow;
    ///
    /// let mut auth_flow = AuthFlow::new(true);
    /// auth_flow.set_token_url("https://example.com/token");
    /// ```
    pub fn set_token_url(&mut self, token_url: &str) -> &mut AuthFlow {
        self.set_config("TOKEN_URL", token_url)
    }

    /// Set the redirect uri of a request
    ///
    /// # Example
    /// ```
    ///  use rust_onedrive::flow::v1::AuthFlow;
    ///
    /// let mut auth_flow = AuthFlow::new(true);
    /// auth_flow.set_redirect_uri("https://localhost:8888/redirect");
    /// ```
    pub fn set_redirect_uri(&mut self, redirect_uri: &str) -> &mut AuthFlow {
        self.set_config("REDIRECT_URI", redirect_uri)
    }

    /// Set the response type of a request:
    ///     1. Code Flow: 'code'
    ///     2. Token Flow: 'token'
    ///
    /// # Example
    /// ```
    ///  use rust_onedrive::flow::v1::AuthFlow;
    ///
    /// let mut auth_flow = AuthFlow::new(true);
    /// auth_flow.set_response_type("code");
    /// ```
    pub fn set_response_type(&mut self, response_type: &str) -> &mut AuthFlow {
        self.set_config("RESPONSE_TYPE", response_type)
    }

    /// Set the code of a request - returned from log in and authorization
    pub fn set_access_code(&mut self, code: &str) -> &mut AuthFlow {
        self.set_config("CODE", code);
        self
    }

    /// Set the refresh token of a request
    pub fn set_refresh(&mut self, code: &str) -> &mut AuthFlow {
        self.set_config("REFRESH_TOKEN", code)
    }

    /// Set the token of a request
    pub fn set_access_token(&mut self, token: &str) -> &mut AuthFlow {
        self.set_config("ACCESS_TOKEN", token)
    }

    pub fn set_access_token_struct(&mut self, access_token: AccessToken) {
        self.access_token = Some(access_token);
    }

    /// Add a scope' for the OAuth URL.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::flow::v1::AuthFlow;
    ///
    /// let mut auth_flow = AuthFlow::new(false);
    /// auth_flow.add_scope("Read");
    /// // or
    /// auth_flow.add_scope("Read")
    ///     .add_scope("Write")
    ///     .add_scope("ReadWrite.All");
    /// ```
    pub fn add_scope(&mut self, scope: &str) -> &mut AuthFlow {
        self.scopes.push(scope.to_string());
        self
    }

    pub fn set_response_mode(&mut self, response_mode: &str) -> &mut AuthFlow {
        self.set_config("RESPONSE_MODE", response_mode)
    }

    /// Set the state of a request
    pub fn set_state(&mut self, state: &str) -> &mut AuthFlow {
        self.set_config("STATE", state)
    }

    /// Set url to use for sending authentication info of a request
    pub fn with_listener(&mut self, url: &str) -> &mut AuthFlow {
        self.set_config("SERVER_URL", url)
    }

    pub fn get_client_id(&self) -> Option<&String> {
        self.params.get("CLIENT_ID")
    }
    pub fn get_client_secret(&self) -> Option<&String> {
        self.params.get("CLIENT_SECRET")
    }
    pub fn get_auth_url(&self) -> Option<&String> {
        self.params.get("AUTH_URL")
    }
    pub fn get_token_url(&self) -> Option<&String> {
        self.params.get("TOKEN_URL")
    }
    pub fn get_redirect_uri(&self) -> Option<&String> {
        self.params.get("REDIRECT_URI")
    }
    pub fn get_access_code(&self) -> Option<&String> {
        self.params.get("CODE")
    }
    pub fn get_access_token(&self) -> Option<&String> {
        self.params.get("ACCESS_TOKEN")
    }
    pub fn get_refresh_token(&self) -> Option<&String> {
        self.params.get("REFRESH_TOKEN")
    }
    pub fn get_scopes(&self) -> Option<&Vec<String>> {
        Some(&self.scopes)
    }
    pub fn get_server(&self) -> Option<&String> {
        self.params.get("SERVER_URL")
    }
    pub fn get_access_token_struct(self) -> Option<AccessToken> {
        self.access_token
    }

    fn set_config(&mut self, config_key: &str, config_value: &str) -> &mut AuthFlow {
        if !self.params.contains_key(config_key) || self.allow_reset {
            self.params
                .insert(config_key.to_string(), config_value.to_string());
        } else {
            AuthFlow::print_reset_error(config_key);
        }
        self
    }

    /// By default, once a value has a value has been set (token, auth_url, etc.)
    /// the value cannot be changed unless the caller explicitly says to allow
    /// change.
    ///
    /// Give the bool true as the argument to allow changing values. This is
    /// permanent until the caller explicitly sets allow_change to false.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::flow::v1::AuthFlow;
    ///
    /// let mut auth_flow = AuthFlow::new(true);
    /// auth_flow.set_client_id("client_id");
    /// // attempting to change client id would not work here.
    ///
    /// // Allow changing the authorization flow parameters before or after
    /// // a request
    /// auth_flow.allow_reset(true);
    ///
    /// // Client id can be changed now.
    /// auth_flow.set_client_id("new_client_id");
    ///
    /// // Set the allow reset back to false for safety
    /// auth_flow.allow_reset(false);
    ///  ```
    pub fn allow_reset(&mut self, allow_reset: bool) {
        self.allow_reset = allow_reset;
    }

    fn print_reset_error(config_to_reset: &str) {
        println!(
            "\nERROR:\n{} has already been set, call allow_reset(true) to change {}\n",
            config_to_reset, config_to_reset
        );
    }

    // TODO: Encoding scopes is not implemented yet
    // Join the scopes when manually setting them versus
    // using the default url: https://graph.microsoft.com/.default
    pub fn join_scopes(&mut self) -> String {
        self.scopes.join(" & ")
    }

    /// Token flow and code flow are the same except for the response type.
    /// Token flow uses 'token' and code flow uses 'code'
    ///
    /// All flows must go through a browser or browser control.
    ///
    /// Token Flow
    ///     GET https://login.microsoftonline.com/common/oauth2/v2.0/authorize?
    ///     client_id={client_id}
    ///     &scope={scope}
    ///     &response_type=code
    ///     &redirect_uri={redirect_uri}
    ///
    /// Code Flow
    ///     GET https://login.microsoftonline.com/common/oauth2/v2.0/authorize?
    ///     client_id={client_id}
    ///     &scope={scope}
    ///     &response_type=token
    ///     &redirect_uri={redirect_uri}
    pub fn build(&mut self, to_build: FlowType) -> Option<String> {
        match to_build {
            FlowType::AuthorizeTokenFlow => Some(self.build_auth(to_build)),
            FlowType::AuthorizeCodeFlow => Some(self.build_auth(to_build)),
            FlowType::GrantTypeAuthCode => Some(
                self.build_grant_request(to_build)
                    .expect("Could not build access token body"),
            ),
            FlowType::GrantTypeRefreshToken => Some(
                self.build_grant_request(to_build)
                    .expect("Could not build refresh token body"),
            ),
        }
    }

    /// Access Tokens and Refresh Access Tokens
    ///
    /// Access Tokens
    ///
    /// POST https://login.microsoftonline.com/common/oauth2/v2.0/token
    /// Content-Type: application/x-www-form-urlencoded
    ///
    /// Body of request:
    ///     client_id={client_id}
    ///     &redirect_uri={redirect_uri}
    ///     &client_secret={client_secret}
    ///     &code={code}
    ///     &grant_type=authorization_code
    ///
    /// Refresh Access Tokens
    ///
    /// POST https://login.microsoftonline.com/common/oauth2/v2.0/token
    /// Content-Type: application/x-www-form-urlencoded
    ///
    /// Body of Request:
    ///     client_id={client_id}
    ///     &redirect_uri={redirect_uri}8
    ///     &client_secret={client_secret}
    ///     &refresh_token={refresh_token}
    ///     &grant_type=refresh_token
    fn build_grant_request(&mut self, grant_type: FlowType) -> result::Result<String, io::Error> {
        let req_type = match grant_type {
            FlowType::GrantTypeAuthCode => FlowType::GrantTypeAuthCode.as_str(),
            FlowType::GrantTypeRefreshToken => FlowType::GrantTypeRefreshToken.as_str(),
            FlowType::AuthorizeTokenFlow => panic!("Not a grant type"),
            FlowType::AuthorizeCodeFlow => panic!("Not a grant type"),
        };

        let param_type = match grant_type {
            FlowType::GrantTypeAuthCode => "code",
            FlowType::GrantTypeRefreshToken => "refresh_token",
            FlowType::AuthorizeTokenFlow => panic!("Not a grant type"),
            FlowType::AuthorizeCodeFlow => panic!("Not a grant type"),
        };

        let encoded: String = form_urlencoded::Serializer::new(String::from(""))
            .append_pair(
                "client_id",
                self.params
                    .get("CLIENT_ID")
                    .expect("Couldn't set client_id")
                    .as_str(),
            )
            .append_pair(
                "redirect_uri",
                self.params
                    .get("REDIRECT_URI")
                    .expect("Couldn't set redirect_id")
                    .as_str(),
            )
            .append_pair(
                "client_secret",
                self.params
                    .get("CLIENT_SECRET")
                    .expect("Couldn't set client_secret")
                    .as_str(),
            )
            .append_pair(
                param_type,
                self.params
                    .get(&param_type.to_uppercase())
                    .unwrap()
                    .as_str(),
            )
            .append_pair("grant_type", req_type)
            .finish();
        Ok(encoded.to_string())
    }

    /// The first prat of the request is determined by the user of the
    /// FlowType enum.
    ///
    ///
    /// TOKEN FLOW = FlowType::
    ///     GET https://login.microsoftonline.com/common/oauth2/v2.0/authorize?
    ///     client_id={client_id}
    ///     &scope={scope}
    ///     &response_type=code
    ///     &redirect_uri={redirect_uri}
    ///
    /// CODE FLOW
    ///     GET https://login.microsoftonline.com/common/oauth2/v2.0/authorize?
    ///     client_id={client_id}
    ///     &scope={scope}
    ///     &response_type=token
    ///     &redirect_uri={redirect_uri}
    pub fn build_auth(&mut self, flow_type: FlowType) -> String {
        let auth_url = &mut self.params["AUTH_URL"].to_string();
        auth_url.push_str("?");
        let encoded: String = form_urlencoded::Serializer::new(String::from(""))
            .append_pair("client_id", &self.params["CLIENT_ID"].to_string())
            .append_pair("scope", "https://graph.microsoft.com/.default")
            .append_pair("response_type", flow_type.as_str())
            .append_pair("redirect_uri", &self.params["REDIRECT_URI"].to_string())
            .finish();
        auth_url.push_str(&encoded);
        auth_url.to_string()
    }

    /// Open the browser to the authentication page. Once the user signs in the
    /// page will redirect to the url that was specified for redirect_url.
    ///
    /// Linux: There are a couple of options to open the browser with a url.
    ///     1. Use xdg-open to open the browser
    ///     2. Use Command to call the firefox process directly:
    ///         .arg("firefox")
    ///         .arg("--new-window");
    pub fn browser_flow(&mut self) -> io::Result<()> {
        let auth_url = self.build(FlowType::AuthorizeCodeFlow).unwrap();
        let handle = thread::spawn(move || {
            let auth_to_string = auth_url.as_str();
            Command::new("xdg-open")
                .arg(auth_to_string)
                .spawn()
                .expect("Could not open browser");
            thread::sleep(Duration::from_millis(3))
        });

        // Make sure threads spawn and finish
        handle.join().unwrap();
        Ok(())
    }

    /// Request Access Tokens
    ///
    /// Builds the url and performs post request for access token.
    /// If successful, the access token String and the struct AccessToken
    /// will automatically be set (see example). This method requires
    /// the token_url and access_code to be set and valid for performing
    /// the post request.
    ///
    /// Don't confuse an access_code for an access_token. The access_code is
    /// used to perform the post request for an access_token. The access_code
    /// is retrieved from the url on a browser redirect when logging in to
    /// a Microsoft account.
    ///
    /// An access token request has the following requirements:
    ///
    /// POST https://login.live.com/oauth20_token.srf
    /// Content-Type: application/x-www-form-urlencoded
    ///
    /// Body of Request:
    ///     client_id={client_id}
    ///     &redirect_uri={redirect_uri}
    ///     &client_secret={client_secret}
    ///     &refresh_token={refresh_token}
    ///     &grant_type=refresh_token
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut auth_flow = AuthFlow::new(true);
    /// auth_flow
    ///     .set_auth_url("https://login.live.com/oauth20_authorize.srf")
    ///     .set_client_id("<client id>")
    ///     .set_client_secret("<client secret>")
    ///     .set_redirect_uri("http://localhost:8000/redirect")
    ///     .set_token_url("https://login.live.com/oauth20_token.srf");
    ///
    /// // Send user to url for for access code
    /// // then set the access code for an AuthFlow struct
    /// auth_flow.set_access_token("<access code>");
    ///
    /// // Run request_access_token() which will automatically set the
    /// // AccessToken struct in AuthFlow.access_token
    /// auth_flow.request_access_token()?;
    ///
    /// // Get the AccessToken struct
    /// let access_token: AccessToken = auth_flow.get_access_token_struct()?;
    /// // Get the access token String from AccessToken
    /// println!("{:#?}", access_token.get_access_token());
    ///
    /// // or
    ///
    /// // Get only the access token string which is also set in AuthFlow
    /// // when calling request_access_token()
    /// println!("{:#?}", auth_flow.get_access_token());
    /// ```
    pub fn request_access_token(&mut self) -> io::Result<()> {
        let client = reqwest::Client::builder()
            .build()
            .expect("could not construct reqwest builder");
        let code_body = self
            .build(FlowType::GrantTypeAuthCode)
            .expect("Could not build with FlowType::GrantTypeAuthCode");
        let access_code = self.params.get("CODE").expect(
            "Could not find access token in HashMap. Ensure the value has been set correctly",
        );
        let access_token_url = self
            .params
            .get("TOKEN_URL")
            .expect("Could not find token_url in HashMap. Ensure the value has been set correctly");

        let mut res = client
            .post(access_token_url)
            .header(header::AUTHORIZATION, access_code.as_str())
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(code_body)
            .send()
            .expect("Error in sending access token post request");

        let mut json_str = res
            .text()
            .expect("could not get request body: bad request or invalid access_code");
        let mut data = json::parse(&json_str.as_str())
            .expect("could not get request body: bad request or invalid access_code");
        let access_token_str = data["access_token"]
            .as_str()
            .expect("could not get request body: bad request or invalid access_code");

        self.set_access_token(&data["access_token"].as_str().unwrap());
        self.access_token = Some(AccessToken::new(
            data["token_type"]
                .as_str()
                .expect("could not convert token_type to str"),
            data["expires_in"]
                .as_u64()
                .expect("could not convert expires_in to u64"),
            data["scope"].as_str()
                .expect("could not convert scope to str"),
            data["access_token"]
                .as_str()
                .expect("could not convert access_token to str"),
            data["user_id"]
                .as_str()
                .expect("could not convert user_id to str"),
        ));

        Ok(())
    }

    /// Writes the AuthFlow struct as a JSON file using serde_json.
    /// An AuthFlow struct can then be brought back in.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a file and the file name itself.
    pub fn as_json_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let mut file = File::create(path)?;
        let serialized = serde_json::to_string(self)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    /// Writes the AuthFlow struct as a JSON file using serde_json only if the path/file
    /// is not already created.
    /// # Arguments
    ///
    /// * `path` - Path to a file and the file name itself.
    pub fn as_new_json_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), io::Error> {
        File::open(&path)
            .map_err(|error| {
                if error.kind() == ErrorKind::NotFound {
                    File::create(&path).unwrap_or_else(|error| {
                        panic!(
                            "The file was originally not found but an error occurred: {:?}",
                            error
                        );
                    })
                } else {
                    panic!("\nError in file creation, error: \n{:?}\n", error);
                }
            })
            .expect("Could not write to file");

        let serialized = serde_json::to_string(self).expect("Error serializing struct");
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("Error writing file with serialized struct");
        file.write_all(serialized.as_bytes())
            .expect("Could not write AuthFlow as a new json file");
        Ok(())
    }

    /// Get a Graph from a previously saved Graph as JSON
    ///
    /// # Arguments
    ///
    /// * `path` - Path to a file and the file name itself.
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<AuthFlow> {
        let f = File::open(path)?;
        let graph = serde_json::from_reader(f)?;
        Ok(graph)
    }

    pub fn from_file_as_vec<P: AsRef<Path>>(path: P) -> io::Result<Vec<AuthFlow>> {
        let f = File::open(path)?;
        let buffer = BufReader::new(f);
        let mut graph_vec: Vec<AuthFlow> = Vec::new();

        for file_name in buffer.lines() {
            let graph = AuthFlow::from_file(&file_name?)?;
            graph_vec.push(graph);
        }

        Ok(graph_vec)
    }
}

#[cfg(test)]
mod flow_tests {
    use super::*;
    use std::fs;

    #[test]
    fn create() {
        let mut auth_flow = AuthFlow::new(true);
        auth_flow
            .set_client_id("graph_client_id")
            .set_client_secret("A_client_secret")
            .set_auth_url("https://example.com/authorize")
            .set_token_url("https://example.com/token");

        let mut result = auth_flow.get_client_id();
        assert_eq!(result.is_none(), false);
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), "graph_client_id");
        result = auth_flow.get_client_secret();
        assert_eq!(result.is_none(), false);
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), "A_client_secret");
        result = auth_flow.get_auth_url();
        assert_eq!(result.is_none(), false);
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), "https://example.com/authorize");
        result = auth_flow.get_token_url();
        assert_eq!(result.is_none(), false);
        assert_eq!(result.is_some(), true);
        assert_eq!(result.unwrap(), "https://example.com/token");
    }

    #[test]
    fn reset_config() {
        let mut auth_flow = AuthFlow::new(true);
        auth_flow.set_client_id("graph_client_id");
        assert_eq!(auth_flow.get_client_id().unwrap(), "graph_client_id");
        auth_flow.allow_reset(true);
        auth_flow.set_client_id("diff_client_id");
        assert_eq!(auth_flow.get_client_id().unwrap(), "diff_client_id");
    }

    #[test]
    fn token_auth() {
        let mut auth_flow = AuthFlow::new(true);
        auth_flow
            .set_auth_url("https://example.com/oauth2/v2.0/authorize")
            .set_client_id("bb301aaa-1201-4259-a230923fds32")
            .set_redirect_uri("http://localhost:8888/redirect")
            .set_response_type("token");
        let auth_url = auth_flow.build(FlowType::AuthorizeTokenFlow).unwrap();
        assert_eq!(auth_url, "https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=token&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");
    }

    #[test]
    fn code_auth() {
        let mut auth_flow = AuthFlow::new(true);
        auth_flow
            .set_auth_url("https://example.com/oauth2/v2.0/authorize")
            .set_client_id("bb301aaa-1201-4259-a230923fds32")
            .set_redirect_uri("http://localhost:8888/redirect")
            .set_response_type("code");
        let auth_url = auth_flow.build(FlowType::AuthorizeCodeFlow).unwrap();
        assert_eq!(auth_url, "https://example.com/oauth2/v2.0/authorize?client_id=bb301aaa-1201-4259-a230923fds32&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");
    }

    #[test]
    fn access_token() {
        let mut code_flow = AuthFlow::new(true);
        code_flow
            .set_client_id("bb301aaa-1201-4259-a230923fds32")
            .set_redirect_uri("http://localhost:8888/redirect")
            .set_response_type("token")
            .set_client_secret("CLDIE3F")
            .set_token_url("https://www.example.com/token")
            .set_access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

        let code_body = code_flow.build(FlowType::GrantTypeAuthCode).unwrap();
        assert_eq!(code_body, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL&grant_type=authorization_code");
    }

    #[test]
    fn refresh_token() {
        let mut refresh_flow = AuthFlow::new(true);
        refresh_flow
            .set_client_id("bb301aaa-1201-4259-a230923fds32")
            .set_redirect_uri("http://localhost:8888/redirect")
            .set_response_type("token")
            .set_client_secret("CLDIE3F")
            .set_refresh("32LKLASDKJ")
            .set_token_url("https://www.example.com/token")
            .set_access_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");
        let refresh_body = refresh_flow.build(FlowType::GrantTypeRefreshToken).unwrap();
        assert_eq!(refresh_body, "client_id=bb301aaa-1201-4259-a230923fds32&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&refresh_token=32LKLASDKJ&grant_type=refresh_token");
    }

    #[test]
    fn flow_as_json_file() {
        let mut auth_flow = AuthFlow::new(true);
        auth_flow
            .set_client_id("bb301aaa-1201-4259-a230923fds32")
            .set_redirect_uri("http://localhost:8888/redirect")
            .set_auth_url("https://example.com/oauth2/v2.0/authorize");
        auth_flow
            .as_json_file("graph_configs/test_file.json")
            .unwrap();

        let metadata = fs::metadata("graph_configs/test_file.json")
            .expect("Could not get metadata for auth_configs/test_file.json");
        let file_type = metadata.file_type();
        assert_eq!(file_type.is_file(), true);

        let auth_flow_from_file: AuthFlow = AuthFlow::from_file("graph_configs/test_file.json")
            .expect("Could not create AuthFlow from graph_configs/test_file.json");
        assert_eq!(
            auth_flow_from_file.get_client_id().unwrap(),
            "bb301aaa-1201-4259-a230923fds32"
        );
        assert_eq!(
            auth_flow_from_file.get_redirect_uri().unwrap(),
            "http://localhost:8888/redirect"
        );
        assert_eq!(
            auth_flow_from_file.get_auth_url().unwrap(),
            "https://example.com/oauth2/v2.0/authorize"
        );

        fs::remove_file("graph_configs/test_file.json").expect(
            "graph_configs/test_file.json could not be removed. It must be removed manually.",
        );
    }
}
