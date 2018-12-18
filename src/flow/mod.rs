/*
OAuth2 for Microsoft Authorization for using the Graph API.

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

use crate::process::ReqBuf;
use reqwest::{header, Response};
use url::form_urlencoded;

#[derive(Debug)]
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
            FlowType::AuthorizeTokenFlow => write!(f, "{}", "token"),
            FlowType::AuthorizeCodeFlow => write!(f, "{}", "code"),
            FlowType::GrantTypeAuthCode => write!(f, "{}", "authorization_code"),
            FlowType::GrantTypeRefreshToken => write!(f, "{}", "refresh_token"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AccessToken {
    token_type: String,
    scope: String,
    expires_in: usize,
    ext_expires_in: usize,
    access_token: String,
}

/// Builder for the OAuth2 specification for Microsoft Graph and Authorization
///
/// # Example
///
/// Creating an AuthFlow uses the builder pattern:
/// ```
/// use rust_onedrive::flow::AuthFlow;
///
///
/// let mut auth_flow = AuthFlow::new(true);
///     auth_flow.set_client_id("client_id")
///        .set_auth_url("https://example.com/authorize")
///        .set_client_secret("client_secret")
///        .set_token_url("https://example.com/token");
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthFlow {
    config_name: String,
    scopes: Vec<String>,
    params: HashMap<String, String>,
    allow_reset: bool,
    default_scope: bool,
}

impl AuthFlow {
    pub fn new(default: bool) -> AuthFlow {
        AuthFlow {
            config_name: String::from("AuthFlow"),
            scopes: Vec::new(),
            params: HashMap::new(),
            allow_reset: false,
            default_scope: default,
        }
    }

    /// Set the client id of a request
    ///Set the client id of an OAuth URL.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::flow::AuthFlow;
    ///
    /// let mut graph = AuthFlow::new(true);
    /// graph.set_client_id("client_id");
    /// ```
    pub fn set_client_id(&mut self, client_id: &str) -> &mut AuthFlow {
        self.set_config("CLIENT_ID", client_id)
    }

    /// Set the client secret of an OAuth URL.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::flow::AuthFlow;
    ///
    /// let mut graph = AuthFlow::new(true);
    /// graph.set_client_secret("client_secret");
    /// ```
    pub fn set_client_secret(&mut self, client_secret: &str) -> &mut AuthFlow {
        self.set_config("CLIENT_SECRET", client_secret)
    }

    /// Set the auth url of a request
    /// Set the authorization URL for OAuth.
    ///
    /// # Example
    /// ```
    ///  use rust_onedrive::flow::AuthFlow;
    ///
    /// let mut graph = AuthFlow::new(true);
    /// graph.set_auth_url("https://example.com/authorize");
    /// ```
    pub fn set_auth_url(&mut self, auth_url: &str) -> &mut AuthFlow {
        self.set_config("AUTH_URL", auth_url)
    }

    /// Set the token url of a request
    pub fn set_token_url(&mut self, token_url: &str) -> &mut AuthFlow {
        self.set_config("TOKEN_URL", token_url)
    }

    /// Set the redirect uri of a request
    pub fn set_redirect_uri(&mut self, redirect_uri: &str) -> &mut AuthFlow {
        self.set_config("REDIRECT_URI", redirect_uri)
    }

    /// Set the response type of a request:
    ///     1. Code Flow: 'code'
    ///     2. Token Flow: 'token'
    pub fn set_response_type(&mut self, response_type: &str) -> &mut AuthFlow {
        self.set_config("RESPONSE_TYPE", response_type)
    }

    /// Set the code of a request - returned from log in and authorization
    pub fn set_code(&mut self, code: &str) -> &mut AuthFlow {
        self.set_config("CODE", code)
    }

    /// Set the refresh token of a request
    pub fn set_refresh(&mut self, code: &str) -> &mut AuthFlow {
        self.set_config("REFRESH_TOKEN", code)
    }

    /// Set the token of a request
    pub fn set_token(&mut self, token: &str) -> &mut AuthFlow {
        self.set_config("ACCESS_TOKEN", token)
    }

    /// Add a scope' for the OAuth URL.
    ///
    /// # Example
    /// ```
    /// use rust_onedrive::flow::AuthFlow;
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
    pub fn get_token(&self) -> Option<&String> {
        self.params.get("ACCESS_TOKEN")
    }
    pub fn get_scope(&self) -> Option<&String> {
        self.params.get("REFRESH_TOKEN")
    }
    pub fn get_server(&self) -> Option<&String> {
        self.params.get("SERVER_URL")
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
    /// use rust_onedrive::flow::AuthFlow;
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
                &mut self
                    .params
                    .get("CLIENT_ID")
                    .expect("Couldn't set client_id")
                    .to_string(),
            )
            .append_pair(
                "redirect_uri",
                &mut self
                    .params
                    .get("REDIRECT_URI")
                    .expect("Couldn't set redirect_id")
                    .to_string(),
            )
            .append_pair(
                "client_secret",
                &mut self
                    .params
                    .get("CLIENT_SECRET")
                    .expect("Couldn't set client_secret")
                    .to_string(),
            )
            .append_pair(
                param_type,
                &mut self
                    .params
                    .get(&param_type.to_uppercase())
                    .unwrap()
                    .to_string(),
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
        let auth_url = &mut self.params.get("AUTH_URL").unwrap().to_string();
        auth_url.push_str("?");
        let encoded: String = form_urlencoded::Serializer::new(String::from(""))
            .append_pair(
                "client_id",
                &self.params.get("CLIENT_ID").unwrap().to_string(),
            )
            .append_pair("scope", "https://graph.microsoft.com/.default")
            .append_pair("response_type", flow_type.as_str())
            .append_pair(
                "redirect_uri",
                &self.params.get("REDIRECT_URI").unwrap().to_string(),
            )
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
    pub fn browser_flow(&mut self, addr: &str) -> io::Result<()> {
        let auth_url = self.build(FlowType::AuthorizeCodeFlow).unwrap();
        let auth_to_string = auth_url.as_str();
        // TODO: Need a way to somehow close the browser process as long
        // as it is in in's own window, which currently it is not.
        Command::new("xdg-open").arg(auth_to_string).spawn()?;
        ReqBuf::start(addr);
        Ok(())
    }

    /// Request Access Tokens
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
    pub fn request_access_token(&mut self) -> std::result::Result<Response, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;
        let code_body = self
            .build(FlowType::GrantTypeAuthCode)
            .expect("Could not build with FlowType::CodeBody");
        let access_code = self.params.get("ACCESS_TOKEN").expect(
            "Could not find access token in HashMap. Ensure the value has been set correctly",
        );
        let access_token_url = self
            .params
            .get("TOKEN_URL")
            .expect("Could not find token_url in HashMap. Ensure the value has been set correctly");

        let res = client
            .post(access_token_url)
            .header(header::AUTHORIZATION, access_code.as_str())
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(code_body)
            .send()
            .expect("Error in sending access token post request");
        Ok(res)
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
        file.write_all(serialized.as_bytes());
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
            .set_client_id("fd3019be-1201-4259-a796-b68d0cf5ff1b")
            .set_redirect_uri("http://localhost:8888/redirect")
            .set_response_type("token");
        let mut auth_url = auth_flow.build(FlowType::AuthorizeTokenFlow).unwrap();
        assert_eq!(auth_url, "https://example.com/oauth2/v2.0/authorize?client_id=fd3019be-1201-4259-a796-b68d0cf5ff1b&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=token&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");
    }

    #[test]
    fn code_auth() {
        let mut auth_flow = AuthFlow::new(true);
        auth_flow
            .set_auth_url("https://example.com/oauth2/v2.0/authorize")
            .set_client_id("fd3019be-1201-4259-a796-b68d0cf5ff1b")
            .set_redirect_uri("http://localhost:8888/redirect")
            .set_response_type("code");
        let mut auth_url = auth_flow.build(FlowType::AuthorizeCodeFlow).unwrap();
        assert_eq!(auth_url, "https://example.com/oauth2/v2.0/authorize?client_id=fd3019be-1201-4259-a796-b68d0cf5ff1b&scope=https%3A%2F%2Fgraph.microsoft.com%2F.default&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect");
    }

    #[test]
    fn access_token() {
        let mut code_flow = AuthFlow::new(true);
        code_flow
            .set_client_id("bb301aaa-1201-4259-a796-b68d0cf5ff1b")
            .set_redirect_uri("http://localhost:8888/redirect")
            .set_response_type("token")
            .set_client_secret("CLDIE3F")
            .set_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");

        let code_body = code_flow.build(FlowType::GrantTypeAuthCode).unwrap();
        assert_eq!(code_body, "client_id=bb301aaa-1201-4259-a796-b68d0cf5ff1b&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&code=ALDSKFJLKERLKJALSDKJF2209LAKJGFL&grant_type=authorization_code");
    }

    #[test]
    fn refresh_token() {
        let mut refresh_flow = AuthFlow::new(true);
        refresh_flow
            .set_client_id("d12019be-1201-4259-a796-b68d0cf5ff1b")
            .set_redirect_uri("http://localhost:8888/redirect")
            .set_response_type("token")
            .set_client_secret("CLDIE3F")
            .set_refresh("32LKLASDKJ")
            .set_code("ALDSKFJLKERLKJALSDKJF2209LAKJGFL");
        let refresh_body = refresh_flow.build(FlowType::GrantTypeRefreshToken).unwrap();
        assert_eq!(refresh_body, "client_id=d12019be-1201-4259-a796-b68d0cf5ff1b&redirect_uri=http%3A%2F%2Flocalhost%3A8888%2Fredirect&client_secret=CLDIE3F&refresh_token=32LKLASDKJ&grant_type=refresh_token");
    }
}
