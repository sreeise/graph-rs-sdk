use crate::scopes::scope::Scope;

pub enum FileScope {
    Read,
    ReadAll,
    ReadWrite,
    ReadWriteAll,
    ReadWriteAppFolder,
    ReadSelected,
    ReadWriteSelected,
}

impl AsRef<str> for FileScope {
    fn as_ref(&self) -> &str {
        match self {
            FileScope::Read => "Files.Read",
            FileScope::ReadAll => "Files.Read.All",
            FileScope::ReadWrite => "Files.ReadWrite",
            FileScope::ReadWriteAll => "Files.ReadWrite.All",
            FileScope::ReadWriteAppFolder => "Files.ReadWrite.AppFolder",
            FileScope::ReadSelected => "Files.Read.Selected",
            FileScope::ReadWriteSelected => "Files.ReadWrite.Selected",
        }
    }
}

impl Scope for FileScope {}

impl ToString for FileScope {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
