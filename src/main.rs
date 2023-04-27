mod keycode_map;

#[tokio::main]
async fn main() {
  // Initialize a HashTable<keycode, keyvalue>, and initialize a device and get an async event stream for the device.
  let keycode_map = keycode_map::initialize_evdev_keycode_hashmap();
  let mut event_stream = initialize_event_stream(initialize_keyboard());
  // Event loop, capture global key presses.
  loop {
    let input = event_stream.next_event().await.unwrap();
    match input.event_type() {
      evdev::EventType::KEY => {
        // When an input happens, input.value() returns 1 for a press, 0 for a release, and 2 for a hold.
        match input.value() {
          1 => println!("{}", keycode_map.get(&input.code()).unwrap_or(&String::from("INVALID_CODE"))),
          2 => println!("{}", keycode_map.get(&input.code()).unwrap_or(&String::from("INVALID_CODE"))),
          _ => {}
        }
      },
      _ => {}
    }
  }
}

/**
 * Initialization Functions
 */
fn initialize_keyboard() -> evdev::Device {
  evdev::Device::open("/dev/input/event7").unwrap()
}

fn initialize_event_stream(device: evdev::Device) -> evdev::EventStream {
  device.into_event_stream().unwrap()
}