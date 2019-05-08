use rust_onedrive::oauth::EncodeBuilder;

fn main() {
    let mut encode_builder = EncodeBuilder::new();
    encode_builder
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .redirect_uri("http://localhost:8000/redirect")
        .response_type("code")
        .response_mode("query")
        .client_id("fd32323-2324323-3443")
        .client_secret("jlk3230987a")
        .state("1234")
        .scope("Files.Read")
        .scope("Files.ReadWrite")
        .scope("Files.Read.All")
        .scope("Files.ReadWrite.All")
        .scope("wl.offline_access");
    println!("{:#?}", encode_builder.build());
}
