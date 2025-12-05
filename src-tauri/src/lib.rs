use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use std::{collections::VecDeque, path::Path};
use tauri::{Emitter, Manager};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

use rodio::{Decoder, OutputStreamBuilder, Sink};

use base64::prelude::{Engine as _, BASE64_STANDARD};
use lofty::picture::MimeType;
use lofty::prelude::*;
use lofty::probe::Probe;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Track {
    path: String,
    title: String,
    artist: String,
    album: String,
    duration_seconds: u64,
    cover_art: Option<String>,
}

pub struct AudioState {
    sink: Mutex<Sink>,
    current_track: Mutex<Option<Track>>,
    queue: Mutex<VecDeque<Track>>,
}

//A HELPER FUNCTION
fn read_track_metadata(path_str: &str) -> Option<Track> {
    let path = Path::new(path_str);

    let tagged_file = Probe::open(path).ok()?.read().ok()?;

    let tag = tagged_file.primary_tag()?;

    let title = tag
        .title()
        .as_deref()
        .unwrap_or("Unknown Title")
        .to_string();
    let artist = tag
        .artist()
        .as_deref()
        .unwrap_or("Unknown Artist")
        .to_string();
    let album = tag
        .album()
        .as_deref()
        .unwrap_or("Unknown Album")
        .to_string();

    let properties = tagged_file.properties();
    let duration_seconds = properties.duration().as_secs();

    let cover_art = tag.pictures().first().map(|pic| {
        let b64 = BASE64_STANDARD.encode(pic.data());
        let mime = pic.mime_type().unwrap_or(&MimeType::Jpeg);

        format!("data:{};base64,{}", mime, b64)
    });

    Some(Track {
        path: path_str.to_string(),
        title,
        artist,
        album,
        duration_seconds,
        cover_art,
    })
}

#[tauri::command]
fn queue_add(track: Track, state: State<'_, AudioState>) -> Result<(), String> {
    // let sink = state.sink.lock().map_err(|_| "Failed to lock sink")?;
    let mut queue = state.queue.lock().map_err(|_| "Failed to lock queue")?;
    queue.push_back(track);
    Ok(())
}

#[tauri::command]
fn queue_skip(state: State<'_, AudioState>) -> Result<Track, String> {
    let mut queue = state.queue.lock().map_err(|_| "Failed to lock queue")?;
    let next_track = match queue.pop_front() {
        Some(t) => t,
        None => return Err("Empty queue".into()),
    };
    drop(queue);
    {
        let mut current = state
            .current_track
            .lock()
            .map_err(|_| "Failed to lock current_track")?;
        *current = Some(next_track.clone());
    }

    play_audio_internal(&next_track, &state)?;

    Ok(next_track)
}

#[tauri::command]
fn get_library_tracks(path: String) -> Vec<Track> {
    let mut tracks = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_file() {
                if let Some(path_str) = path.to_str() {
                    if let Some(track) = read_track_metadata(path_str) {
                        tracks.push(track);
                    }
                }
            }
        }
    }

    tracks
}

fn play_audio_internal(track: &Track, state: &AudioState) -> Result<(), String> {
    let sink = state.sink.lock().map_err(|_| "Failed to lock sink")?;

    sink.stop();

    let file = File::open(&track.path).map_err(|e| format!("File not found: {}", e))?;
    let reader = BufReader::new(file);

    let source = Decoder::new(reader).map_err(|e| format!("Codec error: {}", e))?;

    sink.append(source);
    sink.play();

    Ok(())
}

#[tauri::command]
fn play_audio(track: Track, state: State<'_, AudioState>) -> Result<(), String> {
    {
        let mut queue = state.queue.lock().map_err(|_| "Failed to lock queue")?;
        queue.clear();
    }
    {
        let mut current = state
            .current_track
            .lock()
            .map_err(|_| "Failed to lock current_track")?;
        *current = Some(track.clone());
    }

    play_audio_internal(&track, &state)
}

#[tauri::command]
fn get_current_track(state: State<'_, AudioState>) -> Option<Track> {
    let current = state.current_track.lock().unwrap();
    current.clone()
}

#[tauri::command]
fn pause_audio(state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().map_err(|_| "Failed to lock sink")?;
    sink.pause();

    Ok(())
}

#[tauri::command]
fn resume_audio(state: State<'_, AudioState>) -> Result<(), String> {
    let sink = state.sink.lock().map_err(|_| "Failed to lock sink")?;
    sink.play();

    Ok(())
}

#[tauri::command]
fn is_audio_paused(state: State<'_, AudioState>) -> bool {
    let sink = state.sink.lock().map_err(|_| "Failed to lock sink.");

    let is_playing = sink.expect("Could not get track info").is_paused();
    return is_playing;
}

fn start_autostart_loop(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        loop {
            {
                let state = app_handle.state::<AudioState>();
                let sink = state.sink.lock().unwrap();
                if sink.empty() {
                    drop(sink);
                    let mut queue = state.queue.lock().unwrap();
                    if let Some(next_track) = queue.pop_front() {
                        drop(queue);
                        {
                            let mut current = state.current_track.lock().unwrap();
                            *current = Some(next_track.clone());
                        }

                        let _ = play_audio_internal(&next_track, &state);
                        let _ = app_handle.emit("autoplay_next", next_track);
                    }
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let stream = OutputStreamBuilder::open_default_stream().unwrap();

    let stream_ptr = Box::leak(Box::new(stream));

    let sink = Sink::connect_new(&stream_ptr.mixer());

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(AudioState {
            sink: Mutex::new(sink),
            current_track: Mutex::new(None),
            queue: Mutex::new(VecDeque::new()),
        })
        .setup(|app| {
            start_autostart_loop(app.handle().clone());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            play_audio,
            pause_audio,
            resume_audio,
            get_library_tracks,
            is_audio_paused,
            get_current_track,
            queue_add,
            queue_skip,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
