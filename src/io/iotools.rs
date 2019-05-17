use crate::drive::ItemResult;
use std::fs;
use std::path::Path;
pub struct IOTools;

#[macro_export]
macro_rules! log {
    ($msg:expr) => {{
        println!("{:#?}", $msg);
    }};
}

impl IOTools {
    pub fn create_dir<P: AsRef<Path>>(directory: P) -> ItemResult<()> {
        if let Some(location) = directory.as_ref().to_str() {
            if !Path::new(&location).exists() {
                fs::create_dir_all(&directory)?;
            }
        }
        Ok(())
    }
}
