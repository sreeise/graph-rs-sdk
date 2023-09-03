use crate::auth::{OAuthParameter, OAuthSerializer};
use crate::identity::credentials::app_config::AppConfig;
use crate::identity::{
    AsQuery, Authority, AuthorizationUrl, AzureCloudInstance, Crypto, Prompt, ResponseMode,
    ResponseType,
};
use graph_error::{AuthorizationFailure, AuthorizationResult, AF};
use reqwest::IntoUrl;
use std::collections::BTreeSet;
use url::form_urlencoded::Serializer;
use url::Url;

/// OpenID Connect (OIDC) extends the OAuth 2.0 authorization protocol for use as an additional
/// authentication protocol. You can use OIDC to enable single sign-on (SSO) between your
/// OAuth-enabled applications by using a security token called an ID token.
/// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc
#[derive(Clone, Debug)]
pub struct OpenIdAuthorizationUrl {
    pub(crate) app_config: AppConfig,
    /// Required
    /// Must include code for OpenID Connect sign-in.
    pub(crate) response_type: BTreeSet<ResponseType>,
    /// Optional
    /// Specifies how the identity platform should return the requested token to your app.
    ///
    /// Specifies the method that should be used to send the resulting authorization code back
    /// to your app.
    ///
    /// Can be form_post or fragment.
    ///
    /// For web applications, Microsoft recommends using response_mode=form_post,
    /// to ensure the most secure transfer of tokens to your application.
    ///
    /// Open Id does not support the query response mode.
    ///
    /// Supported values:
    ///
    /// - fragment: Default when requesting an ID token by using the implicit flow.
    /// Also supported if requesting only a code.
    /// - form_post: Executes a POST containing the code to your redirect URI.
    /// Supported when requesting a code.
    pub(crate) response_mode: Option<ResponseMode>,
    /// Optional
    /// A value generated and sent by your app in its request for an ID token. The same nonce
    /// value is included in the ID token returned to your app by the Microsoft identity platform.
    /// To mitigate token replay attacks, your app should verify the nonce value in the ID token
    /// is the same value it sent when requesting the token. The value is typically a unique,
    /// random string.
    pub(crate) nonce: String,
    /// Required
    /// A value included in the request that also will be returned in the token response.
    /// It can be a string of any content you want. A randomly generated unique value typically
    /// is used to prevent cross-site request forgery attacks. The state also is used to encode
    /// information about the user's state in the app before the authentication request occurred,
    /// such as the page or view the user was on.
    pub(crate) state: Option<String>,
    /// Required - the openid scope is already included
    /// A space-separated list of scopes. For OpenID Connect, it must include the scope openid,
    /// which translates to the Sign you in permission in the consent UI. You might also include
    /// other scopes in this request for requesting consent.
    pub(crate) scope: BTreeSet<String>,
    /// Optional
    /// Indicates the type of user interaction that is required. The only valid values at
    /// this time are login, none, consent, and select_account.
    ///
    /// The [Prompt::Login] claim forces the user to enter their credentials on that request,
    /// which negates single sign-on.
    ///
    /// The [Prompt::None] parameter is the opposite, and should be paired with a login_hint to
    /// indicate which user must be signed in. These parameters ensure that the user isn't
    /// presented with any interactive prompt at all. If the request can't be completed silently
    /// via single sign-on, the Microsoft identity platform returns an error. Causes include no
    /// signed-in user, the hinted user isn't signed in, or multiple users are signed in but no
    /// hint was provided.
    ///
    /// The [Prompt::Consent] claim triggers the OAuth consent dialog after the
    /// user signs in. The dialog asks the user to grant permissions to the app.
    ///
    /// Finally, [Prompt::SelectAccount] shows the user an account selector, negating silent SSO but
    /// allowing the user to pick which account they intend to sign in with, without requiring
    /// credential entry. You can't use both login_hint and select_account.
    pub(crate) prompt: BTreeSet<Prompt>,
    /// Optional
    /// The realm of the user in a federated directory. This skips the email-based discovery
    /// process that the user goes through on the sign-in page, for a slightly more streamlined
    /// user experience. For tenants that are federated through an on-premises directory
    /// like AD FS, this often results in a seamless sign-in because of the existing login session.
    pub(crate) domain_hint: Option<String>,
    /// Optional
    /// You can use this parameter to pre-fill the username and email address field of the
    /// sign-in page for the user, if you know the username ahead of time. Often, apps use
    /// this parameter during re-authentication, after already extracting the login_hint
    /// optional claim from an earlier sign-in.
    pub(crate) login_hint: Option<String>,
    response_types_supported: Vec<String>,
}

