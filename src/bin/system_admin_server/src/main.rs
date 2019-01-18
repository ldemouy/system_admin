use system_admin_server_lib::controller::*;
use system_admin_lib::messages::*;
use std::thread;
use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;
use std::os::unix::net::{UnixListener,UnixStream};
use std::io::prelude::*;
use std::fs;
use std::str;
use bincode::deserialize;

fn handle_client(mut stream : UnixStream, tx : Sender<Message>) {
    let mut response = String::new();
    let mut buffer = vec![];
    stream.read_to_end(&mut buffer);
    let message : Message = deserialize(&buffer).expect("couldn't deserialize message");
    tx.send(message);

    stream.write_all(b"Response").expect("couldn't write to socket");
}

fn main() {
    println!("system_admin_server starting");
    let (main_tx, main_rx) = mpsc::channel();
    let controller = Controller::new(main_tx);
    let tx = controller.sender.clone();
    thread::spawn(move || controller.run());

    fs::remove_file("/tmp/system_admin_server.socket");

    let listener = UnixListener::bind("/tmp/system_admin_server.socket").expect("Unable to create socket");

    for stream in listener.incoming() {
        let tx_clone = tx.clone();
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream, tx_clone));
            }
            Err(stream) => {
                println!("Error on Unix Socket");
                break;
            }
        }
    }
}
