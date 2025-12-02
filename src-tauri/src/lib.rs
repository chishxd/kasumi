use lofty::{Accessor, Probe, TaggedFileExt};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use serde::Serialize;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all="camelCase")]
struct Track{
    path: String,
    title: String,
    artist: String,
    album: String,
    duration_seconds: u64,
    cover_art: Option<String>,
}

struct AudioState {
    sink: Mutex<Sink>,
}

#[tauri::command]
fn play_local_music(path: String, state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().map_err(|_| "Failed to lock sink")?;

    let file = File::open(&path).map_err(|e| format!("File not found: {}", e))?;
    let reader = BufReader::new(file);

    let source = Decoder::new(reader).map_err(|e| format!("Codec error: {}", e))?;

    sink.append(source);
    sink.play();

    Ok(())
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
        .manage(AudioState {
            sink: Mutex::new(sink),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, play_local_music])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
