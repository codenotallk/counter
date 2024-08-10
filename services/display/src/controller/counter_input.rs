use serde::{Deserialize, Serialize};

use crate::domain::counter::Counter;


#[derive(Serialize, Deserialize)]
pub struct CounterInput {
    value: u64,
}

impl Into <Counter> for CounterInput {
    fn into(self) -> Counter {
        Counter::new(self.value)
    }
}
