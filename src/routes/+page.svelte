<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { Track } from "../lib/types";

  let tracks = $state<Track[]>([]);
  let isPlaying = $state(false);

  async function playAudio(song: Track) {
    console.log("Attempting to play: ", song.path);
    console.log(song.coverArt?.slice(0, 100));
    try {
      await invoke("play_audio", { path: song.path });
      isPlaying = true;
    } catch (error) {
      console.error("Rust error:", error);
      alert("Error:" + error);
    }
  }

  async function pauseAudio() {
    console.log("MUSIC PAUSED.");
    try {
      await invoke ("pause_audio");
      isPlaying = false;
    } catch (error) {
      console.error("Something went wrong: " + error);
    }
  }
  async function resumeAudio() {
    console.log("MUSIC PAUSED.");
    try {
      await invoke("resume_audio");
      isPlaying = true;
    } catch (error) {
      console.error("Something went wrong: " + error);
    }
  }


  onMount(async () => {
    console.log("Scanning Music Directory...");
    try {
      tracks = await invoke("get_library_tracks");
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
      <p class="now-title">Song</p>
      <p class="now-artist">Gian</p>
    </div>

    <div class="controls">
      {#if isPlaying}
        <button onclick={pauseAudio}>Pause Music</button>
        {:else}
         <button onclick={resumeAudio}>Resume Music</button>
      {/if}
         </div>

  </div>
</main>

<style>
  .player-bar{
    display: flex;
    align-items: center;
    justify-content: space-between;

    padding: 8px 14px;
    border-top: 1px solid rgba(255, 255, 255, 0.25);
    font-size: 0.9rem;
    color: rgba(255,255,255,0.9);
  }

  .now-playing{
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

  .track-scroll {
    flex: 1;
    overflow-x: hidden;
    overflow-y: auto;
    padding: 20px;
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

  .track-grid img {
    height: 100%;
    width: 100%;
    object-fit: cover;
    display: block;
  }

  .overlay {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    padding: 8px 10px;
    background: linear-gradient(
      to top,
      rgba(0, 0, 0, 0.7),
      rgba(0, 0, 0, 0.0)
    );

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
