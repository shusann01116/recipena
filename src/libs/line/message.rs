use line_bot_sdk_rust::line_messaging_api::models::TextMessage;

use crate::infra::line::LineMessage;

impl From<LineMessage> for line_bot_sdk_rust::line_messaging_api::models::Message {
    fn from(value: LineMessage) -> Self {
        match value {
            LineMessage::Text(message) => {
                line_bot_sdk_rust::line_messaging_api::models::Message::Text(TextMessage::new(
                    message,
                ))
            }
        }
    }
}
