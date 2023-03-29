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

### Use

The client supports both blocking and async requests.

### Cargo Feature Flags

- `native-tls`: Use the `native-tls` TLS backend (OpenSSL on *nix, SChannel on Windows, Secure Transport on macOS). 
- `rustls-tls`: Use the `rustls-tls` TLS backend (cross-platform backend, only supports TLS 1.2 and 1.3).

Default features: `default=["native-tls"]`

### Blocking Client

To use the blocking client

```rust
use graph_rs_sdk::prelude::*;

fn main() {
  let client =  Graph::new("ACCESS_TOKEN");
}
```

### Async Client

To use the async client

```rust
use graph_rs_sdk::prelude::*;

fn main() {
  let client = Graph::new_async("ACCESS_TOKEN");
}
```
    
#### The send method and Graph types
The send() method is the main method for sending a request. The return value will be wrapped
in a response object, `GraphResponse<T>` and the body will be a serde_json::Value. 
If the response is a 204 no content and there is no body then the response body returned will 
just be a serde_json::Value with an empty string.

```rust
use graph_rs_sdk::prelude::*;

let client =  Graph::new("ACCESS_TOKEN");

// Returns GraphResponse<serde_json::Value>
let response = client.v1()
    .me()
    .drive()
    .get_drive()
    .send()
    .unwrap();
```

For async requests use the await keyword.

```rust
use graph_rs_sdk::prelude::*;

let client =  Graph::new_async("ACCESS_TOKEN");

// Returns GraphResponse<serde_json::Value>
let response = client.v1()
    .me()
    .drive()
    .get_drive()
    .send()
    .await
    .unwrap();
        
println!("{:#?}", response);  

// Get the body of the response
println!("{:#?}", response.body());
```

##### Custom Types
The json() method can be used to convert the response body to your own types. These
types must implement `serde::Deserialize`.

```rust
use graph_rs_sdk::prelude::*;
        
let client = Graph::new("ACCESS_TOKEN");
        
#[derive(Debug, Serialize, Deserialize)]
pub struct DriveItem {
    id: Option<String>,
    name: Option<String>,
    // ... Any other fields
}
        
let response: GraphResponse<DriveItem> = client.v1()
    .me()
    .drive()
    .get_items("ITEM_ID")
    .json()?;
        
println!("{:#?}", response);   
```

GraphAPI will limit the number of returned items per page even if you specify a very large `.top()` value and will provide a `next_link` link for you to retrieve the next batch.
You can use the `.paging()` method and graph-rs will follow the next_links to return the whole collection.

```rust
use graph_rs_sdk::prelude::*;
        
let client = Graph::new("ACCESS_TOKEN");

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: Option<String>,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: Option<String>,
}

let users_resp: GraphResponse<Vec<User>> = client
    .v1()
    .users()
    .list_user()
    .select(&["id", "userPrincipalName"])
    .paging()
    .json()
    .unwrap();

let users = users_resp.into_body();
println!("{:?}", users);
``` 

### OneDrive

Make requests to drive using a drive id or through specific drives for me, sites,
users, and groups.

```rust
use graph_rs_sdk::prelude::*;
    
let client = Graph::new("ACCESS_TOKEN");

// Some requests don't require an id.
let response = client.v1()
    .drives()
    .get_drive();

// Using a drive id.
let response = client.v1()
    .drive("DRIVE-ID")
    .get_items("ITEM_ID")
    .send()?;

// Using me.
let response = client.v1()
    .me()
    .drive()
    .get_items("ITEM_ID")
    .send()?;
    
println!("{:#?}", response);

// Using users.
let response = client.v1()
    .users("USER_ID")
    .drive()
    .get_items("ITEM_ID")
    .send()?;

println!("{:#?}", response);

// Using sites.
let response = client.v1()
    .sites("SITE-ID")
    .drive()
    .get_items("ITEM_ID")
    .send()?;

println!("{:#?}", response);
```

Create a folder.

```rust
let folder: HashMap<String, serde_json::Value> = HashMap::new();

let response = client.v1()
    .me()
    .drive()
    .create_folder(
        "PARENT_FOLDER_ID",
         &serde_json::json!({
            "name": "docs",
            "folder": folder,
            "@microsoft.graph.conflictBehavior": "fail"
         }),
    )
    .send()?;
        
println!("{:#?}", response);
```

Path based addressing for drive.

```rust
// Pass the path location of the item staring from the OneDrive root folder.
// Start the path with :/ and end with :
    
let response = client.v1()
    .me()
    .drive()
    .get_items(":/documents/document.docx:")
    .send()?;
        
println!("{:#?}", response.body());
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

// Send mail.
let response = client.v1()
    .user("USER-ID")
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
    .send()?;
                                       
println!("{:#?}", response);
```

Mail folders

```rust
// Create a mail folder.
let response = client.v1()
    .user("USER-ID")
    .mail_folders()
    .create_mail_folders(&serde_json::json!({
        "displayName": "Clutter"
    }))
    .send()?;

// List messages in a mail folder.
let response = client.v1()
    .me()
    .mail_folder("drafts")
    .messages()
    .list_messages()
    .send()?;

// Create messages in a mail folder.
let response = client.v1()
    .user("USER-ID")
    .mail_folder("drafts")
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

let mut body = HashMap::new();
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
}
        
// Create a message
let response = client.v1()
    .me()
    .messages()
    .create_messages(&message)
    .send()?;
            
println!(":#?", response);
```              

#### OData Queries

```rust
use graph_rs_sdk::prelude::*;
            
let client = Graph::new("ACCESS_TOKEN");
    
// Get all files in the root of the drive
// and select only specific properties.
let response = client.v1()
    .me()
    .drive()
    .get_drive()
    .select(&["id", "name"])
    .send()?;
    
println!("{:#?}", response.body());
```
   
#### Batch Requests

Batch requests use a mpsc::channel and return the receiver
for responses.

```rust
use graph_rs_sdk::prelude::*;
use std::error::Error;

static USER_ID: &str = "USER_ID";

let client = Graph::new("ACCESS_TOKEN");

let json = serde_json::json!({
    "requests": [
        {
            "id": "1",
            "method": "GET",
            "url": format!("/users/{}/drive", USER_ID)
        },
        {
            "id": "2",
            "method": "GET",
            "url": format!("/users/{}/drive/root", USER_ID)
        },
        {
            "id": "3",
            "method": "GET",
            "url": format!("/users/{}/drive/recent", USER_ID)
        },
        {
            "id": "4",
            "method": "GET",
            "url": format!("/users/{}/drive/root/children", USER_ID)
        },
        {
            "id": "5",
            "method": "GET",
            "url": format!("/users/{}/drive/special/documents", USER_ID)
        }
    ]
});

let recv = client
    .v1()
    .batch(&json)
    .send();

loop {
    match recv.recv() {
        Ok(delta) => {
            match delta {
                Delta::Next(response) => {
                    println!("{:#?}", response);
                },
                Delta::Done(err) => {
                    println!("Finished");

                    // If the delta request ended in an error Delta::Done
                    // will return Some(GraphFailure)
                    if let Some(err) = err {
                        println!("Error: {:#?}", err);
                        println!("Description: {:#?}", err.description());
                    }

                    // All next links have been called.
                    // Break here. The channel has been closed.
                    break;
                },
            }
        },
        Err(e) => {
            println!("{:#?}", e.description());
            break;
        },
    }
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