impl OpenIdAuthorizationUrl {
    pub fn new<T: AsRef<str>, IU: IntoUrl, U: ToString, I: IntoIterator<Item = U>>(
        client_id: T,
        redirect_uri: IU,
        scope: I,
    ) -> AuthorizationResult<OpenIdAuthorizationUrl> {
        let mut scope_set = BTreeSet::new();
        scope_set.insert("openid".to_owned());
        scope_set.extend(scope.into_iter().map(|s| s.to_string()));
        let redirect_uri_result = Url::parse(redirect_uri.as_str());

        Ok(OpenIdAuthorizationUrl {
            app_config: AppConfig {
                tenant_id: None,
                client_id: client_id.as_ref().to_owned(),
                authority: Default::default(),
                authority_url: Default::default(),
                extra_query_parameters: Default::default(),
                extra_header_parameters: Default::default(),
                redirect_uri: Some(redirect_uri.into_url().or(redirect_uri_result)?),
            },
            response_type: BTreeSet::new(),
            response_mode: None,
            nonce: Crypto::sha256_secure_string()?.1,
            state: None,
            scope: scope_set,
            prompt: BTreeSet::new(),
            domain_hint: None,
            login_hint: None,
            response_types_supported: vec![
                "code".into(),
                "id_token".into(),
                "code id_token".into(),
                "id_token token".into(),
            ],
        })
    }

    pub fn builder() -> AuthorizationResult<OpenIdAuthorizationUrlBuilder> {
        OpenIdAuthorizationUrlBuilder::new()
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
        self.url_with_host(&AzureCloudInstance::default())
    }

    pub fn url_with_host(
        &self,
        azure_authority_host: &AzureCloudInstance,
    ) -> AuthorizationResult<Url> {
        self.authorization_url_with_host(azure_authority_host)
    }

    /// Get the nonce.
    ///
    /// This value may be generated automatically by the client and may be useful for users
    /// who want to manually verify that the nonce stored in the client is the same as the
    /// nonce returned in the response from the authorization server.
    /// Verifying the nonce helps mitigate token replay attacks.
    pub fn nonce(&mut self) -> &String {
        &self.nonce
    }
}

impl AuthorizationUrl for OpenIdAuthorizationUrl {
    fn redirect_uri(&self) -> Option<&Url> {
        self.app_config.redirect_uri.as_ref()
    }

    fn authorization_url(&self) -> AuthorizationResult<Url> {
        self.authorization_url_with_host(&AzureCloudInstance::default())
    }

    fn authorization_url_with_host(
        &self,
        azure_authority_host: &AzureCloudInstance,
    ) -> AuthorizationResult<Url> {
        let mut serializer = OAuthSerializer::new();

        let client_id = self.app_config.client_id.as_str().trim();
        if client_id.is_empty() {
            return AuthorizationFailure::result("client_id");
        }

        if self.scope.is_empty() {
            return AuthorizationFailure::result("scope");
        }

        if self.nonce.is_empty() {
            return AuthorizationFailure::msg_result(
                "nonce",
                "nonce is empty - nonce is automatically generated if not updated by the caller",
            );
        }

        serializer
            .client_id(client_id)
            .extend_scopes(self.scope.clone())
            .nonce(self.nonce.as_str())
            .authority(azure_authority_host, &self.app_config.authority);

        if self.response_type.is_empty() {
            serializer.response_type("code");
        } else {
            let response_types = self.response_type.as_query();
            if !self.response_types_supported.contains(&response_types) {
                return AuthorizationFailure::msg_result(
                    "response_type",
                    format!(
                        "response_type is not supported - supported response types are: {}",
                        self.response_types_supported
                            .iter()
                            .map(|s| format!("`{}`", s))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ),
                );
            }

            serializer.response_types(self.response_type.iter());
        }

        if let Some(response_mode) = self.response_mode.as_ref() {
            serializer.response_mode(response_mode.as_ref());
        }

        if let Some(redirect_uri) = self.app_config.redirect_uri.as_ref() {
            serializer.redirect_uri(redirect_uri.as_ref());
        }

        if let Some(state) = self.state.as_ref() {
            serializer.state(state.as_str());
        }

        if !self.prompt.is_empty() {
            serializer.prompt(&self.prompt.as_query());
        }

        if let Some(domain_hint) = self.domain_hint.as_ref() {
            serializer.domain_hint(domain_hint.as_str());
        }

        if let Some(login_hint) = self.login_hint.as_ref() {
            serializer.login_hint(login_hint.as_str());
        }

        let mut encoder = Serializer::new(String::new());
        serializer.encode_query(
            vec![
                OAuthParameter::ResponseMode,
                OAuthParameter::RedirectUri,
                OAuthParameter::State,
                OAuthParameter::Prompt,
                OAuthParameter::LoginHint,
                OAuthParameter::DomainHint,
            ],
            vec![
                OAuthParameter::ClientId,
                OAuthParameter::ResponseType,
                OAuthParameter::Scope,
                OAuthParameter::Nonce,
            ],
            &mut encoder,
        )?;

        let authorization_url = serializer
            .get(OAuthParameter::AuthorizationUrl)
            .ok_or(AF::msg_err("authorization_url", "Internal Error"))?;
        let mut url = Url::parse(authorization_url.as_str())?;
        url.set_query(Some(encoder.finish().as_str()));
        Ok(url)
    }
}

