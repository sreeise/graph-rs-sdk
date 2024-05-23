use std::collections::HashMap;

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use graph_error::{IdentityResult, AF};
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::pkcs12::{ParsedPkcs12_2, Pkcs12};
use openssl::pkey::{PKey, Private};
use openssl::rsa::Padding;
use openssl::sign::Signer;
use openssl::x509::{X509Ref, X509};
use time::OffsetDateTime;
use uuid::Uuid;

fn encode_cert(cert: &X509) -> IdentityResult<String> {
    Ok(format!(
        "\"{}\"",
        URL_SAFE_NO_PAD.encode(cert.to_pem().map_err(|err| AF::x509(err.to_string()))?)
    ))
}

fn encode_cert_ref(cert: &X509Ref) -> IdentityResult<String> {
    Ok(format!(
        "\"{}\"",
        URL_SAFE_NO_PAD.encode(cert.to_pem().map_err(|err| AF::x509(err.to_string()))?)
    ))
}

#[allow(unused)]
fn thumbprint(cert: &X509) -> IdentityResult<String> {
    let digest_bytes = cert
        .digest(MessageDigest::sha1())
        .map_err(|err| AF::x509(err.to_string()))?;
    Ok(URL_SAFE_NO_PAD.encode(digest_bytes))
}

/// Computes the client assertion used in certificate credential authorization flows.
/// The client assertion is computed from the DER encoding of an X509 certificate and it's private key.
///
/// Client assertions are generated using the openssl library for security reasons.
/// You can see an example of how this is done by Microsoft located at
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/msal-net-client-assertions
pub struct X509Certificate {
    client_id: String,
    tenant_id: Option<String>,
    claims: Option<HashMap<String, String>>,
    extend_claims: bool,
    certificate: X509,
    pkey: PKey<Private>,
    certificate_chain: bool,
    parsed_pkcs12: Option<ParsedPkcs12_2>,
    uuid: Uuid,
}

