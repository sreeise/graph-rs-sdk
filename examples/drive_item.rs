use rust_onedrive::drive::Drive;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::drive::EP;
use rust_onedrive::transform::*;
use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::Item;

fn main() {
    let oauth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive = Drive::from(oauth);

    // You can pick a function below to query common OneDrive resources.
    // For more common OneDrive API queries see the EP trait.
    // In addition, there is standard get/post methods. There is also patch and put methods
    // but they have not been thoroughly tested.

    // This will run all the API requests below.
    drive_root(&mut drive);
    drive_root_children(&mut drive);
    special_docs(&mut drive);
    special_docs_child(&mut drive);
    use_get(&mut drive);
}

fn drive_root(drive: &mut Drive) {
    let drive_item: DriveItem = drive.drive_root().unwrap();
    println!("{:#?}", drive_item);
}

fn drive_root_children(drive: &mut Drive) {
    let drive_item: DriveItem = drive.drive_root_child().unwrap();
    println!("{:#?}", drive_item);
}

fn special_docs(drive: &mut Drive) {
    let drive_item: DriveItem = drive.special_documents().unwrap();
    println!("{:#?}", drive_item);
}

fn special_docs_child(drive: &mut Drive) {
    let drive_item: DriveItem = drive.special_documents_child().unwrap();
    println!("{:#?}", drive_item);
}

fn use_get(drive: &mut Drive) {
    // Using the REST methods requires specifying the type.
    // You could also use Serde's Value here as well.
    let drive_item: DriveItem = drive.get("https://graph.microsoft.com/v1.0/drive/root/children").unwrap();
    println!("{:#?}", drive_item);
}
