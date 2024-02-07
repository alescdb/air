use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ErrorMessage {
    message: String,
}

#[allow(dead_code)]
impl ErrorMessage {
    pub fn new(message: &str) -> ErrorMessage {
        ErrorMessage {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ErrorMessage {}
