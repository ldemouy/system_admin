use std::sync::mpsc::{Sender,Receiver};
use system_admin_lib::messages::*;

pub trait SystemAdminModule {
    fn accept_channel(tx: Sender<Message>);
    fn command_list() -> Vec<String>;
    fn run(message : Message);

}