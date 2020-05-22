use std::{convert, fmt, num};

pub struct OAuthError {
    message: String,
}

impl fmt::Debug for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl convert::From<num::ParseFloatError> for OAuthError {
    fn from(_error: num::ParseFloatError) -> Self {
        OAuthError {
            message: "Please provide valid numbers only".into(),
        }
    }
}

impl OAuthError {
    pub fn exit(message: &str) -> Result<(), OAuthError> {
        Err(OAuthError {
            message: message.into(),
        })
    }
}
