use std;
use std::fmt;

pub enum FlowErrorType {
    MissingParam = 0,
    AllowReset = 1,
    InvalidAccessCode = 2,
    MissingAccessCode = 3,
    RequiresGrantType = 4,
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
        }
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
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({}:{})", self.message, self.line, self.column)
    }
}

impl std::error::Error for FlowError {
    fn description(&self) -> &str {
        &self.message
    }
}
