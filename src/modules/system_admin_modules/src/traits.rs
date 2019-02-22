use system_admin_lib::messages::*;

pub trait SystemAdminModule {
    fn command_list(&self) -> Vec<&str>;
    fn name(&self) -> &str;
    fn responds_to(&self) -> Vec<MessageType>;
    fn run(&self, message : &Message) -> Message;
}