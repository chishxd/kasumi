<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { Track } from "../lib/types";
  import ContextMenu from "../components/ContextMenu.svelte";
  import TrackGrid from "../components/TrackGrid.svelte";
  import PlayerBar from "../components/PlayerBar.svelte";

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
  async function queueAdd(song: Track) {
    console.log(`Added ${song.title} to queue`)
    try{
      await invoke("queue_add", {track: song});
    } catch (error){
      console.error("Failed to add song to queue" + error);
    }
  }
  async function queueSkip() {
    console.log("Skipping");
    try{
      await invoke("queue_skip")
    } catch (error){
      console.error("Failed to skip track" + error);
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

  <TrackGrid {tracks} onPlay={playAudio} onMenu={handleContext} />

  <PlayerBar
    {currentTrack}
    {isPaused}
    onPause={pauseAudio}
    onResume={resumeAudio}
    onNext={queueSkip}
  />
</main>

{#if showMenu}
  <ContextMenu track={menuTrack} position={mousePos} onQueueAdd={queueAdd}/>
{/if}

<style>
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
