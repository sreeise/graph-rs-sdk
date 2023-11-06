# graph-rs-sdk

[![crates.io](https://img.shields.io/crates/v/graph-rs-sdk.svg)](https://crates.io/crates/graph-rs-sdk)
![Build](https://github.com/sreeise/graph-rs-sdk/actions/workflows/build.yml/badge.svg)
[![Build status](https://ci.appveyor.com/api/projects/status/llvpt7xiy53dmo7a/branch/master?svg=true)](https://ci.appveyor.com/project/sreeise/rust-onedrive)

### Rust SDK Client for Microsoft Graph and the Microsoft Graph Api

### Available on [crates.io](https://crates.io/crates/graph-rs-sdk)

```toml
graph-rs-sdk = "1.1.1"
tokio = { version = "1.25.0", features = ["full"] }
```

For using types that implement serde `Serialize` as request bodies or passing serde's json macro:

```toml
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

To use stream features add futures crate:

```toml
futures = "0.3"
```

And import `futures::StreamExt` when using [Streaming](#streaming) features.

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

* [Usage](#usage)
  * [Authentication and Authorization](#authentication-and-authorization-in-active-development) 
  * [Async and Blocking Client](#async-and-blocking-client)
    * [Async Client](#async-client-default)
    * [Blocking Client](#blocking-client)
  * [Cargo Feature Flags](#cargo-feature-flags)
  * [Paging (Delta, Next Links)](#paging)
    * [Streaming](#streaming)
    * [Channels](#channels)
  * [API Usage](#api-usage)
  * [Id vs Non-Id methods](#id-vs-non-id-methods-such-as-useruser-id-vs-users)
  * [Information about the project itself (contributor section coming soon)](#for-those-interested-in-the-code-itself-contributor-section-coming-soon)

### What APIs are available

The APIs available are generated from OpenApi configs that are stored in Microsoft's msgraph-metadata repository
for the Graph Api. There may be some requests and/or APIs not yet included in this project that are in the OpenApi
config but in general most of them are implemented.

# Usage

For extensive examples see the [examples directory on GitHub](https://github.com/sreeise/graph-rs-sdk/tree/master/examples)

### Authentication and Authorization (In Active Development)

The crate is undergoing major development in order to support all or most scenarios in the Microsoft Identity Platform
where its possible to do so. Another goal is to make the authentication/authorization impl much easier to use
by providing easy to use clients that follow similar designs to other sdks for the Identity Platform.

This includes token caches, automatic refresh of tokens, interactive web view authentication, and much more. 
The development is well underway - as of right now no merge has gone into master but changes will be there soon
and they will almost certainly be unstable in some respects while we continue to work on this. However, the crate
on crates.io is currently only updated on stable version releases.

### Async and Blocking Client

The crate can do both an async and blocking requests.

#### Async Client (default)

    graph-rs-sdk = "1.1.1"
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

    graph-rs-sdk = "1.1.1"

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
- `defalte`: Enables reqwest feature deflate. For more info see the [reqwest](https://crates.io/crates/reqwest) crate.
- `trust-dns`: Enables reqwest feature trust-dns. For more info see the [reqwest](https://crates.io/crates/reqwest) crate.

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

##### Custom Types
You can pass types your own types to API requests that require a request body by implementing `serde::Serialize`.

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

The sdk provides conveniance methods for getting all data in a paging scenario such as using next links or using [delta links to track changes to Graph data](https://learn.microsoft.com/en-us/graph/delta-query-overview).

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
