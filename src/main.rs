mod keycode_map;
#[tokio::main]
async fn main() {
  // Initialize all devices that emit `key` events.
  let keyboards = get_keyboards();
  // Create a futures vector to await so we do not end the program.
  let mut futures: Vec<tokio::task::JoinHandle<()>> = Vec::new();
  // Spawn an async process for each device, so we can listen on them all.
  for device in keyboards.into_iter() {
    futures.push(tokio::spawn(async {
      // Initialize a HashTable<keycode, keyvalue>, and initialize a device and get an async event stream for the device.
      let keycode_map = keycode_map::initialize_evdev_keycode_hashmap();
      // Setup an async stream, if an error occurs, we end the thread.
      let event_stream_result = device.into_event_stream();
      match event_stream_result {
        Ok(mut event_stream) => {
          // Event loop, capture global key presses from this device.
          loop {
            let input = event_stream.next_event().await.unwrap();
            match input.event_type() {
              evdev::EventType::KEY => {
                // When an input happens, input.value() returns 1 for a press, 0 for a release, and 2 for a hold.
                match input.value() {
                  1 => println!("{}", keycode_map.get(&input.code()).unwrap_or(&String::from(""))),
                  2 => println!("{}", keycode_map.get(&input.code()).unwrap_or(&String::from(""))),
                  _ => {}
                }
              },
              _ => {}
            }
          }
        },
        Err(error) => println!("{}", error)
      }
    }));
  }
  for future in futures.into_iter() {
    future.await.unwrap();
  }
}

// Find all devices that emit evdev `KEY` events, and group them into a vector.
fn get_keyboards() -> Vec<evdev::Device> {
  let mut devices: Vec<evdev::Device> = Vec::new();
  for (path, device) in evdev::enumerate() {
    if device.supported_events().contains(evdev::EventType::KEY) {
      let device_result = evdev::Device::open(path);
      match device_result {
        Ok(device) => devices.push(device),
        Err(error) => println!("{}", error)
      }
    }
  }
  devices
}