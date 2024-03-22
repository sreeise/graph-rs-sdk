# graph-rs-sdk

![Build](https://github.com/sreeise/graph-rs-sdk/actions/workflows/build.yml/badge.svg)
[![Static Badge](https://img.shields.io/badge/crates.io-2.0.0-blue?style=for-the-badge&link=https%3A%2F%2Fcrates.io%2Fcrates%2Fgraph-rs-sdk)](https://crates.io/crates/graph-rs-sdk)

### Rust SDK Client for Microsoft Graph and Microsoft Identity Platform

### Available on [crates.io](https://crates.io/crates/graph-rs-sdk/1.1.4) - v2.0.0 - Latest Stable Version

#### Features:

Microsoft Graph V1 and Beta API Client
  - Wide support for Graph APIs
  - Paging using Streaming, Channels, or Iterators
  - Upload Sessions, OData Queries, and File Downloads

Microsoft Identity Platform (Getting Access Tokens)
- Auth Code, Client Credentials, Device Code, OpenId
- In Memory Token Cache
- Automatic Token Refresh
- Interactive WebView Auth (feature = `interactive-auth`)
- X509 Certificate (feature = `openssl`) and Proof Key Code Exchange (PKCE) Support


```toml
graph-rs-sdk = "2.0.0"
tokio = { version = "1.25.0", features = ["full"] }
```

For using types that implement serde `Serialize` as request bodies or passing serde's json macro:

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

To use [stream](#streaming) features add the [futures](https://crates.io/crates/futures) crate:

```toml
futures = "0.3"
```

And import `futures::StreamExt`.

```rust
use futures::StreamExt;
use graph_rs_sdk::*;
```

Contributing and Wiki:
- [Contributions](https://github.com/sreeise/graph-rs-sdk/wiki/Contributing)
- [Wiki](https://github.com/sreeise/graph-rs-sdk/wiki)

### Feature requests or Bug reports.

For bug reports please file an issue on GitHub and a response or fix will be given as soon as possible.

The [Discussions](https://github.com/sreeise/graph-rs-sdk/discussions) tab on [GitHub](https://github.com/sreeise/graph-rs-sdk/discussions)
is enabled so feel free to stop by there with any questions or feature requests as well. For bugs, please file
an issue first. Features can be requested through issues or discussions. Either way works.
Other than that feel free to ask questions, provide tips to others, and talk about the project in general.

## Table Of Contents

Graph Client

* [Usage](#usage)
  * [Async and Blocking Client](#async-and-blocking-client)
    * [Async Client](#async-client-default)
    * [Blocking Client](#blocking-client)
  * [Cargo Feature Flags](#cargo-feature-flags)
  * [Paging (Delta, Next Links)](#paging)
    * [Streaming](#streaming)
    * [Channels](#channels)
  * [API Usage](#api-usage)
  * [Id vs Non-Id methods](#id-vs-non-id-methods-such-as-useruser-id-vs-users)
  * [Contributing](#contributing)
  * [Wiki](#wiki)
  * [Feature Requests for Bug Reports](#feature-requests-or-bug-reports)

OAuth and Openid

* [OAuth - Getting Access Tokens](#oauth---getting-access-tokens)
  * [Identity Platform Support](#identity-platform-support)
  * [Credentials](#credentials)
    * [Auth Code Grant](#authorization-code-grant)
      * [Authorization Code Grant Secret](#authorization-code-secret)
      * [Authorization Code With Proof Key Code Exchange](#authorization-code-secret-with-proof-key-code-exchange)
    * [Client Credentials](#client-credentials)
      * [Client Secret Credential](#client-secret-credential)
    * [Environment Credentials](#environment-credentials)
      * [Client Secret Environment Credential](#client-secret-environment-credential)
      * [Resource Owner Password Credential](#resource-owner-password-credential)
  * [Automatic Token Refresh](#automatic-token-refresh)
  * [Interactive Authentication](#interactive-authentication)


[Identity Platform Auth Examples](https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/identity_platform_auth)
- [Auth Code Grant](https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/identity_platform_auth/auth_code_grant)
- [OpenId]((https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/identity_platform_auth/openid))
- [Client Credentials]((https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/identity_platform_auth/client_credentials))
- [Url Builders For Flows Using Sign In To Get Authorization Code - Build Sign In Url](https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0/examples/authorization_sign_in)
- [Interactive Auth Examples (feature = `interactive-auth`)]((https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0/examples/identity_platform_auth))
- [Certificate Auth (feature = `openssl`)](https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0/examples/certificate_auth)

### What APIs are available

The APIs available are generated from OpenApi configs that are stored in Microsoft's msgraph-metadata repository
for the Graph Api. There may be some requests and/or APIs not yet included in this project that are in the OpenApi
config but in general most of them are implemented.

# Usage

For extensive examples see the [examples directory on GitHub](https://github.com/sreeise/graph-rs-sdk/tree/master/examples)

### Async and Blocking Client

The crate can do both an async and blocking requests.

#### Async Client (default)

    graph-rs-sdk = "2.0.0"
    tokio = { version = "1.25.0", features = ["full"] }

#### Example

```rust
use graph_rs_sdk::*;

#[tokio::main]
async fn main() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

  let response = client
      .users()
      .list_user()
      .send()
      .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);
  
  Ok(())
}
```

#### Blocking Client

To use the blocking client use the `into_blocking()` method. You should not
use `tokio` when using the blocking client.

    graph-rs-sdk = "2.0.0"

#### Example
use graph_rs_sdk::*;

```rust
fn main() -> GraphResult<()> {
    let client = Graph::new("ACCESS_TOKEN");

    let response = client
        .users()
        .list_user()
        .into_blocking()    
        .send()?;

    println!("{:#?}", response);

    let body: serde_json::Value = response.json()?;
    println!("{:#?}", body);

    Ok(())
}
```

## Cargo Feature Flags

- `native-tls`: Use the `native-tls` TLS backend (OpenSSL on *nix, SChannel on Windows, Secure Transport on macOS). 
- `rustls-tls`: Use the `rustls-tls` TLS backend (cross-platform backend, only supports TLS 1.2 and 1.3).
- `brotli`: Enables reqwest feature brotli. For more info see the [reqwest](https://crates.io/crates/reqwest) crate.
- `deflate`: Enables reqwest feature deflate. For more info see the [reqwest](https://crates.io/crates/reqwest) crate.
- `trust-dns`: Enables reqwest feature trust-dns. For more info see the [reqwest](https://crates.io/crates/reqwest) crate.
- `test-util`: Enables testing features. Adds the ability to set a custom endpoint for mocking frameworks using the `use_test_endpoint` method on the `GraphClient`. 
  - Also allow http (disable https only)

Default features: `default=["native-tls"]`

#### The send method
The send() method is the main method for sending a request and returns a `Result<rewest::Response, GraphFailure>`. See the
[reqwest](https://crates.io/crates/reqwest) crate for information on the Response type.

```rust
use graph_rs_sdk::*;

pub async fn get_drive_item() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

  let response = client
      .me()
      .drive()
      .get_drive()
      .send()
      .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);
  
  Ok(())
}
```

#### Response Errors/Error Types

While the crate does have its own error type and result you will probably want to use crates like
[anyhow](https://crates.io/crates/anyhow) due to the amount of possible errors that could occur.

If the Microsoft Graph API returned an error this is almost always in the body of the response.
The `ResponseExt` for async requests and `BlockingResponseExt` for blocking requests have convenience
methods that can be used to get the error message from the body of the response.

```rust
use graph_rs_sdk::{http::ResponseExt, Graph};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Graph::new("token");
    let response = client.users().list_user().send().await?;

    if !response.status().is_success() {
        if let Ok(error) = response.into_graph_error_message().await {
            println!("{error:#?}");
        }
    }

    Ok(())
}
```

##### Custom Types
You can pass your own types to API requests that require a request body by implementing `serde::Serialize`.

You can implement your own types by utilizing methods from reqwest::Response. These types must implement `serde::Deserialize`.
See the reqwest crate for more info.

```rust
#[macro_use]
extern crate serde;

use graph_rs_sdk::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DriveItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    // ... Any other fields
}

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static ITEM_ID: &str = "ITEM_ID";

pub async fn get_drive_item() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);
  
  let drive_item = DriveItem {
        id: None,
        name: Some("new name".into())
  };

  let response = client
      .me()
      .drive()
      .item(ITEM_ID)
      .update_items(&drive_item)
      .send()
      .await?;

  println!("{:#?}", response);

  let drive_item: DriveItem = response.json().await?;
  println!("{:#?}", drive_item);

  let response = client
      .me()
      .drive()
      .item(ITEM_ID)
      .get_items()
      .send()
      .await?;

  println!("{:#?}", response);
  
  let drive_item: DriveItem = response.json().await?;
  println!("{:#?}", drive_item);
  
  Ok(())
}
```


## Paging

Paging handles scenarios where the response body is a truncated version of the data and a URL is provided
to continue calling and getting the rest of the data. Paging can consist of multiple links in the call chain.

The sdk provides convenience methods for getting all data in a paging scenario such as using next links or using [delta links to track changes to Graph data](https://learn.microsoft.com/en-us/graph/delta-query-overview).

If you just want a quick and easy way to get all next link responses or the JSON bodies you can use the `paging().json()` method which will exhaust all
next link calls and return all the responses in a `VecDeque<Response<Result<T>>>`. Keep in mind that the larger the volume of next link calls that need to be
made the longer the return delay will be when calling this method.

All paging methods have their response body read in order to get the `@odata.nextLink` URL for calling next link requests. Because of this
the original `reqwest::Response` is lost. However, the paging responses are re-wrapped in a Response object (`http::Response`) that is
similar to `reqwest::Response`. 

See the [Detailed Guide On Paging](https://github.com/sreeise/graph-rs-sdk/wiki/Paging-%E2%80%90-Detailed-Guide) to learn more about using Paging calls in the SDK.

There are different levels of support for paging Microsoft Graph APIs. See the documentation, 
[Paging Microsoft Graph data in your app](https://learn.microsoft.com/en-us/graph/paging), for more info on
supported APIs and availability.

```rust
#[macro_use]
extern crate serde;

use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: Option<String>,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
  pub value: Vec<User>,
}

async fn paging() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);

  let deque = client
      .users()
      .list_user()
      .select(&["id", "userPrincipalName"])
      .paging()
      .json::<Users>()
      .await?;
  
  println!("{:#?}", deque);
  
  Ok(())
}
``` 

The [paging](#paging) example shows a simple way to list users and call all next links. You can also
stream the next link responses or use a channel receiver to get the responses.

### Streaming

Streaming is only available using the async client.

```rust
use futures::StreamExt;
use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

pub async fn stream_next_links() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let mut stream = client
        .users()
        .list_user()
        .select(&["id", "userPrincipalName"])
        .paging()
        .stream::<serde_json::Value>()?;

    while let Some(result) = stream.next().await {
        let response = result?;
        println!("{:#?}", response);

        let body = response.into_body();
        println!("{:#?}", body);
    }

    Ok(())
}

pub async fn stream_delta() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);
  let mut stream = client
          .users()
          .delta()
          .paging()
          .stream::<serde_json::Value>()?;

  while let Some(result) = stream.next().await {
      let response = result?;
      println!("{:#?}", response);
  
      let body = response.into_body();
      println!("{:#?}", body);
  }

  Ok(())
}
```

### Channels

```rust
use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

async fn channel_next_links() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);
  let mut receiver = client
      .users()
      .list_user()
      .paging()
      .channel::<serde_json::Value>()
      .await?;

  while let Some(result) = receiver.recv().await {
    match result {
      Ok(response) => {
        println!("{:#?}", response);

        let body = response.into_body();
        println!("{:#?}", body);
      }
      Err(err) => panic!("{:#?}", err),
    }
  }
  
  Ok(())
}

```

## API Usage

The following shows a few examples of how to use the client and a few of the APIs.

### OneDrive

Make requests to drive using a drive id or through specific drives for me, sites,
users, and groups.

```rust
use graph_rs_sdk::*;

async fn drives() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

  let response = client
      .drives()
      .list_drive()
      .send()
      .await
      .unwrap();

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);

  let response = client
      .drive("DRIVE-ID")
      .item("ITEM_ID")
      .get_items()
      .send()
      .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);
  
  Ok(())
}
```

#### Me API
```rust
async fn drive_me() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

  let response = client
      .me()
      .drive()
      .item("ITEM_ID")
      .get_items()
      .send()
      .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);

  Ok(())
}

```

#### Users API
```RUST
async fn drive_users() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

  let response = client
      .user("USER_ID")
      .drive()
      .item("ITEM_ID")
      .get_items()
      .send()
      .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);

  Ok(())
}
```

#### Sites API
```RUST
async fn drive_users() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

  let response = client
      .site("SITE_ID")
      .drive()
      .item("ITEM_ID")
      .get_items()
      .send()
      .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);

  Ok(())
}
```

Create a folder.

```rust
use graph_rs_sdk::*;
use std::collections::HashMap;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static FOLDER_NAME: &str = "NEW_FOLDER_NAME";
static PARENT_ID: &str = "PARENT_ID";

// For more info on creating a folder see:
// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_post_children?view=odsp-graph-online

pub async fn create_new_folder() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);
  let folder: HashMap<String, serde_json::Value> = HashMap::new();

  let response = client
      .me()
      .drive()
      .item(PARENT_ID)
      .create_children(&serde_json::json!({
          "name": FOLDER_NAME,
          "folder": folder,
          "@microsoft.graph.conflictBehavior": "fail"
      }))
      .send()
      .await?;
  
  println!("{:#?}", response);
  
  Ok(())
}

```

Path based addressing for drive.

```rust
// Pass the path location of the item staring from the OneDrive root folder.
// Start the path with :/ and end with :

async fn get_item_by_path() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

  let response = client
          .me()
          .drive()
          .item_by_path(":/documents/document.docx:")
          .get_items()
          .send()
          .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);

  Ok(())
}
```
    
### Mail

```rust
use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

async fn get_mail_folder() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);

  let response = client.me()
      .mail_folder(MAIL_FOLDER_ID)
      .get_mail_folders()
      .send()
      .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await.unwrap();
  println!("{:#?}", body);

  Ok(())
}
```

#### Create message
```rust
use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static MAIL_FOLDER_ID: &str = "MAIL_FOLDER_ID";

async fn create_message() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);
  
  let response = client
      .me()
      .messages()
      .create_messages(&serde_json::json!({
          "subject":"Did you see last night's game?",
          "importance":"Low",
          "body":{
              "contentType":"HTML",
              "content":"They were <b>awesome</b>!"
          },
          "toRecipients":[
              {
                  "emailAddress":{
                      "address":"miriamg@sreeise.onmicrosoft.com"
                  }
              }
          ]
      }))
      .send()
      .await?;

  println!("{:#?}", response);
  
  Ok(())
}

```

#### Send mail

```rust
use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

async fn send_mail() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .send_mail(&serde_json::json!({
            "message": {
              "subject": "Meet for lunch?",
              "body": {
                  "contentType": "Text",
                  "content": "The new cafeteria is open."
              },
              "toRecipients": [
                  {
                      "emailAddress": {
                      "address": "fannyd@contoso.onmicrosoft.com"
                      }
                  }
              ],
              "ccRecipients": [
                  {
                      "emailAddress": {
                      "address": "danas@contoso.onmicrosoft.com"
                      }
                  }
              ]
            },
          "saveToSentItems": "false"
        }))
        .send()
        .await?;

    println!("{:#?}", response);
    
    Ok(())
}

```

#### Mail folders

```rust
use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static MAIL_FOLDER_ID: &str = "MAIL_FOLDER_ID";

async fn create_mail_folder_message() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
  
    let response = client
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .messages()
        .create_messages(&serde_json::json!({
            "subject":"Did you see last night's game?",
            "importance":"Low",
            "body": {
                "contentType":"HTML",
                "content":"They were <b>awesome</b>!"
            },
            "toRecipients":[{
                    "emailAddress":{
                        "address":"miriamg@sreeise.onmicrosoft.com"
                    }
                }
            ]
        }))
        .send()
        .await?;

    println!("{:#?}", response);
    
    Ok(())
}
```

#### Get Inbox Messages
```rust
use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static USER_ID: &str = "USER_ID";

async fn get_user_inbox_messages() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);
  
  let response = client
      .user(USER_ID)
      .mail_folder("Inbox")
      .messages()
      .list_messages()
      .top("2")
      .send()
      .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);

  Ok(())
}
```
        
Use your own struct. Anything that implements serde::Serialize
can be used for things like creating messages for mail or creating
a folder for OneDrive.

```rust
#[macro_use]
extern crate serde;

use graph_rs_sdk::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Message {
    subject: String,
    importance: String,
    body: HashMap<String, String>,
    #[serde(rename = "toRecipients")]
    to_recipients: Vec<ToRecipient>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ToRecipient {
    #[serde(rename = "emailAddress")]
    email_address: EmailAddress,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct EmailAddress {
    address: String,
}

async fn create_message() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

  let mut body: HashMap<String, String> = HashMap::new();
  body.insert("contentType".to_string(), "HTML".to_string());
  body.insert("content".to_string(), "They were <b>awesome</b>!".to_string());

  let message = Message {
      subject: "Did you see last night's game?".into(),
      importance: "Low".into(),
      body,
      to_recipients: vec![
        ToRecipient {
            email_address: EmailAddress {
              address : "AdeleV@contoso.onmicrosoft.com".into()
            }
        }
      ]
  };

  let response = client
      .me()
      .messages()
      .create_messages(&message)
      .send()
      .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);

  Ok(())
}
```              

### OData Queries

```rust
use graph_rs_sdk::*;

async fn create_message() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

// Get all files in the root of the drive
// and select only specific properties.
  let response = client
          .me()
          .drive()
          .get_drive()
          .select(&["id", "name"])
          .send()
          .await?;

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);

  Ok(())
}
```
   
### Batch Requests

Batch requests use a mpsc::channel and return the receiver
for responses.

```rust
use graph_rs_sdk::*;

static USER_ID: &str = "USER_ID";
static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

async fn batch() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);

  let json = serde_json::json!({
        "requests": [
            {
                "id": "1",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive")
            },
            {
                "id": "2",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive/root")
            },
            {
                "id": "3",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive/recent")
            },
            {
                "id": "4",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive/root/children")
            },
            {
                "id": "5",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive/special/documents")
            },
        ]
    });

  let response = client.batch(&json).send().await?;

  let body: serde_json::Value = response.json().await?;
  println!("{:#?}", body);

  Ok(())
}
```   

## Id vs Non-Id methods (such as `user("user-id")` vs `users()`)

Many of the available APIs have methods that do not require an id for a resource
as well as many of the APIs have methods that do require an id. For most all
of these resources the methods are implemented in this sdk by using two different 
naming schemes and letting the user go directly to the methods they want.

As en example, the users API can list users without an id, and you can find `list_users()`
by calling the `users()` method whereas getting a specific user requires a users id
and you can find `get_users()` by calling `user<ID: AsRef<str>>(id: ID)` method.

### Using the `users()` method:

```rust
use graph_rs_sdk::*;

// For more info on users see: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0
// For more examples see the examples directory on GitHub.

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

async fn list_users() -> GraphResult<()> {
  let client = Graph::new(ACCESS_TOKEN);

  let response = client
      .users()
      .list_user()
      .send()
      .await
      .unwrap();

  println!("{:#?}", response);

  let body: serde_json::Value = response.json().await.unwrap();
  println!("{:#?}", body);
  
  Ok(())
}
```

### Using the user id `user<ID: AsRef<str>>(id: ID)` method:

```rust
use graph_rs_sdk::*;

// For more info on users see: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0
// For more examples see the examples directory on GitHub

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static USER_ID: &str = "USER_ID";

async fn get_user() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .user(USER_ID)
        .get_user()
        .send()
        .await?;

    println!("{:#?}", response);
    
    let body: serde_json::Value = response.json().await?;
    println!("{:#?}", body);
    
    Ok(())
}
```

## OAuth - Getting Access Tokens

Use application builders to store your auth configuration and have the client
handle the access token requests for you.

Support for:

- OpenId, Auth Code Grant, Client Credentials, Device Code, Certificate Auth
- Automatic Token Refresh
- Interactive Authentication | features = [`interactive-auth`]
- Device Code Polling
- Authorization Using Certificates | features = [`openssl`]

#### Detailed Examples:

- [Identity Platform Auth Examples](https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/identity_platform_auth)
  - [Auth Code Grant](https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/identity_platform_auth/auth_code_grant)
  - [OpenId]((https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/identity_platform_auth/openid))
  - [Client Credentials]((https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/identity_platform_auth/client_credentials))
- [Url Builders For Flows Using Sign In To Get Authorization Code - Building Sign In Url](https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/authorization_sign_in)
- [Interactive Auth Examples (feature = `interactive-auth`)]((https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/identity_platform_auth))
- [Certificate Auth (feature = `openssl`)](https://github.com/sreeise/graph-rs-sdk/tree/v2.0.0-beta.0/examples/certificate_auth)

There are two main types for building your chosen OAuth or OpenId Connect Flow.

- `PublicClientApplication`
- `ConfidentialClientApplication`

Once you have built a `ConfidentialClientApplication` or a `PublicClientApplication`
you can pass these to the graph client.

Automatic token refresh is also done by passing the `ConfidentialClientApplication` or the
`PublicClientApplication` to the `Graph` client.

For more extensive examples see the
[OAuth Examples](https://github.com/sreeise/graph-rs-sdk/tree/master/examples/oauth) in the examples/oauth
directory on [GitHub](https://github.com/sreeise/graph-rs-sdk).


```rust
fn build_client(confidential_client: ConfidentialClientApplication<ClientSecretCredential>) {
  let graph_client = GraphClient::from(&confidential_client);
}
```

### Identity Platform Support

The following flows from the Microsoft Identity Platform are supported:

- [Authorization Code Grant](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
- [Authorization Code Grant PKCE](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow)
- [Authorization Code Grant Certificate](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow#request-an-access-token-with-a-certificate-credential)
- [Open ID Connect](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc)
- [Device Code Flow](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code)
- [Client Credentials](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
- [Client Credentials With Certificate](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-client-creds-grant-flow#second-case-access-token-request-with-a-certificate)
- [Resource Owner Password Credentials](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth-ropc)

You can use the url builders for those flows that require an authorization code using a redirect after sign in you can use 

## Credentials


### Authorization Code Grant

The authorization code grant is considered a confidential client (except in the hybrid flow)
and we can get an access token by using the authorization code returned in the query of the URL
on redirect after sign in is performed by the user.

Once you have the authorization code you can pass this to the client and the client
will perform the request to get an access token on the first graph api call that you make.


#### Authorization Code Secret

```rust
use graph_rs_sdk::{
  Graph,
  oauth::ConfidentialClientApplication,
};

async fn build_client(
  authorization_code: &str,
  client_id: &str,
  client_secret: &str,
  redirect_uri: url::Url,
  scope: Vec<&str>
) -> anyhow::Result<GraphClient> {
  let mut confidential_client = ConfidentialClientApplication::builder(client_id)
          .with_authorization_code(authorization_code) // returns builder type for AuthorizationCodeCredential
          .with_client_secret(client_secret)
          .with_scope(scope)
          .with_redirect_uri(redirect_uri)
          .build();

  let graph_client = Graph::from(confidential_client);
  
  Ok(graph_client)
}
```

#### Authorization Code Secret With Proof Key Code Exchange

```rust
use graph_rs_sdk::identity::{
  AuthorizationCodeCredential, ConfidentialClientApplication, GenPkce,
  ProofKeyCodeExchange, TokenCredentialExecutor,
};
use lazy_static::lazy_static;
use url::Url;

// You can also pass your own values for PKCE instead of automatic generation by
// calling ProofKeyCodeExchange::new(code_verifier, code_challenge, code_challenge_method)
lazy_static! {
    static ref PKCE: ProofKeyCodeExchange = ProofKeyCodeExchange::oneshot().unwrap();
}

fn authorization_sign_in_url(client_id: &str, redirect_uri: url::Url, scope: Vec<String>) -> anyhow::Result<Url> {
  Ok(AuthorizationCodeCredential::authorization_url_builder(client_id)
          .with_scope(scope)
          .with_redirect_uri(redirect_uri)
          .with_pkce(&PKCE)
          .url()?)
}

fn build_confidential_client(
  authorization_code: &str,
  client_id: &str,
  client_secret: &str,
  redirect_uri: url::Url,
  scope: Vec<String>,
) -> anyhow::Result<ConfidentialClientApplication<AuthorizationCodeCredential>> {
  Ok(ConfidentialClientApplication::builder(client_id)
          .with_auth_code(authorization_code)
          .with_client_secret(client_secret)
          .with_scope(scope)
          .with_redirect_uri(redirect_uri)
          .with_pkce(&PKCE)
          .build())
}
```

### Client Credentials

The OAuth 2.0 client credentials grant flow permits a web service (confidential client) to use its own credentials,
instead of impersonating a user, to authenticate when calling another web service. The grant specified in RFC 6749, 
sometimes called two-legged OAuth, can be used to access web-hosted resources by using the identity of an application. 
This type is commonly used for server-to-server interactions that must run in the background, without immediate 
interaction with a user, and is often referred to as daemons or service accounts.

Client credentials flow requires a one time administrator acceptance
of the permissions for your apps scopes. To see an example of building the URL to sign in and accept permissions
as an administrator see [Admin Consent Example](https://github.com/sreeise/graph-rs-sdk/tree/master/examples/oauth/client_credentials/client_credentials_admin_consent.rs)


#### Client Secret Credential

```rust
use graph_rs_sdk::{
  oauth::ConfidentialClientApplication, Graph
};

pub async fn get_graph_client(tenant: &str, client_id: &str, client_secret: &str) -> Graph {
  let mut confidential_client_application = ConfidentialClientApplication::builder(client_id)
          .with_client_secret(client_secret)
          .with_tenant(tenant)
          .build();

  Graph::from(&confidential_client_application)
}
```


### Environment Credentials

#### Client Secret Environment Credential

Environment Variables:

- AZURE_TENANT_ID (Optional/Recommended - puts the tenant id in the authorization url)
- AZURE_CLIENT_ID (Required)
- AZURE_CLIENT_SECRET (Required)

```rust
pub fn client_secret_credential() -> anyhow::Result<GraphClient> {
    let confidential_client = EnvironmentCredential::client_secret_credential()?;
    Ok(GraphClient::from(&confidential_client))
}
```

#### Resource Owner Password Credential

Environment Variables:

- AZURE_TENANT_ID (Optional - puts the tenant id in the authorization url)
- AZURE_CLIENT_ID (Required)
- AZURE_USERNAME (Required)
- AZURE_PASSWORD (Required)

```rust
pub fn username_password() -> anyhow::Result<GraphClient> {
    let public_client = EnvironmentCredential::resource_owner_password_credential()?;
    Ok(GraphClient::from(&public_client))
}
```


### Automatic Token Refresh

The client stores tokens using an in memory cache.

Using automatic token refresh requires getting a refresh token as part of the token response.
To get a refresh token you must include the `offline_access` scope.

Automatic token refresh is done by passing the `ConfidentialClientApplication` or the
`PublicClientApplication` to the `Graph` client.

If you are using the `client credentials` grant you do not need the `offline_access` scope.
Tokens will still be automatically refreshed as this flow does not require using a refresh token to get
a new access token.

The example below uses the auth code grant.

First create the url where the user will sign in. After sign in the user will be redirected back to your app and
the authentication code will be in the query of the uri.
```rust
pub fn authorization_sign_in_url(client_id: &str, tenant: &str, redirect_uri: url::Url) -> Url {
  let scope = vec!["offline_access"];

  AuthorizationCodeCredential::authorization_url_builder(client_id)
          .with_redirect_uri(redirect_uri)
          .with_scope(scope)
          .url()
          .unwrap()
}
```

Once you have the authorization code you can build a confidential client and pass it to the graph client.

```rust
async fn build_client(
  authorization_code: &str,
  client_id: &str,
  client_secret: &str,
  scope: Vec<String>, // with offline_access
  redirect_uri: url::Url,
) -> anyhow::Result<GraphClient> {
  let mut confidential_client = ConfidentialClientApplication::builder(client_id)
          .with_auth_code(authorization_code) // returns builder type for AuthorizationCodeCredential
          .with_client_secret(client_secret)
          .with_scope(scope)
          .with_redirect_uri(redirect_uri)
          .build();

  let graph_client = GraphClient::from(&confidential_client);

  Ok(graph_client)
}
```


### Interactive Authentication

Requires Feature `interactive-auth`

```toml
[dependencies]
graph-rs-sdk = { version = "...", features = ["interactive-auth"] }
```

Interactive Authentication uses the [wry](https://github.com/tauri-apps/wry) crate to run web view on 
platforms that support it such as on a desktop.

```rust
use graph_rs_sdk::{identity::{AuthorizationCodeCredential, Secret}, GraphClient};

async fn authenticate(
  tenant_id: &str,
  client_id: &str,
  client_secret: &str,
  redirect_uri: url::Url,
  scope: Vec<&str>,
) -> anyhow::Result<GraphClient> {
  std::env::set_var("RUST_LOG", "debug");
  pretty_env_logger::init();

  let (authorization_query_response, mut credential_builder) =
          AuthorizationCodeCredential::authorization_url_builder(client_id)
                  .with_tenant(tenant_id)
                  .with_scope(scope) // Adds offline_access as a scope which is needed to get a refresh token.
                  .with_redirect_uri(redirect_uri)
                  .with_interactive_auth(Secret("client-secret".to_string()), Default::default())
                  .unwrap();

  debug!("{authorization_query_response:#?}");

  let mut confidential_client = credential_builder.with_client_secret(client_secret).build();

  Ok(GraphClient::from(&confidential_client))
}
```


## Contributing

See the [Contributions](https://github.com/sreeise/graph-rs-sdk/wiki/Contributing) guide on GitHub


## Wiki:

See the [GitHub Wiki](https://github.com/sreeise/graph-rs-sdk/wiki)


## Feature requests or Bug reports

For bug reports please file an issue on GitHub and a response or fix will be given as soon as possible.

The [Discussions](https://github.com/sreeise/graph-rs-sdk/discussions) tab on [GitHub](https://github.com/sreeise/graph-rs-sdk/discussions)
is enabled so feel free to stop by there with any questions or feature requests as well. For bugs, please file
an issue first. Features can be requested through issues or discussions. Either way works.
Other than that feel free to ask questions, provide tips to others, and talk about the project in general.
