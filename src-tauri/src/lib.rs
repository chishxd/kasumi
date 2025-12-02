use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use tauri::State;

struct AudioState {
    sink: Mutex<Sink>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let stream = OutputStreamBuilder::open_default_stream().unwrap();

    let stream_ptr = Box::leak(Box::new(stream));

    let sink = Sink::connect_new(&stream_ptr.mixer());

    tauri::Builder::default()
        .manage(AudioState{
            sink: Mutex::new(sink)
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
