use graph_rs_sdk::oauth::OAuth;

fn main() {
    // First run the example: rocket_example.rs
    let mut oauth: OAuth = OAuth::new();
    oauth
        .logout_url("https:://localhost:8000/logout")
        .post_logout_redirect_uri("https:://localhost:8000/redirect");
    oauth.v1_logout().unwrap();
}
