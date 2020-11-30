use graph_rs::error::*;
use graph_rs::prelude::*;
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
        let download_client = client
            .v1()
            .user(id.as_str())
            .drive()
            .download(":/downloadtestdoc.txt:", "./test_files");
        let result = download_client.send();

        if let Err(err) = result {
            match err {
                GraphFailure::GraphRsError(err) => {
                    match err {
                        GraphRsError::DownloadFileExists { name} => {
                            if cfg!(target_os = "windows") {
                                assert_eq!(name, "./test_files\\downloadtestdoc.txt".to_string());
                            } else {
                                assert_eq!(name, "./test_files/downloadtestdoc.txt".to_string());
                            }
                        },
                        _ => panic!("Incorrect error thrown. Should have been GraphRsError::DownloadFileExists. Got: {:#?}", err)
                    }
                },
                _ => panic!("Incorrect error thrown. Should have been GraphRsError::DownloadFileExists. Got: {:#?}", err)
            }
        } else if let Ok(path) = result {
            panic!("Download request should have thrown GraphRsError::DownloadFileExists. Instead got successful PathBuf: {:#?}", path);
        }
    }
}

#[test]
fn download_is_err_config_dir_no_exists() {
    let client = Graph::new("");

    let download_client = client
        .v1()
        .me()
        .drive()
        .download("", "./test_files/download_dir");

    download_client.create_dir_all(false);

    let result = download_client.send();

    if let Err(err) = result {
        match err {
            GraphFailure::GraphRsError(err) => {
                match err {
                    GraphRsError::DownloadDirNoExists { dir } => {
                        assert_eq!("./test_files/download_dir".to_string(), dir)
                    },
                    _ => panic!("Incorrect error thrown. Should have been GraphRsError::DownloadDirNoExists. Got: {:#?}", err)
                }
            },
            _ => panic!("Incorrect error thrown. Should have been GraphRsError::DownloadDirNoExists. Got: {:#?}", err)
        }
    } else if let Ok(path) = result {
        panic!("Download request should have thrown GraphRsError::DownloadDirNoExists. Instead got successful PathBuf: {:#?}", path);
    }
}
