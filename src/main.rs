mod keycode_map;
#[tokio::main]
async fn main() {
  let keycode_map = keycode_map::initialize_evdev_keycode_hashmap();
  let mut event_stream = initialize_event_stream(initialize_keyboard());
  loop {
    let input = event_stream.next_event().await.unwrap();
    match input.event_type() {
      evdev::EventType::KEY => {
        println!("{}", keycode_map.get(&input.code()).unwrap_or(&String::from("INVALID_CODE")))
      },
      _ => {}
    }
  }
}

fn initialize_keyboard() -> evdev::Device {
  evdev::Device::open("/dev/input/event7").unwrap()
}

fn initialize_event_stream(device: evdev::Device) -> evdev::EventStream {
  device.into_event_stream().unwrap()
}