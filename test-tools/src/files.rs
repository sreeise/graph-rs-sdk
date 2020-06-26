use std::path::Path;

pub struct FileUtils;

impl FileUtils {
    pub fn verify_contents<P: AsRef<Path>>(path: P, content: String) {
        let file_content = std::fs::read_to_string(path).unwrap();
        assert_eq!(file_content, content);
    }
}
