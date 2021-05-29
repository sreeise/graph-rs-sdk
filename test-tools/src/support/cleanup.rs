use std::{fs, path::Path};

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

#[derive(Debug, Default)]
pub struct AsyncCleanUp {
    // As in rm -r file for Linux or in other words remove the file.
    rm_f: Vec<String>,
}

impl AsyncCleanUp {
    pub fn new<F>(f: F) -> AsyncCleanUp
    where
        F: Fn(),
    {
        f();
        AsyncCleanUp::default()
    }

    pub fn new_remove_existing(path: &str) -> AsyncCleanUp {
        let path = Path::new(path);
        if path.exists() {
            futures::executor::block_on(tokio::fs::remove_file(path)).unwrap();
        }
        AsyncCleanUp::default()
    }

    pub fn rm_files(&mut self, s: String) {
        self.rm_f.push(s);
    }
}

impl Drop for AsyncCleanUp {
    fn drop(&mut self) {
        for s in &self.rm_f {
            let path = Path::new(s.as_str());
            if path.exists() {
                futures::executor::block_on(tokio::fs::remove_file(path)).unwrap();
            }
        }
    }
}
