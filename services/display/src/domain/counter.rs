
#[derive(Clone)]
pub struct Counter {
    value: u64,
}

impl Counter {
    pub fn new (value: u64) -> Self {
        Self {
            value
        }
    }

    pub fn get_value (&self) -> u64 {
        self.value
    }
    
    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }
}