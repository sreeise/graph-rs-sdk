use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Read;
use chrono::{DateTime, Utc};
use openssl::hash::{DigestBytes, MessageDigest};
use openssl::pkey::{PKey, Private};
use openssl::x509;
use openssl::x509::X509;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use base64::Engine;
use openssl::error::ErrorStack;
use openssl::rsa::{Padding, Rsa};
use openssl::sign::Signer;

pub struct SignedClientAssertion {
    client_id: String,
    tenant_id: Option<String>,
    certificate: X509,
    pkey: PKey<Private>
}

impl SignedClientAssertion {
    pub fn new<T: AsRef<str>>(client_id: T, certificate: X509, private_key: PKey<Private>) -> Self {
        Self {
            client_id: client_id.as_ref().to_owned(),
            tenant_id: None,
            certificate,
            pkey: private_key
        }
    }

    pub fn new_with_tenant<T: AsRef<str>>(client_id: T, tenant_id: T, certificate: X509, private_key: PKey<Private>) -> SignedClientAssertion {
        Self {
            client_id: client_id.as_ref().to_owned(),
            tenant_id: Some(tenant_id.as_ref().to_owned()),
            certificate,
            pkey: private_key
        }
    }

    /// Base64url-encoded SHA-1 thumbprint of the X.509 certificate's DER encoding.
    pub fn get_thumbprint(&self) -> Result<String, ErrorStack> {
        let hash = self.certificate.digest(MessageDigest::sha1())?;
        let hash_hex = hex::encode(hash.as_ref());
        println!("{hash_hex:#?}");

        //println!("DigestBytes: {:#?}", std::str::from_utf8(hash.as_ref()));
        Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hash_hex))
    }

    pub fn get_header(&self) -> Result<HashMap<String, String>, ErrorStack> {
        let mut header = HashMap::new();
        header.insert("x5t".to_owned(), self.get_thumbprint()?);
        header.insert("alg".to_owned(), "RS256".to_owned());
        header.insert("typ".to_owned(), "JWT".to_owned());
        Ok(header)
    }

    pub fn get_claims(&self) -> HashMap<String, String> {
        let aud = {
          if let Some(tenant_id) = self.tenant_id.as_ref() {
              format!("https://login.microsoftonline.com/{}/v2.0", tenant_id)
          }  else {
              "https://login.microsoftonline.com/common/v2.0".to_owned()
          }
        };

        // 10 minutes until expiration.
        let exp = 60 * 10;
        let nbf = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let exp = nbf.checked_add(Duration::from_secs(exp)).unwrap();

        let mut claims = HashMap::new();
        claims.insert("aud".to_owned(), aud);
        claims.insert("exp".to_owned(), exp.as_secs().to_string());
        claims.insert("nbf".to_owned(), nbf.as_secs().to_string());
        claims.insert("sub".to_owned(), self.client_id.to_owned());
        claims.insert("iss".to_owned(), self.client_id.to_owned());
        claims
    }

    pub fn get_signed_client_assertion(&self) -> Result<String, ErrorStack> {
        let header = self.get_header()?;
        let header_base64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(serde_json::to_string(&header).unwrap());
        let claims = self.get_claims();
        let claims_base64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(serde_json::to_string(&claims).unwrap());
        let token = format!("{}.{}", header_base64, claims_base64);

        let mut signer = Signer::new(MessageDigest::sha256(), &self.pkey)?;
        signer.set_rsa_padding(Padding::PKCS1)?;
        signer.update(token.as_str().as_bytes())?;
        let signature = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signer.sign_to_vec()?);
        let signed_client_assertion = format!("{token}.{signature}");

        Ok(signed_client_assertion)
    }
}
