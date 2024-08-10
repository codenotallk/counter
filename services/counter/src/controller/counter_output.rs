use serde::{Deserialize, Serialize};

use crate::domain::counter::Counter;


#[derive(Serialize, Deserialize)]
pub struct CounterOutput {
    value: u64,
}

impl From <Counter> for CounterOutput {
    fn from(value: Counter) -> Self {

        Self {
            value: value.get_value(),
        }
    }
}