use crate::auth::OAuth;
use crate::identity::Authority;

#[derive(Clone)]
#[allow(dead_code)]
pub struct ClientCertificateCredential {
    /// The client (application) ID of the service principal
    pub(crate) client_id: String,
    pub(crate) certificate: String,
    /// The value passed for the scope parameter in this request should be the resource
    /// identifier (application ID URI) of the resource you want, affixed with the .default
    /// suffix. For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// Default is https://graph.microsoft.com/.default.
    pub(crate) scopes: Vec<String>,
    pub(crate) authority: Authority,
    serializer: OAuth,
}

impl ClientCertificateCredential {
    pub fn builder() -> ClientCertificateCredentialBuilder {
        ClientCertificateCredentialBuilder::new()
    }
}

pub struct ClientCertificateCredentialBuilder {
    credential: ClientCertificateCredential,
}

impl ClientCertificateCredentialBuilder {
    fn new() -> ClientCertificateCredentialBuilder {
        ClientCertificateCredentialBuilder {
            credential: ClientCertificateCredential {
                client_id: String::new(),
                certificate: String::new(),
                scopes: vec![],
                authority: Default::default(),
                serializer: OAuth::new(),
            },
        }
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.credential.client_id = client_id.as_ref().to_owned();
        self
    }

    pub fn with_certificate<T: AsRef<str>>(&mut self, certificate: T) -> &mut Self {
        self.credential.certificate = certificate.as_ref().to_owned();
        self
    }

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.credential.authority = Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.credential.authority = authority.into();
        self
    }

    /// Defaults to "https://graph.microsoft.com/.default"
    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scopes: I) -> &mut Self {
        self.credential.scopes = scopes.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn build(&self) -> ClientCertificateCredential {
        self.credential.clone()
    }
}
