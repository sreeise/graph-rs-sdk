# Rust-OneDrive

[![Build Status](https://travis-ci.com/sreeise/rust-onedrive.svg?branch=master)](https://travis-ci.com/sreeise/rust-onedrive)

### OneDrive API - Built with Rust

Still in very early development. OAuth2 authentication is the main priority.
The current implementation is fully built to handle web or native clients for
personal and business accounts. However, the api is still a work in progress
and subject to change.

Using the browser_flow() method for opening the browser and sending a user to 
log in (shown below) currently only works on linux.

Requires Rust nightly:

    rustup install nightly
    cd rust-onedrive
    rustup override set nightly

### Tests and Docs

Of the portions that are implemented there are also examples and docs. Run: 

    cargo doc

Tests are run on travis ci and can be run locally through cargo like normal Rust builds:

    cargo test

### Use

The basic setup for a native client is below. See the examples directory for more information.

#### Setup
    
    // AuthFlow can be either a web client or native client. There is also
    // a generic new() method that requires a more manual setup.
    let mut auth_flow = native_client();
    
    // Set the client id for native clients and redirect url for users logging in. 
    // Web clients require a client id and client secret, native clients do not.
    auth_flow.set_client_id("<CLIENT ID>")
             .set_redirect_uri("https://login.microsoftonline.com/common/oauth2/nativeclient");
    
    // Set the scopes.
    // wl.offline_access will cause the request to return
    // a refresh token as well.
    auth_flow
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("wl.offline_access"); 
    
    // The urls used to request access and refresh tokens
    // can be set manually or automatically. These are automatically
    // set when using AccountType and can either be Account or Graph.    
    auth_flow.use_default_auth_url(AccountType::Account);
    
        
#### User Sign In.

    // Opens up the default browser to the microsoft log in page.
    // Once a user has logged in, the page will redirect. The redirect
    // url ends with a short-lived code to call the api for an access token.
    auth_flow.browser_flow().unwrap();
    
#### Getting access Tokens

    // Set the access code that will be used to request an
    // access token.
    auth_flow.set_access_code(access_code); 
    
    // Request the access token.
    auth_flow.request_access_token();
       
    if auth_flow.req_error.is_some() {
        println!("{:#?}", auth_flow.req_error);
    } else {
        println!("{:#?}", &auth_flow);
    }

