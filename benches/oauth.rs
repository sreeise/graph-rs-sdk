#[macro_use]
extern crate criterion;

use criterion::black_box;
use criterion::Criterion;
use drive_test_tools::common::TestTools;
use graph_oauth::oauth::OAuth;
use rust_onedrive::drive::client::Graph;

fn oauth_setters(oauth: &mut OAuth) {
    oauth
        .client_id("client_id")
        .client_secret("client_secret")
        .authorize_url("https://example.com/authorize?")
        .access_token_url("https://example.com/token?")
        .refresh_token_url("https://example.com/token?")
        .redirect_uri("https://example.com/redirect?")
        .access_code("ADSLFJL4L3")
        .response_mode("response_mode")
        .response_type("response_type")
        .state("state")
        .grant_type("grant_type")
        .nonce("nonce")
        .prompt("login")
        .session_state("session_state")
        .client_assertion("client_assertion")
        .client_assertion_type("client_assertion_type")
        .code_verifier("code_verifier")
        .login_hint("login_hint")
        .domain_hint("domain_hint")
        .resource("resource")
        .logout_url("https://example.com/logout?")
        .post_logout_redirect_uri("https://example.com/redirect?");
}

fn oauth_extend_scopes(oauth: &mut OAuth, scopes: &[String]) {
    oauth.extend_scopes(scopes);
}

fn oauth_setter_bench(c: &mut Criterion) {
    c.bench_function("oauth setters", |b| {
        b.iter(|| oauth_setters(black_box(&mut OAuth::new())))
    });
}

fn oauth_extend_scopes_bench(c: &mut Criterion) {
    let vec = TestTools::random_strings(20, 20);
    c.bench_function("oauth extend scopes", move |b| {
        b.iter(|| oauth_extend_scopes(black_box(&mut OAuth::new()), black_box(&vec)))
    });
}

fn bench_get_item(client: Graph, s: String) {
    client
        .v1()
        .me()
        .drive()
        .get_item()
        .by_id("b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
    client.url_mut(|url| {
        assert_eq!(url.to_string(), s);
    })
}

fn graph_get_item_bench(c: &mut Criterion) {
    c.bench_function("graph get item", move |b| {
        b.iter(|| bench_get_item(black_box(Graph::from("")), black_box("https://graph.microsoft.com/v1.0/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI".to_string())));
    });
}

criterion_group!(
    benches,
    oauth_setter_bench,
    oauth_extend_scopes_bench,
    graph_get_item_bench,
);
criterion_main!(benches);
