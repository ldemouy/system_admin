use std::os::unix::net::UnixStream;
use std::io;
use std::io::prelude::*;
use std::net::Shutdown;
use system_admin_lib::messages::*;
use bincode::serialize;

fn main() {
    loop {
        let mut socket = UnixStream::connect("/tmp/system_admin_server.socket").expect("unable to connect");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("failed to read line");
        //remove newline
        let buffer = &buffer[..(buffer.len()-1)];

        //create Message and pass it down the socket
        let message = Message::new(MessageType::Message{message: buffer.to_string()}, PermissionLevel::Admin, ProtocolType::PlainText, "localhost".to_string());
        let encoded : Vec<u8> = serialize(&message).unwrap();
        socket.write_all(&encoded).unwrap();

        //close the write to send to the other end
        socket.shutdown(Shutdown::Write).expect("Shutdown failure");

        //Read the response
        let mut response = String::new();
        socket.read_to_string(&mut response);

        println!("{}",response);
        socket.shutdown(Shutdown::Read).expect("Shutdown failure");
    }
}
