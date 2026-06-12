use rdev::{listen, Event, EventType, Key};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct KeyboardEventData {
    key: String,
    event_type: String,
    is_double_tap: bool,
    press_count: u32,
}

fn key_to_string(key: Key) -> String {
    let name = format!("{:?}", key);
    match name.as_str() {
        "Return" => "Enter".to_string(),
        s if s.starts_with("Key") => s[3..].to_string(),
        s if s.starts_with("Num") => s[3..].to_string(),
        _ => name,
    }
}

struct KeyPressState {
    last_press_time: Instant,
    last_release_time: Option<Instant>,
    press_count: u32,
}

lazy_static::lazy_static! {
    static ref KEY_PRESS_STATES: Mutex<HashMap<String, KeyPressState>> = Mutex::new(HashMap::new());
}

const DOUBLE_TAP_THRESHOLD: Duration = Duration::from_millis(300);

fn check_double_tap(key: &str) -> (bool, u32) {
    let mut states = KEY_PRESS_STATES.lock().unwrap();
    let now = Instant::now();

    let result = if let Some(state) = states.get_mut(key) {
        if let Some(release_time) = state.last_release_time {
            let time_since_last_release = now.duration_since(release_time);
            if time_since_last_release <= DOUBLE_TAP_THRESHOLD {
                state.press_count += 1;
                state.last_press_time = now;
                state.last_release_time = None;
                (state.press_count >= 2, state.press_count)
            } else {
                state.press_count = 1;
                state.last_press_time = now;
                state.last_release_time = None;
                (false, 1)
            }
        } else {
            let is_double = state.press_count >= 2;
            (is_double, state.press_count)
        }
    } else {
        states.insert(
            key.to_string(),
            KeyPressState {
                last_press_time: now,
                last_release_time: None,
                press_count: 1,
            },
        );
        (false, 1)
    };

    states.retain(|_, state| {
        if let Some(release_time) = state.last_release_time {
            now.duration_since(release_time) <= DOUBLE_TAP_THRESHOLD * 3
        } else {
            true
        }
    });

    result
}

fn reset_key_state(key: &str) {
    let mut states = KEY_PRESS_STATES.lock().unwrap();
    if let Some(state) = states.get_mut(key) {
        state.last_release_time = Some(Instant::now());
    }
}

pub fn start_keyboard_listener(app: &AppHandle) {
    let app_clone = app.clone();

    std::thread::spawn(move || {
        listen(move |event: Event| match event.event_type {
            EventType::KeyPress(key) => {
                let key_str = key_to_string(key);
                let (is_double_tap, press_count) = check_double_tap(&key_str);

                let event_data = KeyboardEventData {
                    key: key_str.clone(),
                    event_type: "keydown".to_string(),
                    is_double_tap,
                    press_count,
                };

                if let Some(window) = app_clone.get_webview_window("main") {
                    println!(
                        "Key pressed: {} (double_tap: {}, count: {})",
                        key_str, is_double_tap, press_count
                    );
                    let result = window.emit("keyboard-event", &event_data);
                    if result.is_err() {
                        println!("Failed to emit keydown event");
                    }
                } else {
                    println!("Main window not found");
                }
            }
            EventType::KeyRelease(key) => {
                let key_str = key_to_string(key);
                reset_key_state(&key_str);

                let event_data = KeyboardEventData {
                    key: key_str.clone(),
                    event_type: "keyup".to_string(),
                    is_double_tap: false,
                    press_count: 1,
                };

                if let Some(window) = app_clone.get_webview_window("main") {
                    println!("Key released: {}", key_str);
                    let result = window.emit("keyboard-event", &event_data);
                    if result.is_err() {
                        println!("Failed to emit keyup event");
                    }
                } else {
                    println!("Main window not found");
                }
            }
            _ => {}
        })
        .unwrap();
    });
}
