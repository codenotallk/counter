use super::counter_input::CounterInput;

pub fn from_json (json: String) -> CounterInput {
    serde_json::from_str(&json).unwrap()
}