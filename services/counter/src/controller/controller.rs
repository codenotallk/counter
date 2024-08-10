use std::{net::UdpSocket, thread, time::Duration};

use crate::domain::counter::Counter;

use super::{counter_output::CounterOutput, provider::convert_to_json};



pub struct Controller {
    run: bool,
    socket: UdpSocket,
    destination: String,
    counter: Counter,
}

impl Controller {
    pub fn new (address: &str, destination: &str) -> Self {
        Self {
            run: true,
            socket: UdpSocket::bind(address).unwrap(),
            destination: destination.to_owned(),
            counter: Counter::new(),
        }
    }

    pub fn run (&mut self) {
        
        while self.run {

            let counter_output = CounterOutput::from(self.counter.clone());
            let json = convert_to_json(&counter_output);

            match self.socket.send_to(json.as_bytes(), &self.destination) {
                Ok(_) => {
                    self.counter.add(1);

                    thread::sleep(Duration::from_millis(500));
                },
                Err(_) => eprintln!("Couldn't send the message to {}", self.destination),
            }
        }
    }
}