# graph-rs

[![Build Status](https://travis-ci.org/sreeise/graph-rs.svg?branch=master)](https://travis-ci.org/sreeise/graph-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/llvpt7xiy53dmo7a/branch/master?svg=true)](https://ci.appveyor.com/project/sreeise/rust-onedrive)

### Graph API Client in Rust

There are several parts to this project:

graph-oauth: OAuth client for getting access/refresh tokens from the Graph api.

graph-error: Errors that come back from the Graph api.

graph-rs(src directory): The Graph client for interacting with the Graph api. The oauth client is also reexported from here.

There are many api's already implemented including activities, attachments, calendar, contacts, drive, education, groups, mail, onenote, planner, and users.
See upcoming changes below for what will be added in the future.

Installation and basic usage can be found below and there are extensive examples in the examples directory.

### Upcoming Changes

This repository is still in development and will change considerably. There are several implementations of Graph api's that need to be added. I am also getting the project ready for a release on crates.io but I don't want to release something that is 1. Not maintainable long term and 2. Not fully implemented. 

I am attempting to maintain backwards compatibility for those who already use this repository as well. I will post an update if that ends up not being possible. To be clear, the way in which the Graph client and the already implemented api's are used will not change. The major changes coming up are adding functionality not changing it.

To make development easier and to provide a maintainable library, work is currently being done for parsing open api configs for the Graph Api. After this work is completed any new api's added or removed will be based on these configurations. Also after the work is done, I will look into adding a first version to crates.io.
 
### Install and Build - Requires Rust nightly

Normal Rust build using cargo. The nightly version is set in the rust-toolchain file.

    $ cargo build

### Tests and Docs
Of the portions that are implemented there are also examples and docs. Run: 

    $ cargo doc --no-deps --open

### Use - subject to change.

The client supports both blocking and async requests.

### Blocking Client

To use the blocking client

    let client =  Graph::new("ACCESS_TOKEN");


### Async Client

To use the async client

    let client = Graph::new_async("ACCESS_TOKEN");
    
#### The send method and Graph types
The send() method is the main method for sending a request. The return value will be wrapped
in a response object and the body will be one of:
   
    1. serde_json::Value
    
    2. Collection<serde_json::Value>
   
    3. Content (204 responses that return a content field)

```rust
use graph_rs::prelude::*;

let client =  Graph::new("ACCESS_TOKEN");

// Returns GraphResponse<Collection<serde_json::Value>>
let response = client.v1()
    .me()
    .drive()
    .root_children()
    .send()
    .unwrap();
        
println!("{:#?}", response);  
```

For async requests use the await keyword.

```rust
use graph_rs::prelude::*;

let client =  Graph::new_async("ACCESS_TOKEN");

// Returns GraphResponse<Collection<serde_json::Value>>
let response = client.v1()
    .me()
    .drive()
    .root_children()
    .send()
    .await
    .unwrap();
        
println!("{:#?}", response);  
```

##### Custom Types
The json() method can be used to convert the response to your own types. These
types must implement serde::Deserialize.

```rust
use graph_rs::prelude::*;
        
let client = Graph::new("ACCESS_TOKEN");
        
#[derive(Debug, Serialize, Deserialize)]
pub struct DriveItem {
    id: Option<String>,
    name: Option<String>,
    // ... Any other fields
}
        
let response: DriveItem = client.v1()
    .me()
    .drive()
    .get_item("ITEM_ID")
    .json()?;
        
println!("{:#?}", response);   
``` 

### OneDrive
```rust
use graph_rs::prelude::*;
    
let client = Graph::new("ACCESS_TOKEN");
    
let response = client.v1()
    .me()
    .drive()
    .get_item("ITEM_ID")
    .send()?;
    
println!("{:#?}", response.value());
    
let folder: HashMap<String, serde_json::Value> = HashMap::new();

let drive_item = client
    .v1()
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
        
println!("{:#?}", drive_item):
    
// Use path based addressing
// Pass the location of the item to get from the OneDrive root folder.
// Start the path with :/ and end with :
    
let response = client.v1()
    .me()
    .drive()
    .get_item(":/document.docx:")
    .send()?;
        
println!("{:#?}", response.value());
```
    
### Mail

```rust
use graph_rs::prelude::*;
        
let client = Graph::new("ACCESS_TOKEN");
        
// Returns serde_json::Value
let json = client.v1()
    .users("USER_ID")
    .mail()
    .messages()
    .list()
    .send()?;
              
// Create a message
let response = client.v1()
    .users("USER_ID")
    .mail()
    .messages()
    .create(&serde_json::json!({
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
        
println!("{:#?}", response.value()); // => Message

// Create a message in a well known folder
let draft_message_response = client.v1()
    .me()
    .mail()
    .mail_folder()
    .messages()
    .create("drafts", &serde_json::json!({
        "subject":"Did you see last night's game?",
        "importance":"Low",
        "body":{
            "contentType":"HTML",
            "content":"They were <b>awesome</b>!"
        },
        "toRecipients":[
            {
                "emailAddress":{
                    "address":"AdeleV@contoso.onmicrosoft.com"
                }
            }
         ]
    })).send();

println!("{:#?}", draft_message_response);
        
let send_mail_response = client.v1()
    .me()
    .mail()
    .messages()
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
                                       
println!("{:#?}", send_mail_response);
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
    .mail()
    .messages()
    .create(&message)
    .send()?;
            
println!(":#?", response);
```              

#### OData Queries

```rust
use graph_rs::prelude::*;
            
let client = Graph::new("ACCESS_TOKEN");
    
// Get all files in the root of the drive
// and select only specific properties.
let response = client.v1()
    .me()
    .drive()
    .root_children()
    .select(&["id", "name"])
    .send()?;
    
println!("{:#?}", response.value()):
```
   
#### Batch Requests

Batch requests use a mpsc::channel and return the receiver
for responses.

```rust
use graph_rs::prelude::*;
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
