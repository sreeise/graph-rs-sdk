use crate::oautherror::OAuthError;
use base64;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::str::FromStr;

/// Enum for the type of JSON web token (JWT).
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum JWTType {
    JWS,
    JWE,
}

impl JWTType {
    pub fn type_from(count: usize) -> Option<JWTType> {
        match count {
            2 => Some(JWTType::JWS),
            4 => Some(JWTType::JWE),
            _ => None,
        }
    }

    pub fn type_from_str(mem: &str) -> Option<JWTType> {
        match mem {
            "payload" => Some(JWTType::JWS),
            "ciphertext" => Some(JWTType::JWE),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            JWTType::JWE => "JWE",
            JWTType::JWS => "JWS",
        }
    }
}

/// Claims in a JSON web token (JWT).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    key: String,
    value: Value,
}

impl Claim {
    pub fn new(key: String, value: Value) -> Claim {
        Claim { key, value }
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> Value {
        self.value.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Header {
    typ: Option<String>,
    alg: Algorithm,
}

impl Header {
    pub fn typ(&self) -> Option<String> {
        self.typ.clone()
    }

    pub fn alg(&self) -> Algorithm {
        self.alg.clone()
    }
}

/// Algorithms used in JSON web tokens (JWT).
/// Does not implement a complete set of Algorithms used in JWTs.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
pub enum Algorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512,
    PS256,
    PS384,
}

impl FromStr for Algorithm {
    type Err = OAuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HS256" => Ok(Algorithm::HS256),
            "HS384" => Ok(Algorithm::ES384),
            "HS512" => Ok(Algorithm::HS512),
            "RS256" => Ok(Algorithm::RS256),
            "RS384" => Ok(Algorithm::RS384),
            "RS512" => Ok(Algorithm::RS512),
            "ES256" => Ok(Algorithm::ES256),
            "ES384" => Ok(Algorithm::ES384),
            "ES512" => Ok(Algorithm::ES512),
            "PS256" => Ok(Algorithm::PS256),
            "PS384" => Ok(Algorithm::PS384),
            _ => Err(OAuthError::error_kind(
                ErrorKind::NotFound,
                "Not an Algorithm Type",
            )),
        }
    }
}

/// Payload types for JSON web tokens (JWT).
#[derive(Debug, Default, Clone, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(default)]
pub struct Payload {
    alg: Option<String>,
    enc: Option<String>,
    zip: Option<String>,
    exp: Option<u32>,
    nbf: Option<String>,
    iss: Option<String>,
    aud: Option<String>,
    pm: Option<String>,
    iti: Option<String>,
    typ: Option<String>,
    sub: Option<String>,
    kid: Option<String>,
    iat: Option<u32>,
}

/// Partial JSON web token (JWT) verification for RFC 7619
///
/// The JWT implementation does not implement full JWT verification.
/// The validation here is best effort to follow section 7.2 of RFC 7519 for
/// JWT validation: https://tools.ietf.org/html/rfc7519#section-7.2
///
/// Callers should not rely on this alone to verify JWTs
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct JWT {
    key: String,
    header: Option<Header>,
    enc: Option<String>,
}

impl JWT {
    pub fn new(key: &str) -> JWT {
        JWT {
            key: key.into(),
            header: None,
            enc: None,
        }
    }

    pub fn header(&self) -> Option<Header> {
        self.header.clone()
    }

    pub fn validate(&mut self) -> Result<(), OAuthError> {
        self.contains_period()?;
        let header = self.decode_header()?;
        self.header = Some(header);
        self.message_jwt()?;

        Ok(())
    }

    // Step 1.
    #[allow(dead_code)]
    fn contains_period(&self) -> Result<(), OAuthError> {
        if !self.key.contains('.') {
            return OAuthError::invalid_data("Invalid Key");
        }
        Ok(())
    }

    // Step 2.
    #[allow(dead_code)]
    fn encoded_header(&self) -> Result<&str, OAuthError> {
        let index = match self.key.find('.') {
            Some(t) => t,
            None => return OAuthError::invalid_data("Invalid Key"),
        };

        let header = &self.key[..index];
        Ok(header)
    }

