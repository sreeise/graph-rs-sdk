use rust_onedrive::prelude::*;

// Use the AsMut<DriveUrl> trait impl to change the path
// and query for a drive request.

fn main() {
    let drive = Drive::new("ACCESS_TOKEN");
    // Or .post(), .put(), .patch(), and .delete()
    let mut req = drive.v1().get();
    req.as_mut().extend_path(&["drive"]);
    let drive_info: serde_json::Value = req.send().unwrap();
    println!("{:#?}", drive_info);
}
