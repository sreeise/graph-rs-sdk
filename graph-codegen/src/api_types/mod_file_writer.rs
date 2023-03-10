use bytes::{BufMut, BytesMut};
use from_as::*;
use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult};
use graph_http::iotools::create_dir;
use inflector::Inflector;
use std::collections::HashSet;
use std::fmt::format;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::ptr::write_bytes;

static BASE_MOD_DECLARATION: &str = "request";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub enum ModFile {
    /// Create the mod and use declarations for a mod file.
    Declarations {
        file: String,
        declarations: Vec<String>,
    },
}

impl ModFile {
    pub fn format_declarations(declarations: &Vec<String>) -> BytesMut {
        let mut bytes = BytesMut::new();

        for mod_dec in declarations.iter() {
            bytes.put(format!("mod {mod_dec};\n").as_bytes());
        }

        bytes.put("\n".as_bytes());
        for use_dec in declarations.iter() {
            bytes.put(format!("pub use {use_dec}::*;\n").as_bytes());
        }

        bytes
    }

    pub fn write_bytes(file: &String, bytes: &mut BytesMut) -> GraphResult<()> {
        //println!("{:#?}", file);
        //println!("{:#?}", file.replacen("/mod.rs", "", 1));
        create_dir(file.replacen("/mod.rs", "", 1)).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .create(true)
            .open(file)?;

        file.write_all(bytes.as_mut()).unwrap();
        file.sync_data().map_err(GraphFailure::from)
    }

    pub fn base_mod_declarations() -> BytesMut {
        let mut buf = BytesMut::new();
        buf.put(BASE_MOD_DECLARATION.as_bytes());
        buf
    }

    pub fn base_mod(folder: &str) -> ModFile {
        ModFile::Declarations {
            file: format!("./src/{folder}/mod.rs"),
            declarations: vec![BASE_MOD_DECLARATION.into()],
        }
    }

    pub fn base_mod_from_ri(ri: ResourceIdentity) -> ModFile {
        let file_name = ri.to_string().to_snake_case();
        ModFile::Declarations {
            file: format!("./src/{file_name}/mod.rs"),
            declarations: vec![BASE_MOD_DECLARATION.into()],
        }
    }

    pub fn declare(file: &str, declarations: Vec<&str>) -> ModFile {
        ModFile::Declarations {
            file: format!("./src/{file}/mod.rs"),
            declarations: declarations.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn write(&self) {
        match self {
            ModFile::Declarations { file, declarations } => {
                ModFile::write_bytes(file, &mut ModFile::format_declarations(declarations))
                    .unwrap();
            }
        }
    }
}

#[derive(
    Default, Builder, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromFile, AsFile,
)]
#[builder(setter(into, strip_option), default)]
pub struct ModFileWriter {
    file: PathBuf,
    declarations: HashSet<String>,
}

impl ModFileWriter {
    pub fn new(file: impl AsRef<Path>, declarations: Vec<String>) -> ModFileWriter {
        ModFileWriter {
            file: file.as_ref().to_path_buf(),
            declarations: HashSet::from_iter(declarations.iter().map(|s| s.to_string())),
        }
    }

    pub fn extend(&mut self, declarations_vec: Vec<String>) {
        self.declarations.extend(declarations_vec);
    }

    pub fn format(&self) -> BytesMut {
        let mut bytes = BytesMut::new();

        for mod_dec in self.declarations.iter() {
            bytes.put(format!("mod {mod_dec};\n").as_bytes());
        }

        bytes.put("\n".as_bytes());
        for use_dec in self.declarations.iter() {
            bytes.put(format!("pub use {use_dec}::*;\n").as_bytes());
        }

        bytes
    }

    pub fn write(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.file)
            .unwrap();

        let mut bytes = self.format();
        file.write_all(bytes.as_mut()).unwrap();
        file.sync_data().unwrap();
    }
}
