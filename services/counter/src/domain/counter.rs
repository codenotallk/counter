
#[derive(Clone)]
pub struct Counter {
    value: u64,
}

impl Counter {
    pub fn new () -> Self {
        Self {
            value: 0
        }
    }

    pub fn add (&mut self, step: u64) {
        self.value += step
    }

    pub fn get_value (&self) -> u64 {
        self.value
    }
}