use graph_error::GraphResult;
use graph_http::traits::ResponseExt;
use graph_rs_sdk::Graph;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// The path where you wan to place the file in OneDrive
// including the file name. For the root folder just
// put the file name here like so: :/file.ext:
static PATH_IN_ONE_DRIVE: &str = ":/Documents/file.ext:";

// The conflict behavior can be one of: fail, rename, or replace.
static CONFLICT_BEHAVIOR: &str = "rename";

pub async fn channel(file: tokio::fs::File) -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some("fail".to_string())
    });

    let response = client
        .me()
        .drive()
        .item_by_path(":/upload_session_file.txt:")
        .create_upload_session(&upload)
        .send()
        .await?;

    let mut upload_session_task = response.into_upload_session_async_read(file).await?;
    let mut receiver = upload_session_task.channel()?;

    while let Some(result) = receiver.recv().await {
        match result {
            Ok(response) => {
                println!("{response:#?}");

                let body: serde_json::Value = response.json().await?;
                println!("{body:#?}");
            }
            Err(err) => panic!("{:#?}", err),
        }
    }

    Ok(())
}
