use std;
use std::fmt;
use std::io;
use std::num;
use std::error;
use std::error::Error;
use core::option;
use std::string;


#[derive(Debug)]
pub enum OAuthError {
    Io(io::Error),
    Parse(num::ParseIntError),
    ParseString(string::ParseError),
}

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            OAuthError::Io(ref err) => write!(f, "IO error: {}", err),
            OAuthError::Parse(ref err) => write!(f, "Parse error: {}", err),
            OAuthError::ParseString(ref err) => write!(f, "Parse string error: {}", err),
        }
    }
}

impl error::Error for OAuthError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            OAuthError::Io(ref err) => err.description(),
            // Normally we can just write `err.description()`, but the error
            // type has a concrete method called `description`, which conflicts
            // with the trait method. For now, we must explicitly call
            // `description` through the `Error` trait.
            OAuthError::Parse(ref err) => error::Error::description(err),
            OAuthError::ParseString(ref err) => error::Error::description(err),
        }
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            OAuthError::Io(ref err) => Some(err),
            OAuthError::Parse(ref err) => Some(err),
            OAuthError::ParseString(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for OAuthError {
    fn from(err: io::Error) -> OAuthError {
        OAuthError::Io(err)
    }
}

impl From<num::ParseIntError> for OAuthError {
    fn from(err: num::ParseIntError) -> OAuthError {
        OAuthError::Parse(err)
    }
}

impl From<string::ParseError> for OAuthError {
    fn from(err: string::ParseError) -> OAuthError {
        OAuthError::ParseString(err)
    }
}

pub enum FlowErrorType {
    MissingParam,
    AllowReset,
    InvalidAccessCode,
    MissingAccessCode,
    RequiresGrantType,
    BadRequest,
}

// TODO: Probably not the best way to define the errors. Implement finding line numbers as well.
impl FlowErrorType {
    pub fn match_error_type(self) -> FlowError {
        match self {
            FlowErrorType::MissingParam => FlowError::new("Missing one or more parameters", 0, 0),
            FlowErrorType::AllowReset => FlowError::new(
                "Allow reset is set to false. Call self.allow_reset(true) to change",
                0,
                0,
            ),
            FlowErrorType::InvalidAccessCode => FlowError::new(
                "The access code is malformed or unusable for a request",
                0,
                0,
            ),
            FlowErrorType::MissingAccessCode => {
                FlowError::new("Missing access code. Set the access_token to fix.", 0, 0)
            }
            FlowErrorType::RequiresGrantType => {
                FlowError::new("The FlowType used is not a grant type", 0, 0)
            }
            FlowErrorType::BadRequest => FlowError::new(
                "could not get request body: bad request or invalid access_code",
                0,
                0,
            ),
        }
    }

    pub fn missing_param(param: &str) -> FlowError {
        let mut message = String::from("Error, missing parameter: ");
        message.push_str(param);
        FlowError::new(message.as_str(), 0, 0)
    }

    pub fn json_prev_parsed_error(param_type: &str) -> FlowError {
        let message = format!(
            "The parsed JSON was originally found to have a {} but an issue occurred.",
            param_type
        );
        FlowError::new(message.as_str(), 0, 0)
    }
}

#[derive(Debug, Clone)]
pub struct FlowError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl FlowError {
    pub fn new(message: &str, line: usize, column: usize) -> FlowError {
        FlowError {
            message: String::from(message),
            line,
            column,
        }
    }
}

impl fmt::Display for FlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

impl std::error::Error for FlowError {
    fn description(&self) -> &str {
        &self.message
    }
}