    // Step 3.
    #[allow(dead_code)]
    fn base64_url_decode_header(&self) -> Result<Vec<u8>, OAuthError> {
        let header = self.encoded_header()?;
        let header = base64::decode_config(&header, base64::URL_SAFE_NO_PAD)?;
        Ok(header)
    }

    // Step 4.
    #[allow(dead_code)]
    fn utf8_encode(&self) -> Result<String, OAuthError> {
        let header = self.base64_url_decode_header()?;
        let utf8_str = std::str::from_utf8(&header)?;
        Ok(utf8_str.to_string())
    }

    // Step 5.
    #[allow(dead_code)]
    fn header_as_json(&self) -> Result<Header, OAuthError> {
        let utf8_encoded = self.utf8_encode()?;
        let jwt_header: Header = serde_json::from_str(&utf8_encoded)?;
        Ok(jwt_header)
    }

    // Step 6.
    pub fn type_of(&self) -> Option<JWTType> {
        let count = self.key.matches('.').count();
        JWTType::type_from(count)
    }

    // Steps 7.

    // Step 8.
    fn message_jwt(&mut self) -> Result<(), OAuthError> {
        let v_claim = self.decode_payload()?;
        let claims = v_claim.iter().find(|v| v.key == "cty");
        if let Some(c) = claims {
            if let Some(value) = c.value.as_str() {
                let mut jwt = JWT::new(value);
                jwt.validate()?
            } else {
                // Step 9.
            }
        }
        Ok(())
    }

    pub fn decode_header(&self) -> Result<Header, OAuthError> {
        // Step 2.
        let index = match self.key.find('.') {
            Some(t) => t,
            None => return OAuthError::invalid_data("Invalid Key"),
        };

        // Step 3.
        let header = &self.key[..index];
        let header = base64::decode_config(&header, base64::URL_SAFE_NO_PAD)?;

        // Step 4.
        let utf8_header = std::str::from_utf8(&header)?;

        // Step 5.
        let value = utf8_header.to_owned();
        let jwt_header: Header = serde_json::from_str(&value)?;

        Ok(jwt_header)
    }

    fn decode_payload(&mut self) -> Result<Vec<Claim>, OAuthError> {
        let mut vec: Vec<Claim> = Vec::new();
        let map = self.decode_claims()?;

        for (i, t) in &map {
            vec.push(Claim {
                key: i.to_owned(),
                value: t.to_owned(),
            });
        }
        Ok(vec)
    }

    // Step 10.
    pub fn decode_claims(&mut self) -> std::result::Result<Map<String, Value>, OAuthError> {
        let key_vec: Vec<&str> = self.key.split('.').collect();
        let payload = key_vec.get(1);

        if let Some(p) = payload {
            let t = base64::decode(&**p)?;
            let v_utf8 = std::str::from_utf8(&t)?;
            let v_owned = v_utf8.to_owned();

            let claims: Map<String, Value> = serde_json::from_str(&v_owned)?;
            return Ok(claims);
        };

        Err(OAuthError::error_kind(ErrorKind::InvalidData, "Invalid"))
    }

    #[allow(dead_code)]
    fn has_duplicates(&mut self, claims: Vec<Claim>) -> Result<(), OAuthError> {
        // https://tools.ietf.org/html/rfc7515#section-5.2
        // Step 4  this restriction includes that the same
        // Header Parameter name also MUST NOT occur in distinct JSON object
        // values that together comprise the JOSE Header.
        let mut set = HashMap::new();
        for claim in claims.iter() {
            if set.contains_key(&claim.key) {
                return OAuthError::invalid_data("Duplicate claims");
            }
            set.insert(&claim.key, &claim.value);
        }
        Ok(())
    }

    #[allow(dead_code, unused_variables)]
    pub fn verify_with_claims(&self, claims: Vec<Claim>) -> Result<(), OAuthError> {
        unimplemented!()
    }
}