impl X509Certificate {
    pub fn new(client_id: impl AsRef<str>, certificate: X509, private_key: PKey<Private>) -> Self {
        Self {
            client_id: client_id.as_ref().to_owned(),
            tenant_id: None,
            claims: None,
            extend_claims: true,
            certificate,
            certificate_chain: false,
            pkey: private_key,
            parsed_pkcs12: None,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn new_with_tenant(
        client_id: impl AsRef<str>,
        tenant_id: impl AsRef<str>,
        certificate: X509,
        private_key: PKey<Private>,
    ) -> Self {
        Self {
            client_id: client_id.as_ref().to_owned(),
            tenant_id: Some(tenant_id.as_ref().to_owned()),
            claims: None,
            extend_claims: true,
            certificate,
            certificate_chain: false,
            pkey: private_key,
            parsed_pkcs12: None,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn new_from_pass(
        client_id: impl AsRef<str>,
        pass: impl AsRef<str>,
        certificate: X509,
    ) -> IdentityResult<Self> {
        let der = encode_cert(&certificate)?;
        let parsed_pkcs12 = Pkcs12::from_der(
            &URL_SAFE_NO_PAD
                .decode(der)
                .map_err(|err| AF::x509(err.to_string()))?,
        )
        .map_err(|err| AF::x509(err.to_string()))?
        .parse2(pass.as_ref())
        .map_err(|err| AF::x509(err.to_string()))?;

        let _ = parsed_pkcs12.cert.as_ref().ok_or(AF::x509(
            "No certificate found after parsing Pkcs12 using pass",
        ))?;

        let private_key = parsed_pkcs12.pkey.as_ref().ok_or(AF::x509(
            "No private key found after parsing Pkcs12 using pass",
        ))?;

        Ok(Self {
            client_id: client_id.as_ref().to_owned(),
            tenant_id: None,
            claims: None,
            extend_claims: true,
            certificate,
            certificate_chain: true,
            pkey: private_key.clone(),
            parsed_pkcs12: Some(parsed_pkcs12),
            uuid: Uuid::new_v4(),
        })
    }

    pub fn new_from_pass_with_tenant(
        client_id: impl AsRef<str>,
        tenant_id: impl AsRef<str>,
        pass: impl AsRef<str>,
        certificate: X509,
    ) -> IdentityResult<Self> {
        let der = encode_cert(&certificate)?;
        let parsed_pkcs12 = Pkcs12::from_der(
            &URL_SAFE_NO_PAD
                .decode(der)
                .map_err(|err| AF::x509(err.to_string()))?,
        )
        .map_err(|err| AF::x509(err.to_string()))?
        .parse2(pass.as_ref())
        .map_err(|err| AF::x509(err.to_string()))?;

        let _ = parsed_pkcs12.cert.as_ref().ok_or(AF::x509(
            "No certificate found after parsing Pkcs12 using pass",
        ))?;

        let private_key = parsed_pkcs12.pkey.as_ref().ok_or(AF::x509(
            "No private key found after parsing Pkcs12 using pass",
        ))?;

        Ok(Self {
            client_id: client_id.as_ref().to_owned(),
            tenant_id: Some(tenant_id.as_ref().to_owned()),
            claims: None,
            extend_claims: true,
            certificate,
            certificate_chain: true,
            pkey: private_key.clone(),
            parsed_pkcs12: Some(parsed_pkcs12),
            uuid: Uuid::new_v4(),
        })
    }

    /// Provide your own set of claims in the payload of the JWT.
    ///
    /// Replace the claims that would be generated for the client assertion.
    /// This replaces the following payload fields: aud, exp, nbf, jti, sub, and iss.
    /// Only the claims given are passed for the payload of the JWT used in the client assertion.
    pub fn replace_claims(&mut self, claims: HashMap<String, String>) {
        self.claims = Some(claims);
        self.extend_claims = false;
    }

    /// Provide your own set of claims in the payload of the JWT.
    ///
    /// Set extend_claims to false in order to replace the claims that would be generated
    /// for the client assertion. This replaces the following payload fields: aud, exp, nbf, jti,
    /// sub, and iss. This ensures that only the claims given are passed for the payload of the JWT
    /// used in the client assertion.
    ///
    /// If extend claims is true, the claims provided are in addition
    /// to those claims mentioned above and do not replace them, however, any claim provided
    /// with the same fields above will replace those that are generated.
    pub fn extend_claims(&mut self, claims: HashMap<String, String>) {
        match self.claims.as_mut() {
            Some(c) => c.extend(claims),
            None => self.claims = Some(claims),
        }

        self.extend_claims = true;
    }

    /// Hex encoded SHA-1 thumbprint of the X.509 certificate's DER encoding.
    ///
    /// You can verify that the correct certificate has been passed
    /// by comparing the hex encoded thumbprint against the thumbprint given in Azure
    /// Active Directory under Certificates and Secrets for your application or by looking
    /// at the keyCredentials customKeyIdentifier field in your applications manifest.
    pub fn get_hex_thumbprint(&self) -> Result<String, ErrorStack> {
        let digest_bytes = self.certificate.digest(MessageDigest::sha1())?;
        Ok(hex::encode(digest_bytes.as_ref()).to_uppercase())
    }

    /// Base64 Url encoded (No Pad) SHA-1 thumbprint of the X.509 certificate's DER encoding.
    pub fn get_thumbprint(&self) -> IdentityResult<String> {
        let digest_bytes = self
            .certificate
            .digest(MessageDigest::sha1())
            .map_err(|err| AF::x509(err.to_string()))?;
        Ok(URL_SAFE_NO_PAD.encode(digest_bytes))
    }

    /// Get the value used for the jti field in the payload. This field is computed
    /// when constructing the [X509Certificate] and will be different from any
    /// custom claims provided.
    ///
    /// The "jti" (JWT ID) claim provides a unique identifier for the JWT.
    /// The identifier value MUST be assigned in a manner that ensures that there is
    /// a negligible probability that the same value will be accidentally assigned to
    /// a different data object; if the application uses multiple issuers, collisions
    /// MUST be prevented among values produced by different issuers as well.
    pub fn get_uuid(&self) -> &Uuid {
        &self.uuid
    }

    /// Set the UUID for the jti field of the claims/payload of the jwt.
    pub fn set_uuid(&mut self, value: Uuid) {
        self.uuid = value;
    }

    fn x5c(&self) -> IdentityResult<String> {
        let parsed_pkcs12 = self.parsed_pkcs12.as_ref().ok_or(AF::x509(
            "No certificate found after parsing Pkcs12 using pass",
        ))?;

        let certificate = parsed_pkcs12.cert.as_ref().ok_or(AF::x509(
            "No certificate found after parsing Pkcs12 using pass",
        ))?;

        let sig = encode_cert(certificate)?;

        if let Some(stack) = parsed_pkcs12.ca.as_ref() {
            let chain = stack
                .into_iter()
                .map(encode_cert_ref)
                .collect::<IdentityResult<Vec<String>>>()
                .map_err(|err| {
                    AF::x509(format!(
                        "Unable to encode certificates in certificate chain - error {err}"
                    ))
                })?
                .join(",");

            Ok(format! {"{},{}", sig, chain})
        } else {
            Ok(sig)
        }
    }

    fn get_header(&self) -> IdentityResult<HashMap<String, String>> {
        let mut header = HashMap::new();
        header.insert("x5t".to_owned(), self.get_thumbprint()?);
        header.insert("alg".to_owned(), "RS256".to_owned());
        header.insert("typ".to_owned(), "JWT".to_owned());

        if self.certificate_chain && self.parsed_pkcs12.is_some() {
            let x5c = self.x5c()?;
            header.insert("x5c".to_owned(), x5c);
        }

        Ok(header)
    }

    fn get_claims(&self, tenant_id: Option<String>) -> IdentityResult<HashMap<String, String>> {
        if let Some(claims) = self.claims.as_ref() {
            if !self.extend_claims {
                return Ok(claims.clone());
            }
        }

        let aud = {
            if let Some(tenant_id) = tenant_id.as_ref() {
                format!(
                    "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                    tenant_id
                )
            } else {
                "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_owned()
            }
        };

        // 10 minutes until expiration.
        let exp = 60 * 10;
        let nbf = OffsetDateTime::now_utc().unix_timestamp();
        let exp = nbf + exp;

        let mut claims = HashMap::new();
        claims.insert("aud".to_owned(), aud);
        claims.insert("exp".to_owned(), exp.to_string());
        claims.insert("nbf".to_owned(), nbf.to_string());
        claims.insert("jti".to_owned(), self.uuid.to_string());
        claims.insert("sub".to_owned(), self.client_id.to_owned());
        claims.insert("iss".to_owned(), self.client_id.to_owned());

        if let Some(internal_claims) = self.claims.as_ref() {
            claims.extend(internal_claims.clone());
        }

        Ok(claims)
    }

    /// JWT Header and Payload in the format header.payload
    fn base64_token(&self, tenant_id: Option<String>) -> IdentityResult<String> {
        let header = self.get_header()?;
        let header = serde_json::to_string(&header)?;
        let header_base64 = URL_SAFE_NO_PAD.encode(header.as_bytes());

        let claims = self.get_claims(tenant_id)?;
        let claims = serde_json::to_string(&claims)?;
        let claims_base64 = URL_SAFE_NO_PAD.encode(claims.as_bytes());

        Ok(format!("{}.{}", header_base64, claims_base64))
    }

    /*
           Altogether the general flow is as follows:

           let header = self.get_header()?;
           let header = serde_json::to_string(&header).unwrap();
           let header_base64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(header.as_bytes());
           let claims = self.get_claims();
           let claims = serde_json::to_string(&claims).unwrap();
           let claims_base64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(claims.as_bytes());
           let token = format!("{}.{}", header_base64, claims_base64);

           let mut signer = Signer::new(MessageDigest::sha256(), &self.pkey)?;
           signer.set_rsa_padding(Padding::PKCS1)?;
           signer.update(token.as_str().as_bytes())?;
           let signature = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signer.sign_to_vec()?);
           let signed_client_assertion = format!("{token}.{signature}");
           Ok(signed_client_assertion)
    */

    pub fn sign(&self) -> IdentityResult<String> {
        let token = self.base64_token(self.tenant_id.clone())?;

        let mut signer = Signer::new(MessageDigest::sha256(), &self.pkey)
            .map_err(|err| AF::x509(err.to_string()))?;
        signer
            .set_rsa_padding(Padding::PKCS1)
            .map_err(|err| AF::x509(err.to_string()))?;
        signer
            .update(token.as_bytes())
            .map_err(|err| AF::x509(err.to_string()))?;
        let signature = URL_SAFE_NO_PAD.encode(
            signer
                .sign_to_vec()
                .map_err(|err| AF::x509(err.to_string()))?,
        );

        Ok(format!("{token}.{signature}"))
    }

    /// Get the signed client assertion.
    ///
    /// The signature is a Base64 Url encoded (No Pad) JWT Header and Payload signed with the private key using SHA_256
    /// and RSA padding PKCS1
    pub fn sign_with_tenant(&self, tenant_id: Option<String>) -> IdentityResult<String> {
        let token = self.base64_token(tenant_id)?;

        let mut signer = Signer::new(MessageDigest::sha256(), &self.pkey)
            .map_err(|err| AF::x509(err.to_string()))?;
        signer
            .set_rsa_padding(Padding::PKCS1)
            .map_err(|err| AF::x509(err.to_string()))?;
        signer
            .update(token.as_bytes())
            .map_err(|err| AF::x509(err.to_string()))?;
        let signature = URL_SAFE_NO_PAD.encode(
            signer
                .sign_to_vec()
                .map_err(|err| AF::x509(err.to_string()))?,
        );

        Ok(format!("{token}.{signature}"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn claims() {
        let cert_bytes = include_bytes!("test/cert.pem");
        let private_key_bytes = include_bytes!("test/key.pem");

        let cert = X509::from_pem(cert_bytes).unwrap();
        let private_key = PKey::private_key_from_pem(private_key_bytes).unwrap();

        let mut certificate = X509Certificate::new("client_id", cert, private_key);
        assert!(certificate.claims.is_none());

        let mut claims = HashMap::new();
        claims.insert("c".to_string(), "fake claim".to_string());
        certificate.extend_claims(claims);

        let extended_claims = certificate.get_claims(None).unwrap();
        assert!(extended_claims.contains_key("iss"));
        assert!(extended_claims.contains_key("sub"));
        assert_eq!(
            extended_claims.get("aud").unwrap().as_str(),
            "https://login.microsoftonline.com/common/oauth2/v2.0/token"
        );
        assert_eq!(extended_claims.get("c").unwrap().as_str(), "fake claim");
    }

    #[test]
    pub fn sign() {
        let cert_bytes = include_bytes!("test/cert.pem");
        let private_key_bytes = include_bytes!("test/key.pem");

        let cert = X509::from_pem(cert_bytes).unwrap();
        let private_key = PKey::private_key_from_pem(private_key_bytes).unwrap();

        let certificate = X509Certificate::new("client_id", cert, private_key);
        assert!(certificate.sign_with_tenant(None).is_ok());
    }
}
