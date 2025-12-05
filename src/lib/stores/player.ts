import type { Track } from '$lib/types';
import { invoke } from '@tauri-apps/api/core';

class Player {
  currentTrack = $state<Track | null>(null);
  isPaused = $state(true);
}

export const player = new Player();

export async function initPlayerState() {
  player.currentTrack = await invoke('get_current_track');
  player.isPaused = await invoke('is_audio_paused');
}

export async function playAudio(song: Track) {
  console.log('Attempting to play: ', song.path);
  console.log(song.coverArt?.slice(0, 100));
  try {
    await invoke('play_audio', { track: song });
    player.currentTrack = await invoke('get_current_track');
    player.isPaused = await invoke('is_audio_paused');
  } catch (error) {
    console.error('Rust error:', error);
    alert('Error:' + error);
  }
}

export async function pauseAudio() {
  console.log('MUSIC PAUSED.');
  try {
    await invoke('pause_audio');
    player.isPaused = await invoke('is_audio_paused');
  } catch (error) {
    console.error('Something went wrong: ' + error);
  }
}
export async function resumeAudio() {
  console.log('MUSIC PAUSED.');
  try {
    await invoke('resume_audio');
    player.isPaused = await invoke('is_audio_paused');
  } catch (error) {
    console.error('Something went wrong: ' + error);
  }
}
