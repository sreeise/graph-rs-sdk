#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DriveError {
    pub error_info: String,
    pub error_type: DriveErrorType,
    pub code: u16,
}

impl DriveError {
    pub fn new(error_info: &str, error_type: DriveErrorType, code: u16) -> DriveError {
        DriveError {
            error_info: error_info.to_string(),
            error_type,
            code,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveErrorInfo {
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveErrorType {
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
}

impl DriveErrorType {
    pub fn as_str(&self) -> &str {
        match *self {
            DriveErrorType::BadRequest => "Cannot process the request because it is malformed or incorrect.",
            DriveErrorType::Unauthorized => "Required authentication information is either missing or not valid for the resource.",
            DriveErrorType::Forbidden => "Access is denied to the requested resource. The user might not have enough permission.",
            DriveErrorType::NotFound => "The requested resource doesn’t exist.",
            DriveErrorType::MethodNotAllowed => "The HTTP method in the request is not allowed on the resource.",
            DriveErrorType::NotAcceptable => "This service doesn’t support the format requested in the Accept header.",
            DriveErrorType::Conflict => "The current state conflicts with what the request expects. For example, the specified parent folder might not exist",
            DriveErrorType::Gone => "The requested resource is no longer available at the server.",
            DriveErrorType::LengthRequired => "A Content-Length header is required on the request.",
            DriveErrorType::PreconditionFailed=> "A precondition provided in the request (such as an if-match header) does not match the resource's current state.",
            DriveErrorType::RequestEntityTooLarge => "The request size exceeds the maximum limit.",
            DriveErrorType::UnsupportedMediaType => " 	The content type of the request is a format that is not supported by the service.",
            DriveErrorType::RequestRangeNotSatisfiable => "The specified byte range is invalid or unavailable.",
            DriveErrorType::UnprocessableEntity => "Cannot process the request because it is semantically incorrect.",
            DriveErrorType::TooManyRequests => "Client application has been throttled and should not attempt to repeat the request until an amount of time has elapsed.",
            DriveErrorType::InternalServerError => "There was an internal server error while processing the request.",
            DriveErrorType::NotImplemented => "The requested feature isn’t implemented.",
            DriveErrorType::ServiceUnavailable => "The service is temporarily unavailable. You may repeat the request after a delay. There may be a Retry-After header.",
            DriveErrorType::InsufficientStorage => "The maximum storage quota has been reached.",
            DriveErrorType::BandwidthLimitExceeded => "Your app has been throttled for exceeding the maximum bandwidth cap. Your app can retry the request again after more time has elapsed.",
        }
    }

    pub fn from_u16(num: u16) -> Option<DriveErrorType> {
        match num {
            400 => Some(DriveErrorType::BadRequest),
            401 => Some(DriveErrorType::Unauthorized),
            403 => Some(DriveErrorType::Forbidden),
            404 => Some(DriveErrorType::NotFound),
            405 => Some(DriveErrorType::MethodNotAllowed),
            406 => Some(DriveErrorType::NotAcceptable),
            409 => Some(DriveErrorType::Conflict),
            410 => Some(DriveErrorType::Gone),
            411 => Some(DriveErrorType::LengthRequired),
            412 => Some(DriveErrorType::PreconditionFailed),
            413 => Some(DriveErrorType::RequestEntityTooLarge),
            415 => Some(DriveErrorType::UnsupportedMediaType),
            416 => Some(DriveErrorType::RequestRangeNotSatisfiable),
            422 => Some(DriveErrorType::UnprocessableEntity),
            429 => Some(DriveErrorType::TooManyRequests),
            500 => Some(DriveErrorType::InternalServerError),
            501 => Some(DriveErrorType::NotImplemented),
            503 => Some(DriveErrorType::ServiceUnavailable),
            507 => Some(DriveErrorType::InsufficientStorage),
            509 => Some(DriveErrorType::BandwidthLimitExceeded),
            _ => None,
        }
    }

    pub fn drive_error(status: u16) -> Option<DriveError> {
        let error_type = DriveErrorType::from_u16(status);
        if error_type.is_some() {
            let error_type = match error_type {
                Some(t) => t,
                None => {
                    return Some(DriveError::new(
                        DriveErrorType::BadRequest.as_str(),
                        DriveErrorType::BadRequest,
                        400,
                    ));
                }
            };
            // String::from(error_t.as_str()); error_t; status;
            return Some(DriveError {
                error_info: String::from(error_type.as_str()),
                error_type,
                code: status,
            });
        }
        None
    }

    pub fn is_error(status: u16) -> bool {
        DriveErrorType::from_u16(status).is_some()
    }
}
