use graph_rs_sdk::identity::{
    AuthorizationCodeCertificateCredential, ConfidentialClientApplication, PKey, X509Certificate,
    X509,
};
use graph_rs_sdk::GraphClient;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use url::Url;

pub fn x509_certificate(
    client_id: &str,
    tenant: &str,
    public_key_path: impl AsRef<Path>,
    private_key_path: impl AsRef<Path>,
) -> anyhow::Result<X509Certificate> {
    // Use include_bytes!(file_path) if the files are local
    let mut cert_file = File::open(public_key_path)?;
    let mut certificate: Vec<u8> = Vec::new();
    cert_file.read_to_end(&mut certificate)?;

    let mut private_key_file = File::open(private_key_path)?;
    let mut private_key: Vec<u8> = Vec::new();
    private_key_file.read_to_end(&mut private_key)?;

    let cert = X509::from_pem(certificate.as_slice())?;
    let pkey = PKey::private_key_from_pem(private_key.as_slice())?;
    Ok(X509Certificate::new_with_tenant(
        client_id, tenant, cert, pkey,
    ))
}

fn build_confidential_client(
    authorization_code: &str,
    client_id: &str,
    tenant: &str,
    scope: Vec<&str>,
    redirect_uri: Url,
    x509certificate: X509Certificate,
) -> anyhow::Result<GraphClient> {
    // ConfidentialClientApplication<AuthorizationCodeCertificateCredential>
    let confidential_client = ConfidentialClientApplication::builder(client_id)
        .with_auth_code_x509_certificate(authorization_code, &x509certificate)?
        .with_tenant(tenant)
        .with_scope(scope)
        .with_redirect_uri(redirect_uri)
        .build();

    Ok(GraphClient::from(&confidential_client))
}
