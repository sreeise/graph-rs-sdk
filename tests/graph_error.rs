use graph_error::download::{AsyncDownloadError, BlockingDownloadError};
use graph_rs_sdk::error::*;
use graph_rs_sdk::prelude::*;
use reqwest::StatusCode;
use test_tools::oauthrequest::OAuthTestClient;
use test_tools::oauthrequest::{ASYNC_THROTTLE_MUTEX, DRIVE_THROTTLE_MUTEX};

fn test_graph_failure(err: GraphFailure, expect: GraphError) {
    match err {
        GraphFailure::GraphError(error) => test_graph_error(error, expect),
        GraphFailure::Io(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::ParseString(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::Utf8Error(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::ReqwestError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::ReqwestHeaderToStr(e) => {
            panic!("Expected GraphFailure::GraphError, got {}", e)
        }
        GraphFailure::SerdeError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::SerdeYamlError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::DecodeError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::RecvError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::BorrowMutError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::UrlParseError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::HyperError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::HyperHttpError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::HyperInvalidUri(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::Parse(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::GraphRsError(e) => panic!("Expected GraphFailure::GraphError, got {}", e),
        GraphFailure::CryptoError => panic!("Expected GraphFailure::GraphError, got CryptoError"),
        GraphFailure::HandlebarsRenderError(e) => {
            panic!("Expected GraphFailure::GraphError, got {}", e)
        }
        GraphFailure::HandlebarsTemplateRenderError(e) => {
            panic!("Expected GraphFailure::GraphError, got {}", e)
        }
    }
}

fn test_graph_error(error: GraphError, expect: GraphError) {
    assert_eq!(error.code, expect.code);
    assert_eq!(error.code_property(), expect.code_property());
    assert_eq!(error.message(), expect.message());
    assert!(error.request_id().is_some());
    assert!(error.date().is_some());
}

fn new_error(status_code: StatusCode, error_code: &str, message: &str) -> GraphError {
    GraphError {
        code: status_code,
        headers: None,
        error_message: ErrorMessage {
            error: Some(ErrorStatus {
                code: Some(error_code.to_string()),
                message: Some(message.to_string()),
                inner_error: None,
            }),
        },
    }
}

// Use a bad access token so we can test that the GraphError parses
// the response correctly.
#[test]
fn auth_graph_error() {
    let client = Graph::new("ACCESS_TOKEN");

    let response = client.v1().me().get_user().send();

    match response {
        Ok(_) => panic!("Got successful request for an invalid access token"),
        Err(err) => test_graph_failure(
            err,
            new_error(
                StatusCode::UNAUTHORIZED,
                "InvalidAuthenticationToken",
                "CompactToken parsing failed with error code: 80049217",
            ),
        ),
    }
}

#[tokio::test]
async fn async_auth_graph_error() {
    let client = Graph::new_async("ACCESS_TOKEN");

    let response = client.v1().me().get_user().send().await;

    if let Ok(_res) = response {
        panic!("Got successful request for an invalid access token");
    } else if let Err(err) = response {
        test_graph_failure(
            err,
            new_error(
                StatusCode::UNAUTHORIZED,
                "InvalidAuthenticationToken",
                "CompactToken parsing failed with error code: 80049217",
            ),
        );
    }
}

#[test]
fn upload_session_graph_error() {
    let client = Graph::new("ACCESS_TOKEN");

    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some("fail".to_string())
    });

    let response = client
        .v1()
        .user("0")
        .drive()
        .create_upload_session(
            ":/upload_session.txt:",
            "./test_files/async_upload_session.txt",
            &upload,
        )
        .send();

    if let Ok(_res) = response {
        panic!("Got successful request for an invalid access token");
    } else if let Err(err) = response {
        test_graph_failure(
            err,
            new_error(
                StatusCode::UNAUTHORIZED,
                "InvalidAuthenticationToken",
                "CompactToken parsing failed with error code: 80049217",
            ),
        );
    }
}

#[tokio::test]
async fn async_upload_session_graph_error() {
    let client = Graph::new_async("ACCESS_TOKEN");

    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some("fail".to_string())
    });

    let response = client
        .v1()
        .user("0")
        .drive()
        .create_upload_session(
            ":/async_upload_session.txt:",
            "./test_files/async_upload_session.txt",
            &upload,
        )
        .send()
        .await;

    if let Ok(_res) = response {
        panic!("Got successful request for an invalid access token");
    } else if let Err(err) = response {
        test_graph_failure(
            err,
            new_error(
                StatusCode::UNAUTHORIZED,
                "InvalidAuthenticationToken",
                "CompactToken parsing failed with error code: 80049217",
            ),
        );
    }
}

// Use a file that doesnt exist to test that the error from downloading
// is parsed correctly.

#[test]
fn drive_download_graph_error() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let download = client
            .v1()
            .drive(id.as_str())
            .download(":/non_existent_file.docx:", "./test_files");

        match download.send() {
			Ok(_) => { panic!("Got successful request for a downloading a file that should not exist") }
			Err(BlockingDownloadError::Graph(e)) =>
				test_graph_error(e, new_error(StatusCode::NOT_FOUND, "itemNotFound", "The resource could not be found.")),
			Err(e) => panic!("Expected BlockingDownloadError::Graph(GraphError..), but got a different variant: {}", e),
		}
    }
}

#[tokio::test]
async fn async_drive_download_graph_error() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let download = client
            .v1()
            .user(id.as_str())
            .drive()
            .download(":/non_existent_file.docx:", "./test_files");

        match download.send().await {
            Ok(_) => {
                panic!("Got successful request for a downloading a file that should not exist")
            }
            Err(AsyncDownloadError::Graph(e)) => test_graph_error(
                e,
                new_error(
                    StatusCode::NOT_FOUND,
                    "itemNotFound",
                    "The resource could not be found.",
                ),
            ),
            Err(e) => panic!(
                "Expected AsyncDownloadError::Graph(GraphError..), but got a different variant: {}",
                e
            ),
        }
    }
}
