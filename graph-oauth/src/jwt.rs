use crate::auth::OAuthReq;
use crate::oautherror::OAuthError;
use graph_error::GraphFailure;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str::FromStr;

/// Enum for the type of JSON web token (JWT).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum JwtType {
    JWS,
    JWE,
}

impl AsRef<str> for JwtType {
    fn as_ref(&self) -> &str {
        match self {
            JwtType::JWE => "JWE",
            JwtType::JWS => "JWS",
        }
    }
}

impl TryFrom<usize> for JwtType {
    type Error = GraphFailure;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(JwtType::JWS),
            4 => Ok(JwtType::JWE),
            _ => OAuthError::invalid_data("Invalid Key"),
        }
    }
}

impl FromStr for JwtType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "payload" => Ok(JwtType::JWS),
            "ciphertext" => Ok(JwtType::JWE),
            _ => Err(()),
        }
    }
}

/// Claims in a JSON web token (JWT).
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

impl Eq for Claim {}

/// Algorithms used in JSON web tokens (JWT).
/// Does not implement a complete set of Algorithms used in JWTs.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, EnumIter)]
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HS256" => Ok(Algorithm::HS256),
            "HS384" => Ok(Algorithm::HS384),
            "HS512" => Ok(Algorithm::HS512),
            "RS256" => Ok(Algorithm::RS256),
            "RS384" => Ok(Algorithm::RS384),
            "RS512" => Ok(Algorithm::RS512),
            "ES256" => Ok(Algorithm::ES256),
            "ES384" => Ok(Algorithm::ES384),
            "ES512" => Ok(Algorithm::ES512),
            "PS256" => Ok(Algorithm::PS256),
            "PS384" => Ok(Algorithm::PS384),
            _ => Err(()),
        }
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
        self.alg
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct JsonWebToken {
    jwt_type: Option<JwtType>,
    header: Option<Header>,
    payload: Option<Vec<Claim>>,
    signature: Option<String>,
}

impl JsonWebToken {
    pub fn header(&self) -> Option<Header> {
        self.header.clone()
    }

    pub fn claims(&self) -> Option<Vec<Claim>> {
        self.payload.clone()
    }

    pub fn signature(&self) -> Option<&String> {
        self.signature.as_ref()
    }
}

/// TODO(#4): JWT Validation - https://github.com/sreeise/rust-onedrive/issues/4
///
/// JSON web token (JWT) verification for RFC 7619
///
/// The JWT implementation does not implement full JWT verification.
/// The validation here is best effort to follow section 7.2 of RFC 7519 for
/// JWT validation: https://tools.ietf.org/html/rfc7519#section-7.2
///
/// Callers should not rely on this alone to verify JWTs
pub struct JwtParser;

impl JwtParser {
    pub fn parse(input: &str) -> OAuthReq<JsonWebToken> {
        // Step 1.
        if !input.contains('.') {
            return OAuthError::invalid_data("Invalid Key");
        }

        // Step 2.
        let index = input
            .find('.')
            .ok_or_else(|| OAuthError::invalid("Invalid Key"))?;

        // Step 3.
        let header = base64::decode_config(&input[..index], base64::URL_SAFE_NO_PAD)?;
        for byte in header.iter() {
            let b = *byte;
            if b == b'\n' || b == b' ' {
                return OAuthError::invalid_data("Invalid Key");
            }
        }

        // Step 4.
        let utf8_header = std::str::from_utf8(&header)?;

        // Step 5.
        let value = utf8_header.to_owned();
        let jwt_header: Header = serde_json::from_str(&value)?;

        let mut jwt = JsonWebToken {
            header: Some(jwt_header),
            ..Default::default()
        };

        // Step 6
        let count: usize = input.matches('.').count();
        let jwt_type = JwtType::try_from(count)?;

        jwt.jwt_type = Some(jwt_type);

        // Step 7.
        match jwt_type {
            JwtType::JWS => {}
            JwtType::JWE => {}
        }

        // Step 8.
        let mut claims: Vec<Claim> = Vec::new();
        let key_vec: Vec<&str> = input.split('.').collect();
        let payload = key_vec.get(1);

        if let Some(p) = payload {
            let t = base64::decode(&**p)?;
            let v_utf8 = std::str::from_utf8(&t)?;
            let v_owned = v_utf8.to_owned();

            let claims_map: Map<String, Value> = serde_json::from_str(&v_owned)?;

            claims = claims_map
                .iter()
                .map(|(key, value)| Claim {
                    key: key.to_owned(),
                    value: value.to_owned(),
                })
                .collect();
        };

        if let Some(c) = claims.iter().find(|v| v.key == "cty") {
            let cty = c
                .value
                .as_str()
                .ok_or_else(|| OAuthError::invalid("Invalid Key"))?;
            if cty.eq("JWT") {
                return JwtParser::parse(cty);
            }
        } else {
            // Step 9.
        }
        // Step 10.

        jwt.payload = Some(claims);
        Ok(jwt)
    }

    #[allow(dead_code)]
    fn contains_duplicates(&mut self, claims: Vec<Claim>) -> OAuthReq<()> {
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
}
