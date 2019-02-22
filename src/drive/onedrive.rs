use crate::drive::base::driveinfo::DriveInfo;
use crate::drive::endpoint::EP;
use crate::drive::Drive;
use crate::flow::accesstoken::AccessToken;
use crate::process::jsonio::JsonFile;
use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OneDrive {
    drive_info: DriveInfo,
    root_path: String,
    access_token: String,
}

impl OneDrive {
    pub fn new(root_path: &str, access_token: String) -> Drive {
        if !Path::new(&root_path).exists() {
            fs::create_dir_all(&root_path);
        }

        let mut drive = Drive::new(&access_token);

        let base_drive_info = drive.drive();
        let drive_info = base_drive_info.item().unwrap();
        let drive_id = &drive_info.id().unwrap()[..12];

        let one_drive = OneDrive {
            drive_info,
            root_path: String::from(root_path),
            access_token,
        };

        let account_vec = vec![root_path, "/", drive_id, ".json"];

        let path = account_vec.join("");

        JsonFile::json_file(&path, &one_drive).unwrap();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();
        file.sync_data();

        let mut perms = fs::metadata(&path).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&path, perms).unwrap();

        drive
    }
}
