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
    history: Mutex<Vec<Track>>,
    progress: Mutex<u64>,
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
        let mut history = state.history.lock().unwrap();

        if let Some(prev) = current.clone() {
            history.push(prev);
        }
        *current = Some(next_track.clone());
    }

    play_audio_internal(&next_track, &state)?;

    Ok(next_track)
}

#[tauri::command]
fn play_previous(state: State<'_, AudioState>) -> Result<Track, String> {
    let mut history = state.history.lock().map_err(|_| "Failed to lock history")?;

    let previous = match history.pop() {
        Some(t) => t,
        None => return Err("No Previous Track".into()),
    };
    {
        let mut current = state.current_track.lock().unwrap();
        *current = Some(previous.clone());
    }

    play_audio_internal(&previous, &state)?;

    Ok(previous)
}

#[tauri::command]
fn get_library_tracks(path: String) -> Vec<Track> {
    let mut tracks = Vec::new();
    let base = Path::new(&path);

    collect_tracks(base, &mut tracks);

    // if let Ok(entries) = std::fs::read_dir(&path) {
    //     for entry in entries.flatten() {
    //         let path = entry.path();
    //
    //         if path.is_file() {
    //             if let Some(path_str) = path.to_str() {
    //                 if let Some(track) = read_track_metadata(path_str) {
    //                     tracks.push(track);
    //                 }
    //             }
    //         }
    //     }
    // }

    tracks
}

fn play_audio_internal(track: &Track, state: &AudioState) -> Result<(), String> {
    let sink = state.sink.lock().map_err(|_| "Failed to lock sink")?;

    sink.stop();

    let file = File::open(&track.path).map_err(|e| format!("File not found: {}", e))?;
    let reader = BufReader::new(file);

    let source = Decoder::new(reader).map_err(|e| format!("Codec error: {}", e))?;

    {
        let mut p = state.progress.lock().unwrap();
        *p = 0;
    }

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
        let mut history = state.history.lock().unwrap();

        if let Some(prev) = current.clone() {
            history.push(prev);
        }
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

                        {
                            let mut p = state.progress.lock().unwrap();
                            *p = 0;
                        }

                        let _ = play_audio_internal(&next_track, &state);
                        let _ = app_handle.emit("autoplay_next", next_track).unwrap();
                    } else {
                        app_handle.emit("playback_ended", ()).unwrap();
                    }
                } else {
                    {
                        let mut p = state.progress.lock().unwrap();
                        *p += 1;
                    }

                    let current = {
                        let p = state.progress.lock().unwrap();
                        *p
                    };

                    let duration = {
                        let c = state.current_track.lock().unwrap();
                        c.as_ref().map(|t| t.duration_seconds).unwrap_or(0);
                    };

                    let _ = app_handle.emit(
                        "playback_progress",
                        serde_json::json!({
                            "current": current,
                            "duration": duration
                        }),
                    );
                }
            }
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
    });
}

fn collect_tracks(path: &Path, out: &mut Vec<Track>) {
    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let p = entry.path();

        if p.is_dir() {
            if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                if name.starts_with('.') {
                    continue;
                }
            }
            collect_tracks(&p, out);
        } else if p.is_file() {
            if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();

                if ["mp3", "flac", "wav", "ogg", "m4a"].contains(&ext_lower.as_str()) {
                    if let Some(path_str) = p.to_str() {
                        if let Some(track) = read_track_metadata(path_str) {
                            out.push(track);
                        }
                    }
                }
            }
        }
    }
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
            history: Mutex::new(Vec::new()),
            progress: Mutex::new(0),
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
            play_previous,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
