use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use url::Url;

/// JSON Web Key (JWK) is a JSON object that represents a cryptographic key.
/// The members of the object represent properties of the key, including its value.
/// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct JsonWebKey {
    /// The "kty" (key type) parameter identifies the cryptographic algorithm family used with
    /// the key, such as "RSA" or "EC".  "kty" values should either be registered in the
    /// IANA "JSON Web Key Types" registry established by [JWA] or be a value that contains
    /// a Collision-Resistant Name.  The "kty" value is a case-sensitive string.
    /// This member MUST be present in a JWK.
    /// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4.1)
    pub kty: String,

    /// The "use" (public key use) parameter identifies the intended use of the public key.
    /// The "use" parameter is employed to indicate whether a public key is used for encrypting
    /// data or verifying the signature on data.
    /// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4.2)
    #[serde(alias = "use")]
    pub _use: Option<String>,
    /// The "key_ops" (key operations) parameter identifies the operation(s) for which the key
    /// is intended to be used.  The "key_ops" parameter is intended for use cases in which
    /// public, private, or symmetric keys may be present.
    /// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4.3)
    pub key_ops: Vec<String>,

    /// The "alg" (algorithm) parameter identifies the algorithm intended for use with the key.
    /// The values used should either be registered in the IANA "JSON Web Signature and
    /// Encryption Algorithms" registry established by JWA or be a value that contains
    /// a Collision-Resistant Name.  The "alg" value is a case-sensitive ASCII string.
    /// Use of this member is OPTIONAL.
    /// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4.4)
    pub alg: Option<String>,

    /// The "kid" (key ID) parameter is used to match a specific key.
    /// This is used, for instance, to choose among a set of keys within a JWK Set during key
    /// rollover.  The structure of the "kid" value is unspecified.
    /// When "kid" values are used within a JWK Set, different keys within the JWK Set SHOULD
    /// use distinct "kid" values.  (One example in which different keys might use the
    /// same "kid" value is if they have different "kty" (key type) values but are considered
    /// to be equivalent alternatives by the application using them.)
    /// The "kid" value is a case-sensitive string.  Use of this member is OPTIONAL.
    /// When used with JWS or JWE, the "kid" value is used to match a JWS or JWE "kid"
    /// Header Parameter value.
    /// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4.5)
    pub kid: Option<String>,

    /// The "x5u" (X.509 URL) parameter is a URI that refers to a resource for
    /// an X.509 public key certificate or certificate chain
    /// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4.6)
    pub x5u: Option<Url>,

    /// The "x5c" (X.509 certificate chain) parameter contains a chain of one or more
    /// PKIX certificates [RFC5280](https://datatracker.ietf.org/doc/html/rfc5280).
    /// The certificate chain is represented as a JSON array of certificate value strings.
    /// Each string in the array is a base64-encoded (Section 4 of
    /// [RFC4648](https://datatracker.ietf.org/doc/html/rfc4648#section-4)
    /// -- not base64url-encoded) DER
    /// [ITU.X690.1994](https://datatracker.ietf.org/doc/html/rfc7517#ref-ITU.X690.1994)
    /// PKIX certificate value. The PKIX certificate containing the key value MUST be the first
    /// certificate. This MAY be followed by additional certificates, with each subsequent
    /// certificate being the one used to certify the previous one.  The key in the first
    /// certificate MUST match the public key represented by other members of the JWK.
    /// Use of this member is OPTIONAL.
    /// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4.7)
    pub x5c: Option<String>,

    /// The "x5t" (X.509 certificate SHA-1 thumbprint) parameter is a base64url-encoded
    /// SHA-1 thumbprint (a.k.a. digest) of the DER encoding of an X.509 certificate [RFC5280]
    /// Note that certificate thumbprints are also sometimes known as certificate fingerprints.
    /// The key in the certificate MUST match the public key represented by
    /// other members of the JWK. Use of this member is OPTIONAL
    /// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4.8)
    pub x5t: Option<String>,

    /// The "x5t#S256" (X.509 certificate SHA-256 thumbprint) parameter is a base64url-encoded
    /// SHA-256 thumbprint (a.k.a. digest) of the DER encoding of an X.509 certificate Note that
    /// certificate thumbprints are also sometimes known as certificate fingerprints.
    /// The key in the certificate MUST match the public key represented by other members of
    /// the JWK. Use of this member is OPTIONAL.
    /// [RFC 7517](https://datatracker.ietf.org/doc/html/rfc7517#section-4.9)
    #[serde(alias = "x5t#S256")]
    pub x5t_s256: Option<String>,

    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}

impl Hash for JsonWebKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kty.hash(state);
        self._use.hash(state);
    }
}

impl Display for JsonWebKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "kty: {}, use: {:#?}, key_ops: {:#?}, alg: {:#?}, kid: {:#?}, x5u: {:#?}, x5c: {:#?}, x5t: {:#?}, x5t#S256: {:#?}",
               self.kty, self._use, self.key_ops, self.alg, self.kid, self.x5u, self.x5c, self.x5t, self.x5t_s256 )
    }
}

/// A JSON Web Key Set (JWKS) is a JSON object that represents a set of JWKs. The JSON object MUST
/// have a "keys" member, which is an array of JWKs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonWebKeySet {
    pub keys: HashSet<JsonWebKey>,
}

impl Display for JsonWebKeySet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "keys: {:#?}", self.keys)
    }
}
