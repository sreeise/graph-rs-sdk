use base64;
use serde_json::Map;
use serde_json::Value;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::str::FromStr;
use crate::oautherror::OAuthError;

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

/*
 7.2.  Validating a JWT

When validating a JWT, the following steps are performed.  The order
of the steps is not significant in cases where there are no
dependencies between the inputs and outputs of the steps.  If any of
the listed steps fail, then the JWT MUST be rejected -- that is,
treated by the application as an invalid input.

1.   Verify that the JWT contains at least one period ('.')
     character.

2.   Let the Encoded JOSE Header be the portion of the JWT before the
     first period ('.') character.

3.   Base64url decode the Encoded JOSE Header following the
     restriction that no line breaks, whitespace, or other additional
     characters have been used.

4.   Verify that the resulting octet sequence is a UTF-8-encoded
     representation of a completely valid JSON object conforming to
     RFC 7159 [RFC7159]; let the JOSE Header be this JSON object.

5.   Verify that the resulting JOSE Header includes only parameters
     and values whose syntax and semantics are both understood and
     supported or that are specified as being ignored when not
     understood.

6.   Determine whether the JWT is a JWS or a JWE using any of the
     methods described in Section 9 of [JWE].

7.   Depending upon whether the JWT is a JWS or JWE, there are two
     cases:

     *  If the JWT is a JWS, follow the steps specified in [JWS] for
        validating a JWS.  Let the Message be the result of base64url
        decoding the JWS Payload.

     *  Else, if the JWT is a JWE, follow the steps specified in
        [JWE] for validating a JWE.  Let the Message be the resulting
        plaintext.

8.   If the JOSE Header contains a "cty" (content type) value of
     "JWT", then the Message is a JWT that was the subject of nested
     signing or encryption operations.  In this case, return to Step
     1, using the Message as the JWT.

9.   Otherwise, base64url decode the Message following the
     restriction that no line breaks, whitespace, or other additional
     characters have been used.

10.  Verify that the resulting octet sequence is a UTF-8-encoded
     representation of a completely valid JSON object conforming to
     RFC 7159 [RFC7159]; let the JWT Claims Set be this JSON object.

Finally, note that it is an application decision which algorithms may
be used in a given context.  Even if a JWT can be successfully
validated, unless the algorithms used in the JWT are acceptable to
the application, it SHOULD reject the JWT.
 */

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
        self.valid_dot_syntax()?;
        let header = self.decode_header()?;
        self.header = Some(header);
        self.message_jwt()?;

        Ok(())
    }

    // 1. Verify that the JWT contains at least one period ('.')
    // character.
    fn valid_dot_syntax(&self) -> Result<(), OAuthError> {
        if !self.key.contains('.') {
            return OAuthError::invalid_data("Invalid Key");
        }
        Ok(())
    }

    pub fn decode_header(&self) -> Result<Header, OAuthError> {
        // Step 2. Let the Encoded JOSE Header be the portion of the JWT before the first period ('.') character.
        let index = match self.key.find('.') {
            Some(t) => t,
            None => return OAuthError::invalid_data("Invalid Key"),
        };

        // Step 3. Base64url decode the Encoded JOSE Header following the restriction that no
        // line breaks, whitespace, or other additional characters have been used.
        let header = &self.key[..index];
        let header = base64::decode_config(&header, base64::URL_SAFE_NO_PAD)?;

        // Step 4 & 5.
        // Step 4. Verify that the resulting octet sequence is a UTF-8-encoded representation of a
        // completely valid JSON object conforming to RFC 7159 [RFC7159]; let the JOSE
        // Header be this JSON object.
        let h = std::str::from_utf8(&header)?;

        // Step 5. Verify that the resulting JOSE Header includes only parameters
        // and values whose syntax and semantics are both understood and
        // supported or that are specified as being ignored when not
        // understood.
        let value = h.to_owned();
        let jwt_header: Header = serde_json::from_str(&value)?;

        Ok(jwt_header)
    }

    // 6. Determine whether the JWT is a JWS or a JWE using any of the
    // methods described in Section 9 of [JWE].
    pub fn type_of(&self) -> Option<JWTType> {
        let count = self.key.matches('.').count();
        JWTType::type_from(count)
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

    pub fn verify_with_claims(&mut self, claims: Vec<Claim>) -> Result<(), OAuthError> {
        let payload = self.decode_payload()?;
        let mut matched = 0;
        for claim in payload.iter() {
            for value in claims.iter() {
                if claim.key().eq(&value.key()) && value.value().eq(&claim.value()) {
                    matched += 1;
                }
            }
        }

        // Step 6. Determine whether the JWT is a JWS or a JWE using any of the
        // methods described in Section 9 of [JWE].
        match self.type_of() {
            // Those implementing this library should do key verification
            // at steps 7.1 and 7.2. This is not provided here.

            // Ste[ 7.Depending upon whether the JWT is a JWS or JWE, there are two cases:

            // Steps 7.1 If the JWT is a JWS, follow the steps specified in [JWS] for validating
            // a JWS.  Let the Message be the result of base64url decoding the JWS Payload.
            // and
            // Step 7.2 Else, if the JWT is a JWE, follow the steps specified in
            //  [JWE] for validating a JWE.  Let the Message be the resulting plaintext.
            Some(JWTType::JWS) | Some(JWTType::JWE) => {
                // 7.2 and 7.2 is verifying the JWT using the Algorithm.
                // We only verify the claims here.
                if claims.len() != matched {
                    return OAuthError::none_error(&format!(
                        "Not all given claims were found. Total given claims: {}, Matched claims: {}",
                        claims.len(),
                        matched
                    ));
                }

                self.has_duplicates(payload)
            },
            None => OAuthError::invalid_data("Invalid header"),
        }
    }

    fn message_jwt(&mut self) -> Result<(), OAuthError> {
        // 8.   If the JOSE Header contains a "cty" (content type) value of
        //  "JWT", then the Message is a JWT that was the subject of nested
        //  signing or encryption operations.  In this case, return to Step
        //  1, using the Message as the JWT.
        // and
        // 9. Otherwise, base64url decode the Message following the
        // restriction that no line breaks, whitespace, or other additional
        // characters have been used.

        let v_claim = self.decode_payload()?;
        let claims = v_claim.iter().find(|v| v.key == "cty");
        if let Some(c) = claims {
            let value = match c.value.as_str() {
                Some(v) => v,
                None => return OAuthError::none_error("Unwrap on none value for Claim"),
            };
            let mut jwt = JWT::new(value);
            jwt.validate()?
        }
        Ok(())
    }

    pub fn decode_claims(&mut self) -> std::result::Result<Map<String, Value>, OAuthError> {
        // 10.  Verify that the resulting octet sequence is a UTF-8-encoded
        //     representation of a completely valid JSON object conforming to
        //     RFC 7159 [RFC7159]; let the JWT Claims Set be this JSON object.
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
}
