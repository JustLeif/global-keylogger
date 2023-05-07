/**
 * Represents a key press event. Contains the key pressed, and the timestamp of the key press.
 */
#[derive(serde::Serialize)]
pub struct KeyPress {
    key: String,
    timestamp_millis: u128,
    // Implement shift, ctrl, alt as optional values on this struct. To see if a key press had combinations of these keys pressed.
}
impl KeyPress {
    pub fn new(key: String, timestamp_millis: u128) -> KeyPress {
        KeyPress {
            key,
            timestamp_millis,
        }
    }
    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_error) => String::from("JSON serialization error."),
        }
    }
}