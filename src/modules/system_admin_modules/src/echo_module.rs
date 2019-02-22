use crate::traits::SystemAdminModule;
use system_admin_lib::messages::Message;
use system_admin_lib::messages::MessageType;
use system_admin_lib::messages::PermissionLevel;
use system_admin_lib::messages::ProtocolType;


pub struct EchoModule {
 }
impl EchoModule {
    pub fn new() -> EchoModule {
        EchoModule{}
    }
}

impl SystemAdminModule for EchoModule {
    fn command_list(&self) -> Vec<&str> {
        vec!["echo"]
    }
    fn name(&self) -> &str {
        "EchoModule"
    }

    fn responds_to(&self) -> Vec<MessageType> {
        vec![MessageType::Action{message : String::new()}, MessageType::Message{message : String::new()}]
    }

    fn run(&self, message: &Message) -> Message {
        match &message.message_type {
            MessageType::Message {message} => {
                let message = message.clone();
                Message::new(MessageType::Message{message}, PermissionLevel::Internal, ProtocolType::PlainText, "localhost".to_string())
            },
            _ => {Message::new(MessageType::Acknowledge, PermissionLevel::Internal, ProtocolType::PlainText, "localhost".to_string())}
        }

    }
}