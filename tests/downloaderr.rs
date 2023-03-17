use graph_error::download::BlockingDownloadError;
use graph_rs_sdk::prelude::*;
use test_tools::oauthrequest::OAuthTestClient;
use test_tools::oauthrequest::DRIVE_THROTTLE_MUTEX;

#[test]
#[should_panic]
fn download_config_dir_no_exists() {
    let client = Graph::new("");

    let download_client = client
        .v1()
        .me()
        .drive()
        .download("", "./test_files/download_dir");

    download_client.create_dir_all(false);

    let _ = download_client.send().unwrap();
}

#[test]
fn download_config_file_exists() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        match client.v1().user(id.as_str()).drive().download(":/downloadtestdoc.txt:", "./test_files").send() {
			Ok(path) => panic!("Download request should have thrown BlockingDownloadError::FileExists. Instead got successful PathBuf: {:#?}", path),

			Err(BlockingDownloadError::FileExists(name)) => {
				if cfg!(target_os = "windows") {
					assert_eq!(name, "./test_files\\downloadtestdoc.txt".to_string());
				} else {
					assert_eq!(name, "./test_files/downloadtestdoc.txt".to_string());
				}
			}

			Err(err) => panic!("Incorrect error thrown. Should have been BlockingDownloadError::FileExists. Got: {:#?}", err),
		}
    }
}

#[test]
fn download_is_err_config_dir_no_exists() {
    let client = Graph::new("");

    let request = client
        .v1()
        .me()
        .drive()
        .download("", "./test_files/download_dir");

    request.create_dir_all(false);

    match request.send() {
		Ok(path) => panic!("Download request should have thrown GraphRsError::DownloadDirNoExists. Instead got successful PathBuf: {:#?}", path),
		Err(BlockingDownloadError::TargetDoesNotExist(dir)) => assert_eq!("./test_files/download_dir".to_string(), dir),
		Err(err) => panic!("Incorrect error thrown. Should have been GraphRsError::DownloadDirNoExists. Got: {:#?}", err),
	}
}
