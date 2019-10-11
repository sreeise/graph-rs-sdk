# graph-rs

[![Build Status](https://travis-ci.org/sreeise/graph-rs.svg?branch=master)](https://travis-ci.org/sreeise/graph-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/llvpt7xiy53dmo7a/branch/master?svg=true)](https://ci.appveyor.com/project/sreeise/rust-onedrive)

### Graph API Client in Rust

Disclaimer:
Integrates with OneDrive, Mail, Calendars, and OneNote. 
Additional APIs are being added and may not be stable. Please create an issues if you 
experience any problems. Note that some APIs may be specific to the Graph v1.0 or Graph beta. 

### Install and Building - Requires Rust nightly
For Windows install the Windows build tools (And related Visual Studio components for Rust to work on Windows).

Install OpenSSL - see https://docs.rs/openssl for install information.

Normal Rust build using cargo. The nightly version is set in the rust-toolchain file.

    $ cargo build

### Tests and Docs
Of the portions that are implemented there are also examples and docs. Run: 

    $ cargo doc --no-deps --open

### Use - subject to change.

See the examples directory for more.

Each request has an three ways of getting the response. The responses are returned with the headers
and the deserialized body (if there is one) wrapped in the GraphResponse struct.

##### serde_json::Value response
The value() method returns the response in JSON using serde_json::Value.

The json() method can be used to convert the response to your own types. These
types must implement serde::Deserialize.

```rust
use graph_rs::prelude::*;
        
let client = Graph::new("ACCESS_TOKEN");

// returns GraphResponse<serde_json::Value>
let value_response = client.v1()
    .me()
    .get()
    .value()?;

println!("{:#?}", value_response);
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

#### The send method and Graph types
The send() method always returns a struct based on that request. These structs are Graph types generated
from the Microsoft Graph metadata document located here: https://graph.microsoft.com/v1.0/$metadata
Beware that the fields on these types do not change and sending a request that alters the response, such 
as OData queries, may not return all of the fields requested. If you use OData queries it is 
recommended that you use your own struct or the the value() method for the response.

```rust
use graph_rs::prelude::*;

// Returns GraphResponse<Collection<DriveItem>>
let response = client.v1()
    .me()
    .drive()
    .root_children()
    .send()?;
        
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
    .value()?;
        
println!("{:#?}", drive_item):
    
// Use path based addressing
// Pass the location of the item to get from the root folder.
 // Start the path with :/ and end with :
    
let response = client.v1()
    .me()
    .drive()
    .get_item(":/document.docx:")
    .value()?;
        
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
    .value()?;
              
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
    .value()?;
        
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
    })).value();

println!("{:#?}", draft_message_response);
        
let send_mail_response = client.v1()
    .me()
    .mail()
    .messages()
    .send_mail()
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
    .value()?;
            
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
    .value()?;
    
println!("{:#?}", response.value()):
```
   
#### Batch Requests

Batch requests use a mpsc::channel and return the receiver
for responses.

```rust
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
            "url": format!("/users/{}/drive/activities", USER_ID)
        }
    ]
});

let recv = client.v1()
    .batch(&json)
    .send()?;

match recv.recv() {
    Ok(value) => {
        println!("{:#?}", value);
    },
    Err(e) => {
        println!("Error: {:#?}", e);
    },
}
```   
        
        
### Coverage

This is not an exhaustive list but it does give a good
overview of most of what is covered so far.  

[x] indicates that its covered.      
        
#### OneDrive  
   
OneDrive API               | Covered
-------------              | -------------
List Children              | [x]
Get Item                   | [x]
Get Drive                  | [x]
List Drives                | [x] 
Recent Files               | [x] 
Shared Files               | [x] 
Get Thumbnails             | [x]
Create Folder              | [x]
Update Item                | [x]
Delete Item                | [x]
Move Item                  | [x]
Copy Item                  | [x]
Download                   | [x]
Upload                     | [x]
Upload Session             | [x]
Track Changes              | [ ]
List Versions              | [x]
Delta                      | [x]
Search Items               | [x]
Preview Item               | [x]
Get Item Analytics         | [ ]
Permissions                | [ ]
Get Special Folder         | [x]
        
#### Mail
   
Mail API                                | Covered
-------------                           | -------------
Message - List                          | [x]
Message - Get                           | [x]
Message - Create                        | [x]
Message - Copy                          | [x]
Message - Forward                       | [x]
Message - Move                          | [x]
Message - Create Forward                | [x]
Message - Create Reply                  | [x]
Message - Create Reply All              | [x]
Message - Reply                         | [x]
Message - Reply All                     | [x]
Message - Send                          | [x]
Message - Send Mail                     | [x]
Message - List Attachment               | [x]
Message - Add Attachment                | [x]
Mail Folder - List                      | [x]
Mail Folder - Get                       | [x]
Mail Folder - Create                    | [x]
Mail Folder - Copy                      | [x]
Mail Folder - Update                    | [x]
Mail Folder - Update Message            | [x]
Mail Folder - Create Message            | [x]
Mail Folder - Forward Message           | [x]
Mail Folder - Move Message              | [x]
Mail Folder - Message Create Forward    | [x]
Mail Folder - Message Create Reply      | [x]
Mail Folder - Message Create Reply All  | [x]
Mail Folder - Message Reply             | [x]
Mail Folder - Message Reply All         | [x]
Mail Folder - Get Message Delta         | [x]
Mail Folder - List Child Folders        | [x]
Mail Folder - Create Child Folder       | [x]
Attachments                             | [x]
Search Folder                           | [x]
Rules                                   | [x]
Focused Inbox                           | [x]

Calendars

Calendar API            | Covered
-------------           | -------------
List Calendars          | [x]
Create calendar         | [x]
Get Calendar            | [x]
Update Calendar         | [x]
Create Calendar         | [x]
Delete Calendar         | [x]
List Calendars Group    | [x]
Get Calendar Group      | [x]
Update Calendar Group   | [x]
Create Calendar Group   | [x]
Delete Calendar Group   | [x]
Get Schedule            | [ ]
Find Meeting Times      | [ ]
Calendar Events         | [ ]
Calendar Attachments    | [x]
Categories              | [x]
List Views              | [x]
Calendar Group View     | [x]
Get Delta View          | [ ]
Reminder View           | [ ] 

Users

Users API               | Covered
-------------           | -------------
List Users              | [x]
Get User                | [x]
Create User             | [x]
Update User             | [x]
Delete User             | [x]
Get Delta               | [x]

OneNote

OneNote API                     | Covered
-------------                   | -------------
List Notebooks                  | [x]
Get Notebook                    | [x]
List Notebook Sections          | [x]
Create Notebook                 | [x]
Copy Notebook                   | [x]
Create Notebook Section         | [x]
Get Recent Notebooks            | [x]
List Section Group              | [x]
Get Section Group               | [x]
List Section Group Sections     | [x]
Create Section Group            | [x]
Create Section Group Section    | [x]
Create Notebook Section         | [x]
List Pages                      | [x]
Get Page                        | [x]
Page Copy To Section            | [x]
Delete Page                     | [x]
Update Page                     | [ ]
Resources                       | [ ]