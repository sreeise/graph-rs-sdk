# graph-rs

![Build](https://github.com/sreeise/graph-rs/actions/workflows/build.yml/badge.svg)
[![Build status](https://ci.appveyor.com/api/projects/status/llvpt7xiy53dmo7a/branch/master?svg=true)](https://ci.appveyor.com/project/sreeise/rust-onedrive)

### Now available on stable Rust at [crates.io](https://crates.io/crates/graph-rs-sdk)

    graph-rs-sdk = "0.3.1"

0.1.0 and above use stable Rust. Anything before 0.1.0 uses nightly Rust.

### Microsoft Graph API Client in Rust

Installation and basic usage can be found below and there are extensive examples in the example's directory
included in the project on [GitHub](https://github.com/sreeise/graph-rs).

### Feature requests or Bug reports.

For bug reports please file an issue on GitHub and a response or fix will be given as soon as possible.

The [Discussions](https://github.com/sreeise/graph-rs/discussions) tab on [GitHub](https://github.com/sreeise/graph-rs/discussions) is enabled so feel free to stop by there with any questions or feature requests as well. For bugs, please file
an issue first. Other than that feel free to ask questions, provide tips to others, and talk about the project in general.

### What Api's are available

The Api's available are generated from OpenApi configs that are stored in Microsoft's msgraph-metadata repository
for the Graph Api. There may be some requests and/or Api's not yet included in this project that are in the OpenApi
config but in general most of them are implemented.

## Use

For extensive examples see the [examples directory on GitHub](https://github.com/sreeise/graph-rs/tree/master/examples)

### Cargo Feature Flags

- `native-tls`: Use the `native-tls` TLS backend (OpenSSL on *nix, SChannel on Windows, Secure Transport on macOS). 
- `rustls-tls`: Use the `rustls-tls` TLS backend (cross-platform backend, only supports TLS 1.2 and 1.3).

Default features: `default=["native-tls"]`
    
#### The send method
The send() method is the main method for sending a request and returns a reqwest::Response. See the
reqwest crate for information on the Response type.

```rust
use graph_rs_sdk::prelude::*;

let client =  Graph::new("ACCESS_TOKEN");

let response = client.v1()
    .me()
    .drive()
    .get_drive()
    .send()
    .await
    .unwrap();

println!("{:#?}", &response);

let body: serde_json::Value = response.json().await.unwrap();
println!("{:#?}", body);
```

##### Custom Types
You can implement your own types by utilizing methods from reqwest::Response. These types must implement `serde::Deserialize` or `DeserializedOwned`.
See the reqwest crate for more info.

```rust
use graph_rs_sdk::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DriveItem {
    id: Option<String>,
    name: Option<String>,
    // ... Any other fields
}

let client = Graph::new("ACCESS_TOKEN");

let response = client
    .me()
    .drive()
    .item("item_id")
    .get_items()
    .send()
    .await
    .unwrap();

let drive_item: DriveItem =  response.json().await.unwrap();

println!("{:#?}", drive_item);
```

GraphAPI will limit the number of returned items per page even if you specify a very large `.top()` value and will provide a `next_link` link for you to retrieve the next batch.
You can use the `.paging()` method to access several different ways to get/call next link requests.


# JSON paging

If you just want a quick and easy way to get all next link responses or the JSON bodies you can use the `paging().json()` method which will exhaust all
next link calls and return all the responses in a `VecDeque<Response<T>>`. Keep in mind that the larger the volume of next link calls that need to be
made the longer the return delay will be when calling this method.

```rust
use graph_rs_sdk::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: Option<String>,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: Option<String>,
}

async fn paging() -> GraphResult<()> {
  let client = Graph::new("ACCESS_TOKEN");

  let deque = client
      .users()
      .list_user()
      .select(&["id", "userPrincipalName"])
      .paging()
      .json::<User>()
      .await?;
  
  println!("{:#?}", deque);
  
  Ok(())
}
``` 

### OneDrive

Make requests to drive using a drive id or through specific drives for me, sites,
users, and groups.

```rust
use graph_rs_sdk::prelude::*;

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
use graph_rs_sdk::prelude::*;
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
use graph_rs_sdk::prelude::*;
        
let client = Graph::new("ACCESS_TOKEN");
        
// List messages for a user.
let response = client.v1()
    .user("USER-ID")
    .messages()
    .list_messages()
    .send()?;

// List messages using me.
let response = client.v1()
    .me()
    .messages()
    .list_messages()
    .send()?;
             
// Create a message
let response = client.v1()
    .user("USER_ID")
    .messages()
    .create_messages(&serde_json::json!({
        "subject":"Did you see last night's game?",
        "importance":"Low",
        "body":{
            "contentType":"HTML",
                "content":"They were <b>awesome</b>!"
            },
        "toRecipients":[{
            "emailAddress":{
                "address":"AdeleV@contoso.onmicrosoft.com"
            }
        }]
    }))
    .send()?;
        
println!("{:#?}", response.body()); // => Message
```

#### Create message
```rust
use graph_rs_sdk::prelude::*;

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
}

```

#### Send mail

```rust
use graph_rs_sdk::prelude::*;

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
use graph_rs_sdk::prelude::*;

async fn create_mail_folder_message() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
  
    let response = client
        .me()
        .mail_folder(MAIL_FOLDER_ID)
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

#### Get Inbox Messages
```rust
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
use graph_rs_sdk::prelude::*;

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
use graph_rs_sdk::prelude::*;

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

## For those interested in the code itself

### Build

Normal Rust build using cargo.

    $ cargo build

### Docs
Of the portions that are implemented there are also examples and docs. Run:

    $ cargo doc --no-deps --open

There are several parts to this project:

* graph-oauth: OAuth client for getting access/refresh tokens from the Graph api.
* graph-error: Errors that come back from the Graph Api.
* graph-codegen: OpenApi parser and generator specifically for the Graph Api's.
* graph-core: Common types shared across all or multiple parts of the project
* test-tools: Helps facilitate project testing.  
* graph-rs (src directory): The Graph client for interacting with the Graph Api
  including the Api's generated from the OpenApi config. The oauth client is also 
  reexported from here.
  
### Testing

The project does validation testing for the Graph Api's using a developer sandbox to ensure the implementation
provided here works correctly. However, the total amount of individual requests that can be called and that is provided in this 
project is well into the hundreds, and some areas are lacking in coverage. The goal is to cover the main parts of each
Api.

Tests are run on Ubuntu Linux and Windows 10 instances.

### graph-rs versions before 12/13/2020

The graph-rs project is now published on crates.io and that is the recommended version to use.
Because of the many changes that came with publishing, if you still need to migrate or would like 
to use the previous version then you can use the v2master branch which is still the same as the 
master branch before it was published as a crate.
