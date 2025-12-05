<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { Track } from "../lib/types";
  import ContextMenu from "../components/ContextMenu.svelte";
  import TrackCard from "../components/TrackCard.svelte";

  let tracks = $state<Track[]>([]);
  let currentTrack = $state<Track | null>(null);
  let mousePos = $state({ x: 0, y: 0 });
  let menuTrack = $state<Track | null>(null);
  let showMenu = $state<boolean>(false);

  let isPaused = $state(false);

  let menuElement = $state<HTMLDivElement | undefined>(undefined);

  async function handleContext(event: MouseEvent, track: Track) {
    event.preventDefault();

    menuTrack = track;
    mousePos.x = event.clientX;
    mousePos.y = event.clientY;
    showMenu = true;
    console.log(`Clicked at: ${mousePos.x},${mousePos.y}`);
  }

  $effect(() => {
    if (showMenu && menuElement) {
      let menuWidth = menuElement.offsetWidth;
      let menuHeight = menuElement.offsetHeight;
      let windowWidth = window.innerWidth;
      let windowHeigth = window.innerHeight;

      if (mousePos.x + menuWidth > windowWidth) {
        mousePos.x = windowWidth - menuWidth - 5;
      }
      if (mousePos.y + menuHeight > windowHeigth) {
        mousePos.y = windowHeigth - menuHeight - 5;
      }
    }
  });

  function closeMenu() {
    showMenu = false;
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

  onMount(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (showMenu && event.key === "Escape") {
        showMenu = false;
      }
    }

    window.addEventListener("click", closeMenu);
    window.addEventListener("keydown", handleKeyDown);

    return () => {
      window.removeEventListener("click", closeMenu);
      window.removeEventListener("keydown", handleKeyDown);
    };
  });
</script>

<main class="container">
  <header>
    <p>Welcome to Kasumi</p>
  </header>

  <div class="track-scroll">
    <div class="track-grid">
      {#each tracks as track}
        <TrackCard {track} onPlay={playAudio} onContextMenu={handleContext} />
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
  <ContextMenu track={menuTrack} position={mousePos} />
{/if}

<style>
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
    gap: 48px;
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
