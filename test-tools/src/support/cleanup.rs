use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct CleanUp {
    // As in rm -r file for Linux or in other words remove the file.
    rm_f: Vec<String>,
}

impl CleanUp {
    pub fn new<F>(f: F) -> CleanUp
    where
        F: Fn(),
    {
        f();
        CleanUp::default()
    }

    pub fn rm_files(&mut self, s: String) {
        self.rm_f.push(s);
    }
}

impl Drop for CleanUp {
    fn drop(&mut self) {
        for s in &self.rm_f {
            if Path::new(s.as_str()).exists() {
                fs::remove_file(Path::new(s.as_str())).unwrap();
            }
        }
    }
}
