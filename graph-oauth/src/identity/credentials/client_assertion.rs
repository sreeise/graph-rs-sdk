use anyhow::Context;
use base64::Engine;
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Padding;
use openssl::sign::Signer;
use openssl::x509::X509;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Computes the client assertion used in certificate credential authorization flows.
/// The client assertion is computed from the DER encoding of an X509 certificate and it's private key.
///
/// Client assertions are generated using the openssl library for security reasons.
/// You can see an example of how this is done by Microsoft located at
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/msal-net-client-assertions
pub struct ClientAssertion {
    client_id: String,
    tenant_id: Option<String>,
    claims: Option<HashMap<String, String>>,
    extend_claims: bool,
    certificate: X509,
    pkey: PKey<Private>,
    uuid: Uuid,
}

impl ClientAssertion {
    pub fn new<T: AsRef<str>>(client_id: T, certificate: X509, private_key: PKey<Private>) -> Self {
        Self {
            client_id: client_id.as_ref().to_owned(),
            tenant_id: None,
            claims: None,
            extend_claims: true,
            certificate,
            pkey: private_key,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn new_with_tenant<T: AsRef<str>>(
        client_id: T,
        tenant_id: T,
        certificate: X509,
        private_key: PKey<Private>,
    ) -> ClientAssertion {
        Self {
            client_id: client_id.as_ref().to_owned(),
            tenant_id: Some(tenant_id.as_ref().to_owned()),
            claims: None,
            extend_claims: true,
            certificate,
            pkey: private_key,
            uuid: Uuid::new_v4(),
        }
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
    pub fn with_claims(&mut self, claims: HashMap<String, String>, extend_claims: bool) {
        self.claims = Some(claims);
        self.extend_claims = extend_claims;
    }

    /// Hex encoded SHA-1 thumbprint of the X.509 certificate's DER encoding.
    ///
    /// You can verify that the correct certificate has been passed
    /// by comparing the hex encoded thumbprint against the thumbprint given in Azure
    /// Active Directory under Certificates and Secrets for your application or by looking
    /// at the keyCredentials customKeyIdentifier field in your applications manifest.
    pub fn get_thumbprint(&self) -> Result<String, ErrorStack> {
        let digest_bytes = self.certificate.digest(MessageDigest::sha1())?;
        Ok(hex::encode(digest_bytes.as_ref()).to_uppercase())
    }

    /// Base64 Url encoded (No Pad) SHA-1 thumbprint of the X.509 certificate's DER encoding.
    pub fn get_thumbprint_base64(&self) -> Result<String, ErrorStack> {
        let digest_bytes = self.certificate.digest(MessageDigest::sha1())?;
        Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest_bytes))
    }

    /// Get the value used for the jti field in the payload. This field is computed
    /// when constructing the [ClientAssertion] and will be different from any
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

    fn get_header(&self) -> Result<HashMap<String, String>, ErrorStack> {
        let mut header = HashMap::new();
        header.insert("x5t".to_owned(), self.get_thumbprint_base64()?);
        header.insert("alg".to_owned(), "RS256".to_owned());
        header.insert("typ".to_owned(), "JWT".to_owned());
        Ok(header)
    }

    fn get_claims(&self) -> anyhow::Result<HashMap<String, String>> {
        if let Some(claims) = self.claims.as_ref() {
            if !self.extend_claims {
                return Ok(claims.clone());
            }
        }

        let aud = {
            if let Some(tenant_id) = self.tenant_id.as_ref() {
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
        let nbf = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let exp = nbf
            .checked_add(Duration::from_secs(exp))
            .context("Unable to set exp claims field - Reason: Unknown")?;

        let mut claims = HashMap::new();
        claims.insert("aud".to_owned(), aud);
        claims.insert("exp".to_owned(), exp.as_secs().to_string());
        claims.insert("nbf".to_owned(), nbf.as_secs().to_string());
        claims.insert("jti".to_owned(), self.uuid.to_string());
        claims.insert("sub".to_owned(), self.client_id.to_owned());
        claims.insert("iss".to_owned(), self.client_id.to_owned());

        if let Some(internal_claims) = self.claims.as_ref() {
            claims.extend(internal_claims.clone());
        }

        Ok(claims)
    }

    /// JWT Header and Payload in the format header.payload
    fn base64_token(&self) -> anyhow::Result<String> {
        let header = self.get_header()?;
        let header = serde_json::to_string(&header)?;
        let header_base64 =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(header.as_bytes());
        let claims = self.get_claims()?;
        let claims = serde_json::to_string(&claims)?;
        let claims_base64 =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(claims.as_bytes());
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

    /// Get the signed client assertion.
    ///
    /// The signature is a Base64 Url encoded (No Pad) JWT Header and Payload signed with the private key using SHA_256
    /// and RSA padding PKCS1
    pub fn sign(&self) -> anyhow::Result<String> {
        let token = self.base64_token()?;
        let mut signer = Signer::new(MessageDigest::sha256(), &self.pkey)?;
        signer.set_rsa_padding(Padding::PKCS1)?;
        signer.update(token.as_str().as_bytes())?;
        let signature =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(signer.sign_to_vec()?);
        Ok(format!("{token}.{signature}"))
    }
}
