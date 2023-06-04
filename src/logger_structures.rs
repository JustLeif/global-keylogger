/**
 * Represents a key press event. Contains the key pressed, and the timestamp of the key press.
 */
pub struct KeyPress {
    pub key: String,
    pub timestamp_millis: u64, // TODO! Implement shift, ctrl, alt as optional values on this struct. To see if a key press had combinations of these keys pressed.
    pub session_id: String,
}
impl KeyPress {
    pub fn new(key: String, timestamp_millis: u64, session_id: String) -> KeyPress {
        KeyPress {
            key,
            timestamp_millis,
            session_id,
        }
    }
}
