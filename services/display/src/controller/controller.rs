use core::str;
use std::{net::UdpSocket, sync::mpsc::Sender};

use crate::domain::counter::Counter;

use super::provider::from_json;


pub struct Controller {
    run: bool,
    socket: UdpSocket,
    sender: Sender<Counter>
}

impl Controller {
    pub fn new (port: &str, sender: Sender<Counter>) -> Self {

        let address = format!("{}:{}" ,"127.0.0.1", port);
        
        Controller {
            run: true, 
            socket: UdpSocket::bind(address).unwrap(),
            sender,
        }
    }

    pub fn run (&self) {
        let mut buffer = [0; 100];

        while self.run {
            self.receive (&mut buffer);
        }
    }

    fn receive (&self, mut buffer: &mut [u8]) -> String {

        let (bytes, _) = self.socket.recv_from(&mut buffer).unwrap();

        let data = match str::from_utf8(&buffer [..bytes]) {
            Ok (data) => data,
            Err (_) => "Unable to convert to string",
        };

        let input = from_json(data.to_owned());

        match self.sender.send(input.into()) {
            Ok(_) => (),
            Err(e) => eprintln!("Error to send {}", e),
        }

        data.to_string()
    }
}