use crate::GraphHeaders;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::io::ErrorKind;
use std::string::ToString;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct InnerError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "request-id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "innerError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<InnerError>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorStatus>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GraphError {
    pub headers: Option<GraphHeaders>,
    pub error_info: String,
    pub error_type: ErrorType,
    pub code: u16,
    pub error_message: ErrorMessage,
}

impl GraphError {
    pub fn new(
        headers: Option<GraphHeaders>,
        error_info: &str,
        error_type: ErrorType,
        code: u16,
        error_message: ErrorMessage,
    ) -> GraphError {
        GraphError {
            headers,
            error_info: error_info.to_string(),
            error_type,
            code,
            error_message,
        }
    }

    pub fn is_error(status: u16) -> bool {
        ErrorType::from_u16(status).is_some()
    }

    pub fn set_headers(&mut self, headers: GraphHeaders) {
        self.headers = Some(headers);
    }

    pub fn set_error(&mut self, code: u16) -> Result<(), GraphError> {
        let error_type = ErrorType::from_u16(code).unwrap();
        self.error_info = error_type.to_string();
        self.error_type = error_type;
        self.code = code;
        Ok(())
    }

    pub fn set_error_message(&mut self, error_message: ErrorMessage) {
        self.error_message = error_message;
    }

    pub fn try_set_error_message(&mut self, msg: &str) {
        let error_message: Result<ErrorMessage, serde_json::error::Error> =
            serde_json::from_str(msg);
        if let Ok(error_message) = error_message {
            self.set_error_message(error_message);
        }
    }

    pub fn message(&self) -> Option<String> {
        self.error_message.error.as_ref()?.message.clone()
    }

    pub fn code_property(&self) -> Option<String> {
        self.error_message.error.as_ref()?.code.clone()
    }

    pub fn inner_error(&self) -> Option<&InnerError> {
        self.error_message.error.as_ref()?.inner_error.as_ref()
    }

    pub fn request_id(&self) -> Option<String> {
        self.error_message
            .error
            .as_ref()?
            .inner_error
            .as_ref()?
            .request_id
            .clone()
    }

    pub fn date(&self) -> Option<String> {
        self.error_message
            .error
            .as_ref()?
            .inner_error
            .as_ref()?
            .date
            .clone()
    }

    pub fn detailed_error_code(&self) -> Option<String> {
        self.error_message
            .error
            .as_ref()?
            .inner_error
            .as_ref()?
            .code
            .clone()
    }

    pub async fn try_from_async_with_err_message(value: reqwest::Response) -> Option<GraphError> {
        let status = value.status().as_u16();
        let mut graph_error = GraphError::try_from(status).ok()?;
        graph_error.set_headers(GraphHeaders::from(&value));

        if let Ok(text) = value.text().await {
            let error_message: ErrorMessage =
                serde_json::from_str(text.as_str()).unwrap_or_default();
            graph_error.error_message = error_message;
        }

        Some(graph_error)
    }
}

impl Error for GraphError {
    fn description(&self) -> &str {
        if let Some(err) = self.error_message.error.as_ref() {
            if let Some(message) = err.message.as_ref() {
                return message.as_str();
            }
        }
        self.error_info.as_str()
    }

    fn source<'a>(&'a self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl std::fmt::Display for GraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nError Code: {:#?}\nError Message: {:#?}",
            &self.code, &self.error_info
        )
    }
}

