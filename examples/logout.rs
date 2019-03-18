#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate reqwest;
use rocket::http::RawStr;
use rocket_codegen::routes;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::transform::*;

fn main() {
    // First run the example: rocket_example.rs
    let mut oauth: OAuth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    oauth.v1_logout().unwrap();

    rocket::ignite().mount("/", routes![redirect]).launch();
}

#[get("/redirect?<lc>")]
fn redirect(lc: &RawStr) -> String {
    println!("{:#?}", lc);
    String::from("Successfully logged out! You can close your browser.")
}
