use rust_onedrive::drive::baseitem::BaseItem;
use rust_onedrive::drive::driveitem::DriveInfo;
use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::endpoint::DriveEndPoint;
use rust_onedrive::drive::endpoint::EP;
use rust_onedrive::drive::Drive;
use rust_onedrive::flow::v1::AuthFlow;
use rust_onedrive::process::jsonio::JsonFile;

fn main() {
    /*
    Either use AuthFlow.into_drive() or Drive::new("Access Token")

    See native_client.rs and web_client.rs for getting access_token/refresh_tokens and
    using serde_json to save AuthFlow to a file.
    */
    let mut drive = Drive::new("YOUR ACCESS TOKEN");
    let drive_item = drive.drive_root_child();
    println!("{:#?}", &drive_item);
    JsonFile::json_file("examples/drive_item.json", &drive_item).unwrap();
}

pub fn auth_flow_to_drive() {
    // See native_client.rs and web_client.rs for getting access_token/refresh_tokens and
    // using serde_json to save AuthFlow to a file.
    let mut auth_flow: AuthFlow = JsonFile::from_file("examples/auth_flow.json").unwrap();
    let drive = auth_flow.into_drive().unwrap();
    println!("{:#?}", &drive);
}

pub fn base_item() {
    let mut auth_flow: AuthFlow = JsonFile::from_file("examples/auth_flow.json").unwrap();
    let mut drive = auth_flow.into_drive().unwrap();
    // The base item holds one of DriveInfo, DriveItem, or Value.
    // where
    // DriveInfo: A top level drive and information for that drive such as id.
    // DriveItem: Houses the top level of a drive item that includes: Vec<Value>
    // Value: The specific information for a DriveItem that may include
    // files/folders, users, remote items, etc.
    let base_item = drive.drive_me();
    if base_item.is_some() {
        println!("{:#?}", &base_item); // Option<DriveItem<DriveInfo>>
    } else {
        println!("{:#?}", &base_item.error); // Option<DriveError>
    }
}

pub fn base_item_by_endpoint() {
    let mut auth_flow: AuthFlow = JsonFile::from_file("examples/auth_flow.json").unwrap();
    let mut drive = auth_flow.into_drive().unwrap();
    // This is the same as requesting drive.drive(), except this requires explicitly stating
    // what will be returned and the endpoint to request. This is more of a manual approach
    // and can result in an error if the caller specifies the wrong type in BaseItem
    let base_item: BaseItem<DriveInfo> = drive.base_item(DriveEndPoint::Drive);
    if base_item.is_none() {
        let error = base_item.error; // Option<DriveError>
        println!("{:#?}", error);
    } else {
        let drive_info = base_item.item().unwrap(); // DriveInfo
        println!("{:#?}", drive_info);
    }
}

pub fn base_item_by_url() {
    let mut auth_flow: AuthFlow = JsonFile::from_file("examples/auth_flow.json").unwrap();
    let mut drive = auth_flow.into_drive().unwrap();
    // This is the same as requesting drive.drive(), except this requires explicitly stating
    // what will be returned and the endpoint to request. This is more of a manual approach
    // and can result in an error if the caller specifies the wrong type in BaseItem
    let base_item: BaseItem<DriveItem> =
        drive.base_item_from_url("https://graph.microsoft.com/v1.0/drive/root/children");
    if base_item.is_none() {
        let error = base_item.error; // Option<DriveError>
        println!("{:#?}", error);
    } else {
        let drive_info = base_item.item().unwrap();
        println!("{:#?}", drive_info);
    }
}

pub fn download_urls() {
    let mut auth_flow: AuthFlow = JsonFile::from_file("examples/auth_flow.json").unwrap();
    let mut drive = auth_flow.into_drive().unwrap();
    let base_item: BaseItem<DriveItem> =
        drive.base_item_from_url("https://graph.microsoft.com/v1.0/drive/root/children");

    if base_item.is_none() {
        let error = base_item.error; // Option<DriveError>
        println!("{:#?}", error);
    } else {
        let drive_info = base_item.item().unwrap(); // DriveItem
        let value = drive_info.value().unwrap(); // Vec<Value>
        let mut download_urls = Vec::new();

        // Iterate through Vec<Value>, and get the download urls.
        for val in value.iter() {
            let url = val.microsoft_graph_download_url().clone(); // Option<String>

            if url.is_some() {
                download_urls.push(url.unwrap());
            }
        }

        println!("{:#?}", &download_urls);
    }
}
