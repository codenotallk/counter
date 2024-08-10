use crate::domain::counter::Counter;

pub struct CounterRepresentation {
    value: u64,
}

impl From<Counter> for CounterRepresentation {
    fn from(value: Counter) -> Self {
        Self {
            value: value.get_value(),
        }
    }
}

impl CounterRepresentation {
    pub fn get_value (&self) -> u64 {
        self.value
    }
}