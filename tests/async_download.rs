use graph_error::{GraphFailure, GraphResult};
use graph_rs::prelude::*;
use std::path::{Path, PathBuf};
use test_tools::oauthrequest::OAuthRequest;
use test_tools::oauthrequest::DRIVE_THROTTLE_MUTEX;
use test_tools::support::cleanup::CleanUp;
use test_tools::FileUtils;

#[tokio::test]
async fn async_download() -> Result<(), GraphFailure> {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();

    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let file_location = "./test_files/download.txt";
        let mut clean_up = CleanUp::new(|| {
            let path = Path::new(file_location);
            if path.exists() {
                std::fs::remove_file(path).unwrap();
            }
        });

        clean_up.rm_files(file_location.into());

        let bearer = token.bearer_token();
        let client = Graph::new_async(&bearer);

        let download = client
            .v1()
            .drives(id.as_str())
            .drive()
            .download(":/download.txt:", "./test_files");

        let path_buf: PathBuf = download.send().await?;
        FileUtils::verify_contents(path_buf.as_path(), "Download Test Text File".into());
    }
    Ok(())
}
