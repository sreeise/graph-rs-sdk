use jsonwebtoken::TokenData;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct JwksKeySet {
    pub keys: HashSet<JwksKey>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct JwksKey {
    pub kid: String,
    #[serde(alias = "n")]
    pub modulus: String,
    #[serde(alias = "e")]
    pub exponent: String,
}

impl JwksKey {
    pub fn new(kid: impl ToString, modulus: impl ToString, exponent: impl ToString) -> JwksKey {
        JwksKey {
            kid: kid.to_string(),
            modulus: modulus.to_string(),
            exponent: exponent.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct JwtHeader {
    pub typ: String,
    pub alg: String,
    pub kid: String,
    pub x5t: Option<String>,
}

impl Display for JwtHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "typ: {}, alg: {}, kid: {}, x5t: {:#?}",
            self.typ, self.alg, self.kid, self.x5t
        )
    }
}

pub type DecodedJwt = TokenData<Claims>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iss: String,
    pub iat: usize,
    pub nbf: usize,
    pub exp: usize,
    pub aio: Option<String>,
    pub c_hash: Option<String>,
    pub cc: Option<String>,
    pub email: Option<String>,
    pub name: Option<String>,
    pub nonce: Option<String>,
    pub oid: Option<String>,
    pub preferred_username: Option<String>,
    pub rh: Option<String>,
    pub sub: Option<String>,
    pub tid: Option<String>,
    pub uti: Option<String>,
    pub ver: Option<String>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}
