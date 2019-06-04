pub mod application;
pub mod createdby;
pub mod driveinfo;
pub mod driveitem;
pub mod expandchildren;
pub mod file;
pub mod filesysteminfo;
pub mod folder;
pub mod group;
pub mod hashes;
pub mod image;
pub mod lastmodifiedby;
pub mod owner;
pub mod package;
pub mod parentreference;
pub mod photo;
pub mod quota;
pub mod remoteitem;
pub mod shared;
pub mod sharedby;
pub mod sharepointid;
pub mod specialfolder;
pub mod thumbnail;
pub mod user;
pub mod value;
pub mod video;
pub mod view;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {}