pub struct OpenIdAuthorizationUrlBuilder {
    auth_url_parameters: OpenIdAuthorizationUrl,
}

impl OpenIdAuthorizationUrlBuilder {
    pub(crate) fn new() -> AuthorizationResult<OpenIdAuthorizationUrlBuilder> {
        let mut scope = BTreeSet::new();
        scope.insert("openid".to_owned());

        Ok(OpenIdAuthorizationUrlBuilder {
            auth_url_parameters: OpenIdAuthorizationUrl {
                app_config: AppConfig::default(),
                response_type: BTreeSet::new(),
                response_mode: None,
                nonce: Crypto::sha256_secure_string()?.1,
                state: None,
                scope,
                prompt: Default::default(),
                domain_hint: None,
                login_hint: None,
                response_types_supported: vec![
                    "code".into(),
                    "id_token".into(),
                    "code id_token".into(),
                    "id_token token".into(),
                ],
            },
        })
    }

    pub fn with_redirect_uri<T: AsRef<str>>(
        &mut self,
        redirect_uri: T,
    ) -> anyhow::Result<&mut Self> {
        self.auth_url_parameters.app_config.redirect_uri = Some(Url::parse(redirect_uri.as_ref())?);
        Ok(self)
    }

    pub fn with_client_id<T: AsRef<str>>(&mut self, client_id: T) -> &mut Self {
        self.auth_url_parameters.app_config.client_id = client_id.as_ref().to_owned();
        self
    }

    /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
    pub fn with_tenant<T: AsRef<str>>(&mut self, tenant: T) -> &mut Self {
        self.auth_url_parameters.app_config.authority =
            Authority::TenantId(tenant.as_ref().to_owned());
        self
    }

