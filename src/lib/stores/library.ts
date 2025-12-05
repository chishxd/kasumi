import type { Track } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

class Library {
  tracks = $state<Track[]>([]);
}

export const library = new Library();

export async function loadLibrary() {
  try {
    const libTracks = await invoke<Track[]>('get_library_tracks');
    library.tracks.splice(0, library.tracks.length, ...libTracks);
  } catch (error) {
    console.error('Failed to load library:' + error);
  }
}
