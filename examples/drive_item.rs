use rust_onedrive::drive::endpoint::EP;
use rust_onedrive::drive::Drive;
use rust_onedrive::flow::v1::AuthFlow;
use rust_onedrive::process::jsonio::JsonFile;

fn main() {
    /*
    Either use AuthFlow.into_drive() or Drive::new("Access Token")
    */
    let mut drive = Drive::new("YOUR ACCESS TOKEN");
    let drive_item = drive.drive_root_child();
    println!("{:#?}", &drive_item);
    JsonFile::json_file("examples/drive_item.json", &drive_item);
}


pub fn auth_flow_to_drive() {
    let mut auth_flow: AuthFlow = JsonFile::from_file("examples/web_auth_flow.json").unwrap();
    let mut drive = auth_flow.into_drive().unwrap();
}