use std::{sync::{Arc, Mutex}, thread, time::Duration};

use crate::domain::repository::Repository;

use super::{counter_representation::CounterRepresentation, view::CLI};


pub struct Presenter {
    run: bool, 
    repository: Arc<Mutex<Repository>>, 
}

impl Presenter {
    pub fn new (repository: Arc<Mutex<Repository>>) -> Self {
        Self {
            run: true,
            repository,
        }
    }

    pub fn run (&self) {

        while self.run {
            // match self.receiver.recv() {
            //     Ok (counter) => {
            //         CLI::show(CounterRepresentation::from(counter));
            //     }, 
            //     Err (err) => {
            //         panic!("Error in the channel: {}", err);
            //     }
            // }

            let repository = self.repository.lock().unwrap();
            let counter = repository.get();

            CLI::show(CounterRepresentation::from(counter));

            drop (repository);

            thread::sleep(Duration::from_millis(500));

        }
    }
}