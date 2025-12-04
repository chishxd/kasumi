<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { Track } from "../lib/types";
  // import { show } from "@tauri-apps/api/app";

  let tracks = $state<Track[]>([]);
  let currentTrack = $state<Track | null>(null);
  let mousePos = $state({ x: 0, y: 0 });
  let menuTrack = $state<Track | null>(null);
  let showMenu = $state<boolean>(false);

  let isPaused = $state(false);

  function handleContext(event: MouseEvent, track: Track) {
    event.preventDefault();
    menuTrack = track;
    mousePos.x = event.clientX;
    mousePos.y = event.clientY;
    showMenu = true;
    console.log(`Clicked at: ${mousePos.x},${mousePos.y}`);
  }

  async function playAudio(song: Track) {
    console.log("Attempting to play: ", song.path);
    console.log(song.coverArt?.slice(0, 100));
    try {
      await invoke("play_audio", { track: song });
      currentTrack = await invoke("get_current_track");
      isPaused = await invoke("is_audio_paused");
    } catch (error) {
      console.error("Rust error:", error);
      alert("Error:" + error);
    }
  }

  async function pauseAudio() {
    console.log("MUSIC PAUSED.");
    try {
      await invoke("pause_audio");
      isPaused = await invoke("is_audio_paused");
    } catch (error) {
      console.error("Something went wrong: " + error);
    }
  }
  async function resumeAudio() {
    console.log("MUSIC PAUSED.");
    try {
      await invoke("resume_audio");
      isPaused = await invoke("is_audio_paused");
    } catch (error) {
      console.error("Something went wrong: " + error);
    }
  }

  onMount(async () => {
    console.log("Scanning Music Directory...");
    try {
      tracks = await invoke("get_library_tracks");
      currentTrack = await invoke("get_current_track");
      isPaused = await invoke("is_audio_paused");
    } catch (error) {
      console.error("Ooopsies... Something went wrong: ", error);
    }
  });
</script>

<main class="container">
  <header>
    <p>Welcome to Kasumi</p>
  </header>

  <div class="track-scroll">
    <div class="track-grid">
      {#each tracks as track}
        <button
          class="track-card"
          onclick={() => {
            playAudio(track);
          }}
          oncontextmenu={(e) => {
            handleContext(e, track);
          }}
        >
          {#if track.coverArt}
            <img src={track.coverArt} alt={track.title} />
          {:else}
            <div class="placeholder">🎵</div>
          {/if}
          <!-- <p>{track.coverArt?.slice(0, 50)}</p> -->
          <div class="overlay">
            <p class="track-title">{track.title}</p>
            <p class="track-artist">{track.artist}</p>
          </div>
        </button>
      {/each}
    </div>
  </div>

  <div class="player-bar">
    <div class="now-playing">
      <p class="now-title">{currentTrack?.title || "No Music"}</p>
      <p class="now-artist">{currentTrack?.artist || "No artist"}</p>
    </div>

    <div class="controls">
      {#if isPaused}
        <button onclick={resumeAudio} class="player-btn">PLAY</button>
      {:else}
        <button onclick={pauseAudio} class="player-btn">PAUSE</button>
      {/if}
    </div>
  </div>
</main>

{#if showMenu}
  <div
    class="context-menu"
    style="position: absolute; top: {mousePos.y}px; left: {mousePos.x}px;"
  >
    Add to queue
  </div>
{/if}

<style>
  .context-menu {
    position: fixed;
    z-index: 9999;
    padding: 6px;
    background: rgba(30, 30, 30, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
    backdrop-filter: blur(10px);
    border-radius: 8px;
    color: white;
    min-width: 180px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .player-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;

    padding: 8px 14px;
    border-top: 1px solid rgba(255, 255, 255, 0.25);
    font-size: 0.9rem;
    color: rgba(255, 255, 255, 0.9);
  }

  .now-playing {
    display: flex;
    align-items: center;
    gap: 8px;
    min-width: 0;
  }

  .now-title {
    font-size: 0.9rem;
    font-weight: 600;
  }

  .now-artist {
    font-size: 0.75rem;
    opacity: 0.7;
  }

  .player-btn {
    padding: 6px 14px;
    border-radius: 999px;
    border: 1px solid rgba(255, 255, 255, 0.4);
    background-color: rgba(0, 0, 0, 0.3);
    color: #f5f5f5;
    font-size: 0.85rem;
    cursor: pointer;
    transition:
      transform 0.15s ease,
      background 0.15s ease,
      box-shadown 0.15s ease;
  }

  .player-btn:hover {
    transform: translateY(-2px) scale(1.03);
    background-color: rgba(255, 255, 255, 0.15);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.25);
  }

  .track-scroll {
    flex: 1;
    overflow-x: hidden;
    overflow-y: auto;
    padding: 20px;
    z-index: 0;
  }
  .track-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 24px;
  }

  .track-card {
    all: unset;
    position: relative;
    border-radius: 16px;
    overflow: hidden;
    transition:
      transform 0.15s ease,
      box-shadow 0.15s ease;
  }

  .track-card:hover {
    transform: translate(-3px) scale(1.02);
    /* box-shadow: 0 8px 18px rgba(0,0,0,0.25); */
  }
  .track-grid img {
    height: 100%;
    width: 100%;
    object-fit: cover;
    display: block;
    pointer-events: none;
  }

  .overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    padding: 8px 10px;
    background: linear-gradient(to top, rgba(0, 0, 0, 0.7), rgba(0, 0, 0, 0));

    color: white;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    align-items: start;
  }

  .track-title {
    font-size: 1.05rem;
    font-weight: 600;
    margin: 0;
  }
  .track-artist {
    opacity: 0.8;
    font-size: 0.85rem;
    font-weight: 300;
    margin-top: 2px;
  }

  .container {
    background: rgba(255, 255, 255, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(10px);
    text-align: center;
    border-radius: 16px;
    margin: 20px;

    display: flex;
    flex-direction: column;
    height: calc(100vh - 40px);
    overflow: hidden;
  }

  :global(body) {
    background-color: transparent !important;
    margin: 0;
    font-family: sans-serif;
    overflow: hidden;
  }

  :global(html) {
    background-color: transparent !important;
  }
</style>
