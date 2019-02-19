use std::sync::mpsc::{Sender,Receiver};
use system_admin_lib::messages::*;

pub trait SystemAdminModule {
    fn accept_channel(&self, tx: Sender<Message>);
    fn command_list(&self) -> Vec<String>;
    fn run(&self, message : &Message) -> Message;
}