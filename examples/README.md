# Examples

To use the examples effectively, create a folder labeled: "example_files"
under the examples directory:
    
    ./examples/example_files
     
The examples have various places where the OAuth and Drive instances are stored
in JSON files to make it easier to use across multiple examples.

A good place to start is the rocket example: rocket_example.rs

### Using the OneDrive and Graph API

Drive requests can be done for either the OneDrive V1.0 endpoint
or the graph beta endpoint.

The basic way of creating a request is by selecting the version and
then selecting the type of request.

Here is how you would get the metadata for all files in the root
of a drive:

    let mut drive = Drive::new("ACCESS_TOKEN");
    
    // Create the request.
    let request = drive.v1().drive_root_child();
    
    // Send the request.
    let collection: DriveItemCollection = request.send().unwrap();