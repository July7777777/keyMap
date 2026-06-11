use rdev::{listen, Event, EventType, Key};
use tauri::{AppHandle, Emitter, Manager};

pub fn start_keyboard_listener(app: &AppHandle) {
    let app_clone = app.clone();

    std::thread::spawn(move || {
        listen(move |event: Event| {
            if let EventType::KeyPress(key) = event.event_type {
                let key_str: String = match key {
                    Key::KeyA => "A".to_string(),
                    Key::KeyB => "B".to_string(),
                    Key::KeyC => "C".to_string(),
                    Key::KeyD => "D".to_string(),
                    Key::KeyE => "E".to_string(),
                    Key::KeyF => "F".to_string(),
                    Key::KeyG => "G".to_string(),
                    Key::KeyH => "H".to_string(),
                    Key::KeyI => "I".to_string(),
                    Key::KeyJ => "J".to_string(),
                    Key::KeyK => "K".to_string(),
                    Key::KeyL => "L".to_string(),
                    Key::KeyM => "M".to_string(),
                    Key::KeyN => "N".to_string(),
                    Key::KeyO => "O".to_string(),
                    Key::KeyP => "P".to_string(),
                    Key::KeyQ => "Q".to_string(),
                    Key::KeyR => "R".to_string(),
                    Key::KeyS => "S".to_string(),
                    Key::KeyT => "T".to_string(),
                    Key::KeyU => "U".to_string(),
                    Key::KeyV => "V".to_string(),
                    Key::KeyW => "W".to_string(),
                    Key::KeyX => "X".to_string(),
                    Key::KeyY => "Y".to_string(),
                    Key::KeyZ => "Z".to_string(),
                    Key::Num1 => "1".to_string(),
                    Key::Num2 => "2".to_string(),
                    Key::Num3 => "3".to_string(),
                    Key::Num4 => "4".to_string(),
                    Key::Num5 => "5".to_string(),
                    Key::Num6 => "6".to_string(),
                    Key::Num7 => "7".to_string(),
                    Key::Num8 => "8".to_string(),
                    Key::Num9 => "9".to_string(),
                    Key::Num0 => "0".to_string(),
                    Key::Space => "Space".to_string(),
                    Key::Return => "Enter".to_string(),
                    Key::Escape => "Escape".to_string(),
                    Key::Tab => "Tab".to_string(),
                    Key::Backspace => "Backspace".to_string(),
                    Key::Delete => "Delete".to_string(),
                    Key::Insert => "Insert".to_string(),
                    Key::Home => "Home".to_string(),
                    Key::End => "End".to_string(),
                    Key::PageUp => "PageUp".to_string(),
                    Key::PageDown => "PageDown".to_string(),
                    Key::F1 => "F1".to_string(),
                    Key::F2 => "F2".to_string(),
                    Key::F3 => "F3".to_string(),
                    Key::F4 => "F4".to_string(),
                    Key::F5 => "F5".to_string(),
                    Key::F6 => "F6".to_string(),
                    Key::F7 => "F7".to_string(),
                    Key::F8 => "F8".to_string(),
                    Key::F9 => "F9".to_string(),
                    Key::F10 => "F10".to_string(),
                    Key::F11 => "F11".to_string(),
                    Key::F12 => "F12".to_string(),
                    other => format!("{:?}", other),
                };

                if let Some(window) = app_clone.get_webview_window("main") {
                    println!("Key pressed: {}", key_str);
                    let result = window.emit("keyboard-event", key_str);
                    if result.is_err() {
                        println!("Failed to emit event");
                    }
                } else {
                    println!("Main window not found");
                }
            }
        })
        .unwrap();
    });
}
