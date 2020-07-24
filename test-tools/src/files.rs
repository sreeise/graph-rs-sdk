use std::path::Path;

pub struct FileUtils;

impl FileUtils {
    pub fn verify_contents<P: AsRef<Path>>(path: P, content: String) {
        let file_content = std::fs::read_to_string(path).unwrap();
        assert_eq!(file_content, content);
    }

    pub async fn verify_contents_async<P: AsRef<Path>>(path: P, content: String) {
        let file_content = tokio::fs::read_to_string(path).await.unwrap();
        assert_eq!(file_content, content);
    }
}
