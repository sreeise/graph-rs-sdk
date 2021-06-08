use examples_common::TestServer;
use graph_rs_sdk::oauth::OAuth;
use warp::Filter;

// First run the example: rocket_example.rs
#[tokio::main]
async fn main() {
    let server = TestServer::serve(
        warp::get()
            .and(warp::path("redirect"))
            .and(warp::query::raw())
            .map(|code: String| {
                println!("{:#?}", code);
                Ok("Successfully logged out! You can close your browser.")
            }),
        ([127, 0, 0, 1], 8001),
    );

    let mut oauth: OAuth = OAuth::new();
    oauth
        .logout_url("https:://localhost:8000/logout")
        .post_logout_redirect_uri("https:://localhost:8000/redirect");
    oauth.v1_logout().unwrap();

    server.await.expect("Failed to await server");
}
