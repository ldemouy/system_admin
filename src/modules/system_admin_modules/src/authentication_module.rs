use crate::traits::SystemAdminModule;
use guid_create::GUID;
use system_admin_lib::messages::{Message, MessageType, PermissionLevel, ProtocolType};
use pam::*;
use sqlite::*;

struct AuthenticationModule{}
impl SystemAdminModule for AuthenticationModule {
    fn command_list(&self) -> Vec<&str> {
        vec!["Authenticate"]
    }
    fn name (&self) -> &str {
        "AuthenticationModule"
    }
    fn responds_to(&self) -> Vec<MessageType> {
        vec![MessageType::Authenticate{username : String::new(), password : String::new()}]
    }

    fn run(&self, message: &Message) -> Message {
        match &message.message_type {
            MessageType::Authenticate {username, password} => {
                let service = "system_admin";
                let mut auth = pam::Authenticator::with_password(service).unwrap();
                auth.get_handler().set_credentials(username.clone(), password.clone());
                if auth.authenticate().is_ok() {
                    let connection = sqlite::open(":memory:").unwrap();
                    let guid = GUID::rand();
                    connection.execute(format!("CREATE TABLE IF NOT EXISTS authenticated_users (username TEXT PRIMARY KEY, auth_token TEXT);\
                    INSERT OR REPLACE INTO authenticated_user(username, auth_token) VALUES({},{})",username,guid.to_string())).unwrap();

                    return Message::new(MessageType::AuthenticationSuccess, PermissionLevel::Internal, message.protocol.clone(), message.origin.clone())
                }
            }
            _ => {}
        }
        Message::new(MessageType::AuthenticationFail, PermissionLevel::Internal, message.protocol.clone(), message.origin.clone())
    }
}