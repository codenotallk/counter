use std::sync::mpsc::Receiver;

use crate::domain::counter::Counter;

use super::{counter_representation::CounterRepresentation, view::CLI};


pub struct Presenter {
    run: bool, 
    receiver: Receiver <Counter>,    
}

impl Presenter {
    pub fn new (receiver: Receiver<Counter>) -> Self {
        Self {
            run: true,
            receiver,
        }
    }

    pub fn run (&self) {

        while self.run {
            match self.receiver.recv() {
                Ok (counter) => {
                    CLI::show(CounterRepresentation::from(counter));
                }, 
                Err (err) => {
                    panic!("Error in the channel: {}", err);
                }
            }
        }
    }
}