impl Default for GraphError {
    fn default() -> Self {
        GraphError::new(
            None,
            ErrorType::BadRequest.as_str(),
            ErrorType::BadRequest,
            400,
            Default::default(),
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ErrorType {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    RequestEntityTooLarge,
    UnsupportedMediaType,
    RequestRangeNotSatisfiable,
    UnprocessableEntity,
    TooManyRequests,
    InternalServerError,
    NotImplemented,
    ServiceUnavailable,
    InsufficientStorage,
    BandwidthLimitExceeded,
    UnknownError,
}

impl ErrorType {}

impl ErrorType {
    pub fn as_str(&self) -> &str {
        match *self {
            ErrorType::BadRequest => "Cannot process the request because it is malformed or incorrect.",
            ErrorType::Unauthorized => "Required authentication information is either missing or not valid for the resource.",
            ErrorType::Forbidden => "Access is denied to the requested resource. The user might not have enough permission.",
            ErrorType::NotFound => "The requested resource doesnt exist.",
            ErrorType::MethodNotAllowed => "The HTTP method in the request is not allowed on the resource.",
            ErrorType::NotAcceptable => "This service doesnt support the format requested in the Accept header.",
            ErrorType::Conflict => "The current state conflicts with what the request expects. For example, the specified parent folder might not exist",
            ErrorType::Gone => "The requested resource is no longer available at the server.",
            ErrorType::LengthRequired => "A Content-Length header is required on the request.",
            ErrorType::PreconditionFailed=> "A precondition provided in the request (such as an if-match header) does not match the resource's current state.",
            ErrorType::RequestEntityTooLarge => "The request size exceeds the maximum limit.",
            ErrorType::UnsupportedMediaType => "The content type of the request is a format that is not supported by the service.",
            ErrorType::RequestRangeNotSatisfiable => "The specified byte range is invalid or unavailable.",
            ErrorType::UnprocessableEntity => "Cannot process the request because it is semantically incorrect.",
            ErrorType::TooManyRequests => "Client application has been throttled and should not attempt to repeat the request until an amount of time has elapsed.",
            ErrorType::InternalServerError => "There was an internal server error while processing the request.",
            ErrorType::NotImplemented => "The requested feature isn’t implemented.",
            ErrorType::ServiceUnavailable => "The service is temporarily unavailable. You may repeat the request after a delay. There may be a Retry-After header.",
            ErrorType::InsufficientStorage => "The maximum storage quota has been reached.",
            ErrorType::BandwidthLimitExceeded => "Your app has been throttled for exceeding the maximum bandwidth cap. Your app can retry the request again after more time has elapsed.",
            ErrorType::UnknownError => "Unknown error or failure",
        }
    }

    pub fn from_u16(num: u16) -> Option<ErrorType> {
        match num {
            400 => Some(ErrorType::BadRequest),
            401 => Some(ErrorType::Unauthorized),
            403 => Some(ErrorType::Forbidden),
            404 => Some(ErrorType::NotFound),
            405 => Some(ErrorType::MethodNotAllowed),
            406 => Some(ErrorType::NotAcceptable),
            409 => Some(ErrorType::Conflict),
            410 => Some(ErrorType::Gone),
            411 => Some(ErrorType::LengthRequired),
            412 => Some(ErrorType::PreconditionFailed),
            413 => Some(ErrorType::RequestEntityTooLarge),
            415 => Some(ErrorType::UnsupportedMediaType),
            416 => Some(ErrorType::RequestRangeNotSatisfiable),
            422 => Some(ErrorType::UnprocessableEntity),
            429 => Some(ErrorType::TooManyRequests),
            500 => Some(ErrorType::InternalServerError),
            501 => Some(ErrorType::NotImplemented),
            503 => Some(ErrorType::ServiceUnavailable),
            507 => Some(ErrorType::InsufficientStorage),
            509 => Some(ErrorType::BandwidthLimitExceeded),
            _ => None,
        }
    }

    pub fn is_error(status: u16) -> bool {
        ErrorType::from_u16(status).is_some()
    }
}

impl ToString for ErrorType {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

/// Returns the matching GraphError for a u16.
/// This method will panic with a NoneError if there is no corresponding u16 to match.
/// Only use this method if you are sure the u16 given will match
/// or you if the method results with a panic.

impl TryFrom<u16> for GraphError {
    type Error = std::io::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if !GraphError::is_error(value) {
            return Err(std::io::Error::new(
                ErrorKind::InvalidInput,
                "u16 given is not a graph error.",
            ));
        }

        let error_type = ErrorType::from_u16(value).unwrap();
        Ok(GraphError {
            headers: None,
            error_info: error_type.to_string(),
            error_type,
            code: value,
            error_message: Default::default(),
        })
    }
}

impl TryFrom<reqwest::blocking::Response> for GraphError {
    type Error = std::io::Error;

    fn try_from(value: reqwest::blocking::Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        let mut graph_error = GraphError::try_from(status)?;
        graph_error.set_headers(GraphHeaders::from(&value));

        if let Ok(text) = value.text() {
            let error_message: ErrorMessage =
                serde_json::from_str(text.as_str()).unwrap_or_default();
            graph_error.error_message = error_message;
        }

        Ok(graph_error)
    }
}

impl TryFrom<&reqwest::blocking::Response> for GraphError {
    type Error = std::io::Error;

    fn try_from(value: &reqwest::blocking::Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        let mut graph_error = GraphError::try_from(status)?;
        graph_error.set_headers(GraphHeaders::from(value));
        Ok(graph_error)
    }
}

impl TryFrom<&reqwest::Response> for GraphError {
    type Error = std::io::Error;

    fn try_from(value: &reqwest::Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        let mut graph_error = GraphError::try_from(status)?;
        graph_error.set_headers(GraphHeaders::from(value));
        Ok(graph_error)
    }
}
