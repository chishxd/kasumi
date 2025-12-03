use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

use tauri::State;
use serde::Serialize;

use rodio::{Decoder, OutputStreamBuilder, Sink};

use lofty::prelude::*;
use lofty::probe::Probe;
use lofty::picture::MimeType;
use base64::prelude::{Engine as _, BASE64_STANDARD};


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

fn read_track_metadata(path_str: &str) -> Option<Track>{
    let path = Path::new(path_str);

    let tagged_file = Probe::open(path).ok()?.read().ok()?;

    let tag = tagged_file.primary_tag()?;

    let title = tag.title().as_deref().unwrap_or("Unknown Title").to_string();
    let artist = tag.artist().as_deref().unwrap_or("Unknown Artist").to_string();
    let album = tag.album().as_deref().unwrap_or("Unknown Album").to_string();

    let properties = tagged_file.properties();
    let duration_seconds = properties.duration().as_secs();

    let cover_art = tag.pictures().first().map(|pic| {
        let b64 = BASE64_STANDARD.encode(pic.data());
        let mime = pic.mime_type().unwrap_or(&MimeType::Jpeg);

        format!("data:{};base64,{}", mime, b64)
    });

    Some(Track{
        path: path_str.to_string(),
        title,
        artist,
        album,
        duration_seconds,
        cover_art,
    })

}


#[tauri::command]
fn get_library_tracks() -> Vec<Track>{
    let music_dir = "/home/chish/Music/";
    
    let mut tracks = Vec::new();

    if let Ok(entries) = std::fs::read_dir(music_dir){
        for entry in entries.flatten(){
            let path = entry.path();

            if path.is_file(){
                if let Some(path_str) = path.to_str(){
                    if let Some(track) = read_track_metadata(path_str){
                        tracks.push(track);
                    }
                }

            }
        }
        
    }

    tracks
}

#[tauri::command]
fn play_audio(path: String, state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().map_err(|_| "Failed to lock sink")?;

    let file = File::open(&path).map_err(|e| format!("File not found: {}", e))?;
    let reader = BufReader::new(file);

    let source = Decoder::new(reader).map_err(|e| format!("Codec error: {}", e))?;

    sink.append(source);
    sink.play();

    Ok(())
}

#[tauri::command]
fn pause_audio(state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().map_err(|_| "Failed to lock sink")?;
    sink.pause();

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
        .invoke_handler(tauri::generate_handler![greet, play_audio, pause_audio,get_library_tracks])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
