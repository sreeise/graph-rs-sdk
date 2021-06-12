use graph_error::GraphFailure;
use graph_rs_sdk::prelude::*;

// This example shows how you can handle errors returned from Microsoft Graph.
//
// The GraphError struct represents a Microsoft Graph Error that resembles the type of errors
// described in the docs: https://docs.microsoft.com/en-us/graph/errors
//
// When an error is returned from Microsoft graph a GraphFailure::GraphError(GraphError) is
// returned which you can use to get the request id and message for debugging. The inner error
// of GraphError also typically provides better info on the cause of the error.
//
// Since parts of the error are wrapped in other structs and Options you can use
// the convenience methods on GraphError making it easier to retrieve error information:
//
// 1. err.request_id() to look up the request.
// 2. err.message() to get the dynamically generated error info specific to this request.
// 3. err.detailed_error_code() a Detailed error code that can be looked
// up here: https://docs.microsoft.com/en-us/graph/errors#detailed-error-codes
// 4. err.code_property() A code property to help describe the error that can be
// looked up here: https://docs.microsoft.com/en-us/graph/errors#code-property

fn main() {
    // Use a fake access token to cause an error to be returned.
    let client = Graph::new("1234");

    let response = client.v1().me().get_user().send();

    if let Err(e) = response {
        match e {
            GraphFailure::GraphError(err) => {
                // Basic error information from the HTTP Response
                println!(
                    "Description: {:#?}",
                    err.code.canonical_reason().unwrap_or_default()
                );
                println!("Code: {:#?}\n", err.code);

                // Graph error information:
                println!("Code property: {:#?}", err.code_property());
                println!("Request ID: {:#?}", err.request_id());
                println!("Message: {:#?}", err.message());
                println!("Date: {:#?}", err.date());
                println!("Detailed Error Code: {:#?}", err.detailed_error_code());
            }
            _ => {}
        }
    }
}