    pub fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
        self.auth_url_parameters.app_config.authority = authority.into();
        self
    }

    /// Default is code.
    /// Must include code for the open id connect flow.
    /// Can also include id_token or token if using the hybrid flow.
    ///
    /// Supported response types are:
    ///
    /// - code
    /// - id_token
    /// - code id_token
    /// - id_token token
    pub fn with_response_type<I: IntoIterator<Item = ResponseType>>(
        &mut self,
        response_type: I,
    ) -> &mut Self {
        self.auth_url_parameters.response_type = BTreeSet::from_iter(response_type.into_iter());
        self
    }

    /// Specifies how the identity platform should return the requested token to your app.
    ///
    /// Supported values:
    ///
    /// - **fragment**: Default when requesting an ID token by using the implicit flow.
    ///     Also supported if requesting only a code.
    /// - **form_post**: Executes a POST containing the code to your redirect URI.
    ///     Supported when requesting a code.
    pub fn with_response_mode(&mut self, response_mode: ResponseMode) -> &mut Self {
        self.auth_url_parameters.response_mode = Some(response_mode);
        self
    }

    /// A value included in the request, generated by the app, that is included in the
    /// resulting id_token as a claim. The app can then verify this value to mitigate token
    /// replay attacks. The value is typically a randomized, unique string that can be used
    /// to identify the origin of the request.
    ///
    /// Because openid requires a nonce as part of the OAuth flow a nonce is already included.
    /// The nonce is generated internally using the same requirements of generating a secure
    /// random string as is done when using proof key for code exchange (PKCE) in the
    /// authorization code grant. If you are unsure or unclear how the nonce works then it is
    /// recommended to stay with the generated nonce as it is cryptographically secure.
    pub fn with_nonce<T: AsRef<str>>(&mut self, nonce: T) -> &mut Self {
        if self.auth_url_parameters.nonce.is_empty() {
            self.auth_url_parameters.nonce.push_str(nonce.as_ref());
        } else {
            self.auth_url_parameters.nonce = nonce.as_ref().to_owned();
        }
        self
    }

    /// A value included in the request, generated by the app, that is included in the
    /// resulting id_token as a claim. The app can then verify this value to mitigate token
    /// replay attacks. The value is typically a randomized, unique string that can be used
    /// to identify the origin of the request.
    ///
    /// The nonce is generated in the same way as generating a PKCE.
    ///
    /// Internally this method uses the Rust ring cyrpto library to
    /// generate a secure random 32-octet sequence that is base64 URL
    /// encoded (no padding). This sequence is hashed using SHA256 and
    /// base64 URL encoded (no padding) resulting in a 43-octet URL safe string.
    #[doc(hidden)]
    pub(crate) fn with_nonce_generated(&mut self) -> anyhow::Result<&mut Self> {
        self.auth_url_parameters.nonce = Crypto::sha256_secure_string()?.1;
        Ok(self)
    }

    pub fn with_state<T: AsRef<str>>(&mut self, state: T) -> &mut Self {
        self.auth_url_parameters.state = Some(state.as_ref().to_owned());
        self
    }

    /// Takes an iterator of scopes to use in the request.
    /// Replaces current scopes if any were added previously.
    /// To extend scopes use [OpenIdAuthorizationUrlBuilder::extend_scope].
    pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.auth_url_parameters.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Extend the current list of scopes.
    pub fn extend_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) -> &mut Self {
        self.auth_url_parameters
            .scope
            .extend(scope.into_iter().map(|s| s.to_string()));
        self
    }

    /// Automatically adds profile, email, and offline_access to the scope parameter.
    /// The openid scope is already included when using [OpenIdCredential]
    pub fn with_default_scope(&mut self) -> anyhow::Result<&mut Self> {
        self.with_nonce_generated()?;
        self.with_response_type(vec![ResponseType::Code, ResponseType::IdToken]);
        self.auth_url_parameters.scope.extend(
            vec!["profile", "email", "offline_access"]
                .into_iter()
                .map(|s| s.to_string()),
        );
        Ok(self)
    }

    /// Indicates the type of user interaction that is required. Valid values are login, none,
    /// consent, and select_account.
    ///
    /// - **prompt=login** forces the user to enter their credentials on that request, negating single-sign on.
    /// - **prompt=none** is the opposite. It ensures that the user isn't presented with any interactive prompt.
    ///     If the request can't be completed silently by using single-sign on, the Microsoft identity platform returns an interaction_required error.
    /// - **prompt=consent** triggers the OAuth consent dialog after the user signs in, asking the user to
    ///     grant permissions to the app.
    /// - **prompt=select_account** interrupts single sign-on providing account selection experience
    ///     listing all the accounts either in session or any remembered account or an option to choose to use a different account altogether.
    pub fn with_prompt<I: IntoIterator<Item = Prompt>>(&mut self, prompt: I) -> &mut Self {
        self.auth_url_parameters.prompt.extend(prompt.into_iter());
        self
    }

    /// Optional
    /// The realm of the user in a federated directory. This skips the email-based discovery
    /// process that the user goes through on the sign-in page, for a slightly more streamlined
    /// user experience. For tenants that are federated through an on-premises directory
    /// like AD FS, this often results in a seamless sign-in because of the existing login session.
    pub fn with_domain_hint<T: AsRef<str>>(&mut self, domain_hint: T) -> &mut Self {
        self.auth_url_parameters.domain_hint = Some(domain_hint.as_ref().to_owned());
        self
    }

    /// Optional
    /// You can use this parameter to pre-fill the username and email address field of the
    /// sign-in page for the user, if you know the username ahead of time. Often, apps use
    /// this parameter during reauthentication, after already extracting the login_hint
    /// optional claim from an earlier sign-in.
    pub fn with_login_hint<T: AsRef<str>>(&mut self, login_hint: T) -> &mut Self {
        self.auth_url_parameters.login_hint = Some(login_hint.as_ref().to_owned());
        self
    }

    pub fn build(&self) -> OpenIdAuthorizationUrl {
        self.auth_url_parameters.clone()
    }

    pub fn url(&self) -> AuthorizationResult<Url> {
        self.auth_url_parameters.url()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn unsupported_response_type() {
        let _ = OpenIdAuthorizationUrl::builder()
            .unwrap()
            .with_response_type([ResponseType::Code, ResponseType::Token])
            .with_client_id("client_id")
            .with_scope(["scope"])
            .url()
            .unwrap();
    }
}
