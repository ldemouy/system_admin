use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use system_admin_lib::messages::*;
use system_admin_modules::*;
use system_admin_modules::traits::*;


pub struct Controller {
    pub sender: Sender<(Message, Sender<Message>)>,
    receiver: Receiver<(Message, Sender<Message>)>,
    modules: Vec<Arc<SystemAdminModule + Send + Sync>>

}

impl Controller {
    pub fn new () -> Controller {
        let (sender, receiver) : (Sender<(Message, Sender<Message>)>, Receiver<(Message, Sender<Message>)>) = mpsc::channel();
        Controller{sender, receiver, modules: vec![]}
    }
    pub fn run (&self) {
        loop {
            let (message, tx) = self.receiver.recv().expect("channel error");
            let mut message_string = String::new();
            match &message.message_type {
                MessageType::Message{message} => message_string = message.clone(),
                _ => {}
            }
            let mut iter = message_string.split_whitespace();
            let c = iter.next().unwrap().to_string();
            let commands = self.modules.iter().filter(|module| module.command_list().iter().filter(|command| *command == &c).count() > 0 );
            for command in commands {
                tx.send(command.run(&message));
            }
            let send_message = Message::new(MessageType::Message {message:"Response".to_string()}, PermissionLevel::Internal, ProtocolType::PlainText, "localhost".to_string());
            tx.send(send_message);
        }
    }
}