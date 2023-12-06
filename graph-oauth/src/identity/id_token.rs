//use crate::jwt::{JsonWebToken, JwtParser};
use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};

use base64::{DecodeError, Engine};
use jsonwebtoken::errors as JwtErrors;
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation};
use std::str::FromStr;
use url::form_urlencoded::parse;

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iss: String,
    pub iat: usize,
    pub nbf: usize,
    pub exp: usize,
    pub aio: String,
    pub c_hash: String,
    pub cc: String,
    pub email: String,
    pub name: String,
    pub nonce: String,
    pub oid: String,
    pub preferred_username: String,
    pub rh: String,
    pub sub: String,
    pub tid: String,
    pub uti: String,
    pub ver: String,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}

/// ID tokens are sent to the client application as part of an OpenID Connect flow.
/// They can be sent alongside or instead of an access token. ID tokens are used by the
/// client to authenticate the user. To learn more about how the Microsoft identity
/// platform issues ID tokens, see [ID tokens in the Microsoft identity platform.](https://learn.microsoft.com/en-us/azure/active-directory/develop/id-tokens)
#[derive(Default, Clone, Eq, PartialEq, Serialize)]
pub struct IdToken {
    pub code: Option<String>,
    pub id_token: String,
    pub state: Option<String>,
    pub session_state: Option<String>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
    #[serde(skip)]
    log_pii: bool,
}

impl IdToken {
    pub fn new(
        id_token: &str,
        code: Option<&str>,
        state: Option<&str>,
        session_state: Option<&str>,
    ) -> IdToken {
        IdToken {
            code: code.map(|value| value.into()),
            id_token: id_token.into(),
            state: state.map(|value| value.into()),
            session_state: session_state.map(|value| value.into()),
            additional_fields: Default::default(),
            log_pii: false,
        }
    }

    /// Decode the id token payload.
    pub fn decode_payload(&self) -> JwtErrors::Result<serde_json::Value> {
        let parts: Vec<&str> = self.id_token.split('.').collect();
        if parts.is_empty() {
            return Err(JwtErrors::Error::from(JwtErrors::ErrorKind::InvalidToken));
        }
        let payload_decoded = base64::engine::general_purpose::STANDARD_NO_PAD
            .decode(parts[1])
            .unwrap();
        let utf8_payload = String::from_utf8(payload_decoded)?.to_owned();
        let payload: serde_json::Value = serde_json::from_str(&utf8_payload)?;
        Ok(payload)
    }

    /// Decode the id token header.
    pub fn decode_header(&self) -> JwtErrors::Result<jsonwebtoken::Header> {
        /*
        let parts: Vec<&str> = self.id_token.split('.').collect();
        if parts.is_empty() {
            return  Err(JwtErrors::Error::from(JwtErrors::ErrorKind::InvalidToken));
        }
        let header_decoded = base64::engine::general_purpose::STANDARD_NO_PAD.decode(parts[0])?;
        let utf8_header = String::from_utf8(header_decoded)?;
        let jwt_header: JwtHeader = serde_json::from_str(&utf8_header)?;
         */

        jsonwebtoken::decode_header(self.id_token.as_str())
    }

    /// Decode and validate the id token.
    pub fn decode(
        &self,
        n: &str,
        e: &str,
        client_id: &str,
        issuer: Option<&str>,
    ) -> JwtErrors::Result<TokenData<Claims>> {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[client_id]);
        if let Some(issuer) = issuer {
            validation.set_issuer(&[issuer]);
        }

        jsonwebtoken::decode::<Claims>(
            &self.id_token,
            &DecodingKey::from_rsa_components(n, e).unwrap(),
            &validation,
        )
    }

    /// Enable or disable logging of personally identifiable information such
    /// as logging the id_token. This is disabled by default. When log_pii is enabled
    /// passing an [IdToken] to logging or print functions will log id_token field.
    /// By default this does not get logged.
    pub fn enable_pii_logging(&mut self, log_pii: bool) {
        self.log_pii = log_pii;
    }
}

impl Display for IdToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#?}, {:#?}, {:#?}, {:#?}",
            self.id_token, self.state, self.session_state, self.code
        )
    }
}

impl AsRef<str> for IdToken {
    fn as_ref(&self) -> &str {
        self.id_token.as_str()
    }
}

impl TryFrom<String> for IdToken {
    type Error = serde::de::value::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id_token: IdToken = IdToken::from_str(value.as_str())?;
        Ok(id_token)
    }
}

impl TryFrom<&str> for IdToken {
    type Error = serde::de::value::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id_token: IdToken = IdToken::from_str(value)?;
        Ok(id_token)
    }
}

impl Debug for IdToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.log_pii {
            f.debug_struct("IdToken")
                .field("code", &self.code)
                .field("id_token", &self.id_token)
                .field("session_state", &self.session_state)
                .field("additional_fields", &self.additional_fields)
                .finish()
        } else {
            f.debug_struct("IdToken")
                .field("code", &self.code)
                .field("id_token", &"[REDACTED]")
                .field("session_state", &self.session_state)
                .field("additional_fields", &self.additional_fields)
                .finish()
        }
    }
}

struct IdTokenVisitor;

impl<'de> Deserialize<'de> for IdToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        impl<'de> Visitor<'de> for IdTokenVisitor {
            type Value = IdToken;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("`code`, `id_token`, `state`, and `session_state`")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let d = serde_urlencoded::Deserializer::new(parse(v.as_bytes()));
                d.deserialize_str(IdTokenVisitor)
                    .map_err(|err| Error::custom(err))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let d = serde_urlencoded::Deserializer::new(parse(v));
                d.deserialize_bytes(IdTokenVisitor)
                    .map_err(|err| Error::custom(err))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut id_token = IdToken::default();
                while let Ok(Some((key, value))) = map.next_entry::<String, String>() {
                    match key.as_bytes() {
                        b"code" => id_token.code = Some(value),
                        b"id_token" => id_token.id_token = value,
                        b"state" => id_token.state = Some(value),
                        b"session_state" => id_token.session_state = Some(value),
                        _ => {
                            id_token
                                .additional_fields
                                .insert(key.to_string(), Value::String(value.to_string()));
                        }
                    }
                }

                Ok(id_token)
            }
        }
        deserializer.deserialize_identifier(IdTokenVisitor)
    }
}

impl FromStr for IdToken {
    type Err = serde::de::value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let deserialize_result = serde_urlencoded::from_str(s);
        if deserialize_result.is_err() {
            return Ok(IdToken::new(s, None, None, None));
        }
        deserialize_result
    }
}
