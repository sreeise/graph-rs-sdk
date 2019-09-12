# Examples

To use the examples effectively, create a folder labeled: "example_files"
under the examples directory:
    
    ./examples/example_files
     
The examples have various places where the response metadata from requests
is stored in in JSON files to make it easier to use across multiple examples.

A good place to start is the rocket example: rocket_example.rs for authorization
and access tokens.

### Using the Graph API

    use graph_rs::prelude::*;
    
    let client = Graph::new("ACCESS_TOKEN");
    
    // Drive requests.
    let drive_item: GraphResponse<DriveItem> = client.v1()
                        .me()
                        .drive()
                        .get_item()
                        .by_id("ITEM_ID");
                        .send()
                        .unwrap():
    println!("{:#?}", drive_item.value()):
    
    // User requests
    let user: GraphResponse<User> = client.v1()
                    .me()
                    .user()
                    .get()
                    .send()
                    .unwrap();
    println!({:#?}, user.value());