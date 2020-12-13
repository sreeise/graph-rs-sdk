#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#[macro_use]
extern crate rocket;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate reqwest;
use graph_rs_sdk::oauth::OAuth;
use rocket::http::RawStr;
use rocket_codegen::routes;

fn main() {
    // First run the example: rocket_example.rs
    let mut oauth: OAuth = OAuth::new();
    oauth
        .logout_url("https:://localhost:8000/logout")
        .post_logout_redirect_uri("https:://localhost:8000/redirect");
    oauth.v1_logout().unwrap();

    rocket::ignite().mount("/", routes![redirect]).launch();
}

#[get("/redirect?<lc>")]
fn redirect(lc: &RawStr) -> String {
    println!("{:#?}", lc);
    String::from("Successfully logged out! You can close your browser.")
}
