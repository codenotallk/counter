use super::counter_representation::CounterRepresentation;

pub struct CLI;

impl CLI {
    pub fn show (representation: CounterRepresentation) {
        println!("Counter {}", representation.get_value());
    }
}