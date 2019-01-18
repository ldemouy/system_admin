use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::rc::Rc;

use system_admin_lib::messages::*;
use system_admin_modules::*;
use system_admin_modules::traits::*;


pub struct Controller {
    pub main_tx : Sender<Message>,
    pub sender: Sender<Message>,
    receiver: Receiver<Message>,
    modules: Vec<Rc<SystemAdminModule>>

}

impl Controller {
    pub fn new (main_tx :Sender<Message>) -> Controller {
        let (sender, receiver) : (Sender<Message>, Receiver<Message>) = mpsc::channel();
        Controller{main_tx, sender, receiver, modules: vec![]}
    }
    pub fn run (&self) {
        loop {
            let message = self.receiver.recv().expect("channel error");
            println!("{:?}", message);
        }
    }
}