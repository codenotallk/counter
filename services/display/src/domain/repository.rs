use super::counter::Counter;


pub struct Repository {
    counter: Counter,
}

impl Repository {
    pub fn new () -> Self {
        Self {
            counter: Counter::new(0),
        }
    }

    pub fn update (&mut self, counter: Counter) {
        self.counter.set_value (counter.get_value());
    }

    pub fn get (&self) -> Counter {
        self.counter.clone ()
    }
}