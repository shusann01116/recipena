use crate::prelude::*;

pub type LineClientError = W<line_bot_sdk_rust::line_messaging_api::apis::Error>;

impl From<line_bot_sdk_rust::line_messaging_api::apis::Error> for LineClientError {
    fn from(e: line_bot_sdk_rust::line_messaging_api::apis::Error) -> Self {
        W(e)
    }
}

impl std::error::Error for LineClientError {}

impl std::fmt::Display for LineClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            line_bot_sdk_rust::line_messaging_api::apis::Error::Api(api_error) => {
                write!(f, "API error: {}", api_error.code)
            }
            line_bot_sdk_rust::line_messaging_api::apis::Error::Header(invalid_header_value) => {
                write!(f, "Header error: {invalid_header_value}")
            }
            line_bot_sdk_rust::line_messaging_api::apis::Error::Http(error) => {
                write!(f, "HTTP error: {error}")
            }
            line_bot_sdk_rust::line_messaging_api::apis::Error::Hyper(error) => {
                write!(f, "Hyper error: {error}")
            }
            line_bot_sdk_rust::line_messaging_api::apis::Error::Serde(error) => {
                write!(f, "Serde error: {error}")
            }
            line_bot_sdk_rust::line_messaging_api::apis::Error::UriError(invalid_uri) => {
                write!(f, "URI error: {invalid_uri}")
            }
        }
    }
}

impl std::fmt::Debug for LineClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LineClientError: {self}")
    }
}
