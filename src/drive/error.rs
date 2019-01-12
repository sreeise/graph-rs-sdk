#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveResponseError {
    error: Error,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    code: String,
    message: String,
    #[serde(rename = "innerError")]
    inner_error: InnerError,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerError {
    #[serde(rename = "request-id")]
    request_id: String,
    date: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorResponse {
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    RequestEntityTooLarge = 413,
    UnsupportedMediaType = 415,
    RequestRangeNotSatisfiable = 416,
    UnprocessableEntity = 422,
    TooManyRequests = 429,
    InternalServerError = 500,
    NotImplemented = 501,
    ServiceUnavailable = 503,
    InsufficientStorage = 507,
    BandwidthLimitExceeded = 509,
}

impl ErrorResponse {
    pub fn as_str(&self) -> &str {
        match *self {
            ErrorResponse::BadRequest => "Cannot process the request because it is malformed or incorrect.",
            ErrorResponse::Unauthorized => "Required authentication information is either missing or not valid for the resource.",
            ErrorResponse::Forbidden => "Access is denied to the requested resource. The user might not have enough permission.",
            ErrorResponse::NotFound => "The requested resource doesn’t exist.",
            ErrorResponse::MethodNotAllowed => "The HTTP method in the request is not allowed on the resource.",
            ErrorResponse::NotAcceptable => "This service doesn’t support the format requested in the Accept header.",
            ErrorResponse::Conflict => "The current state conflicts with what the request expects. For example, the specified parent folder might not exist",
            ErrorResponse::Gone => "The requested resource is no longer available at the server.",
            ErrorResponse::LengthRequired => "A Content-Length header is required on the request.",
            ErrorResponse::PreconditionFailed=> "A precondition provided in the request (such as an if-match header) does not match the resource's current state.",
            ErrorResponse::RequestEntityTooLarge => "The request size exceeds the maximum limit.",
            ErrorResponse::UnsupportedMediaType => " 	The content type of the request is a format that is not supported by the service.",
            ErrorResponse::RequestRangeNotSatisfiable => "The specified byte range is invalid or unavailable.",
            ErrorResponse::UnprocessableEntity => "Cannot process the request because it is semantically incorrect.",
            ErrorResponse::TooManyRequests => "Client application has been throttled and should not attempt to repeat the request until an amount of time has elapsed.",
            ErrorResponse::InternalServerError => "There was an internal server error while processing the request.",
            ErrorResponse::NotImplemented => "The requested feature isn’t implemented.",
            ErrorResponse::ServiceUnavailable => "The service is temporarily unavailable. You may repeat the request after a delay. There may be a Retry-After header.",
            ErrorResponse::InsufficientStorage => "The maximum storage quota has been reached.",
            ErrorResponse::BandwidthLimitExceeded => "Your app has been throttled for exceeding the maximum bandwidth cap. Your app can retry the request again after more time has elapsed.",
        }
    }

    pub fn from_u16(num: u16) -> Option<ErrorResponse> {
        match num {
            400 => Some(ErrorResponse::BadRequest),
            401 => Some(ErrorResponse::Unauthorized),
            403 => Some(ErrorResponse::Forbidden),
            404 => Some(ErrorResponse::NotFound),
            405 => Some(ErrorResponse::MethodNotAllowed),
            406 => Some(ErrorResponse::NotAcceptable),
            409 => Some(ErrorResponse::Conflict),
            410 => Some(ErrorResponse::Gone),
            411 => Some(ErrorResponse::LengthRequired),
            412 => Some(ErrorResponse::PreconditionFailed),
            413 => Some(ErrorResponse::RequestEntityTooLarge),
            415 => Some(ErrorResponse::UnsupportedMediaType),
            416 => Some(ErrorResponse::RequestRangeNotSatisfiable),
            422 => Some(ErrorResponse::UnprocessableEntity),
            429 => Some(ErrorResponse::TooManyRequests),
            500 => Some(ErrorResponse::InternalServerError),
            501 => Some(ErrorResponse::NotImplemented),
            503 => Some(ErrorResponse::ServiceUnavailable),
            507 => Some(ErrorResponse::InsufficientStorage),
            509 => Some(ErrorResponse::BandwidthLimitExceeded),
            _ => None,
        }
    }
}
