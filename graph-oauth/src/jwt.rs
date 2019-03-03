use base64;
use crate::oautherror::OAuthError;
use std::io::ErrorKind;
use std::any::TypeId;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Header {
    typ: String,
    alg: Algorithm,
}

impl Header {
    pub fn typ(&self) -> String {
        self.typ.clone()
    }

    pub fn alg(&self) -> Algorithm {
        self.alg.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Algorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512
}

impl Algorithm {
    pub fn from_str(s: &str) -> Result<Algorithm, OAuthError> {
        match s {
            "HS256" => Ok(Algorithm::HS256),
            "HS384" => Ok(Algorithm::ES384),
            "HS512" => Ok(Algorithm::HS512),
            "RS256" => Ok(Algorithm::RS256),
            "RS384" => Ok(Algorithm::RS384),
            "RS512" =>Ok(Algorithm::RS512),
            "ES256" => Ok(Algorithm::ES256),
            "ES384" => Ok(Algorithm::ES384),
            "ES512" => Ok(Algorithm::ES512),
            _ => Err(OAuthError::error_kind(ErrorKind::NotFound, "Not an Algorithm Type")),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct JWT {
    key: String,
    header: Option<Header>,
}

impl JWT {
    pub fn new(key: &str) -> JWT {
        JWT {
            key: key.into(),
            header: None,
        }
    }

    pub fn header(&self) -> Option<Header> {
        self.header.clone()
    }


    // https://tools.ietf.org/html/rfc7519#section-7.2
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
    pub fn validate(&mut self) -> std::result::Result<(), OAuthError> {
        // Step 1. Verify that the JWT contains at least one period ('.') character.
        if !self.key.contains('.') {
            return Err(OAuthError::error_kind(ErrorKind::InvalidData, "Invalid Key"));
        }

        // Step 2. Let the Encoded JOSE Header be the portion of the JWT before the first period ('.') character.
        let index = match self.key.find('.') {
            Some(t) => t,
            None => return Err(OAuthError::error_kind(ErrorKind::InvalidData, "Invalid Key")),
        };

        let header = &self.key[..index];

        // Step 3. Base64url decode the Encoded JOSE Header following the restriction that no
        // line breaks, whitespace, or other additional characters have been used.
        let header = base64::decode(&header).map_err(|e| OAuthError::from(e));

        let jwt_header = match header {
            Ok(t) => t,
            Err(e) => return Err(e),
        };


        // Step 4 & 5.
        // Step 4. Verify that the resulting octet sequence is a UTF-8-encoded representation of a
        // completely valid JSON object conforming to RFC 7159 [RFC7159]; let the JOSE
        // Header be this JSON object.
        let h = match std::str::from_utf8(&jwt_header) {
            Ok(t) => t,
            Err(e) => return Err(OAuthError::from(e))
        };

        // Step 5. Verify that the resulting JOSE Header includes only parameters
        // and values whose syntax and semantics are both understood and
        // supported or that are specified as being ignored when not
        // understood.
        let value = h.to_owned();
        let jwt_header: Header = match serde_json::from_str(&value) {
            Ok(t) => t,
            Err(e) => return Err(OAuthError::from(e))
        };

        if jwt_header.typ != "JWT" {
            return Err(OAuthError::error_kind(ErrorKind::InvalidData, "Invalid Header"));
        }

        self.header = Some(jwt_header.to_owned());

        Ok(())
    }
}