use crate::{AuthorizationResponse, IntoCredentialBuilder};
use graph_error::{WebViewError, WebViewResult};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum WebViewAuthorizationEvent<CredentialBuilder: Clone + Debug> {
    Authorized {
        authorization_response: AuthorizationResponse,
        credential_builder: CredentialBuilder,
    },
    Unauthorized(AuthorizationResponse),
    WindowClosed(String),
}

impl<CredentialBuilder: Clone + Debug> IntoCredentialBuilder<CredentialBuilder>
    for WebViewAuthorizationEvent<CredentialBuilder>
{
    type Response = AuthorizationResponse;
    type Error = WebViewError;

    fn into_credential_builder(self) -> Result<(Self::Response, CredentialBuilder), Self::Error> {
        match self {
            WebViewAuthorizationEvent::Authorized {
                authorization_response,
                credential_builder,
            } => Ok((authorization_response, credential_builder)),
            WebViewAuthorizationEvent::Unauthorized(authorization_response) => {
                Err(WebViewError::Authorization {
                    error: authorization_response
                        .error
                        .map(|query_error| query_error.to_string())
                        .unwrap_or_default(),
                    error_description: authorization_response.error_description.unwrap_or_default(),
                    error_uri: authorization_response.error_uri.map(|uri| uri.to_string()),
                })
            }
            WebViewAuthorizationEvent::WindowClosed(reason) => {
                Err(WebViewError::WindowClosed(reason))
            }
        }
    }
}

impl<CredentialBuilder: Clone + Debug> IntoCredentialBuilder<CredentialBuilder>
    for WebViewResult<WebViewAuthorizationEvent<CredentialBuilder>>
{
    type Response = AuthorizationResponse;
    type Error = WebViewError;

    fn into_credential_builder(self) -> Result<(Self::Response, CredentialBuilder), Self::Error> {
        match self {
            Ok(auth_event) => match auth_event {
                WebViewAuthorizationEvent::Authorized {
                    authorization_response,
                    credential_builder,
                } => Ok((authorization_response, credential_builder)),
                WebViewAuthorizationEvent::Unauthorized(authorization_response) => {
                    Err(WebViewError::Authorization {
                        error: authorization_response
                            .error
                            .map(|query_error| query_error.to_string())
                            .unwrap_or_default(),
                        error_description: authorization_response
                            .error_description
                            .unwrap_or_default(),
                        error_uri: authorization_response.error_uri.map(|uri| uri.to_string()),
                    })
                }
                WebViewAuthorizationEvent::WindowClosed(reason) => {
                    Err(WebViewError::WindowClosed(reason))
                }
            },
            Err(err) => Err(err),
        }
    }
}
