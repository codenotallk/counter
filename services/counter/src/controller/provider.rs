use super::counter_output::CounterOutput;



pub fn convert_to_json (output: &CounterOutput) -> String {
    serde_json::to_string(output).unwrap()
}