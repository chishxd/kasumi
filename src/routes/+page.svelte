<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { load } from "@tauri-apps/plugin-store";
  import type { Track } from "../lib/types";
  import ContextMenu from "../components/ContextMenu.svelte";
  import TrackGrid from "../components/TrackGrid.svelte";
  import PlayerBar from "../components/PlayerBar.svelte";
  import { listen } from "@tauri-apps/api/event";
  import { open } from "@tauri-apps/plugin-dialog";
  import type { Store } from "@tauri-apps/plugin-store";

  let tracks = $state<Track[]>([]);
  let currentTrack = $state<Track | null>(null);
  let mousePos = $state({ x: 0, y: 0 });
  let menuTrack = $state<Track | null>(null);
  let showMenu = $state<boolean>(false);
  let store: Store;
  let musicDir: string | null = null;
  let showFolderPicker: boolean = $state(false);

  let isPaused = $state(false);
  let ended = $state(false);

  let menuElement = $state<HTMLDivElement | undefined>(undefined);

  function handlePlayPause() {
    if (ended && currentTrack) {
      playAudio(currentTrack);
      ended = false;
      return;
    }

    if (isPaused) {
      resumeAudio();
    } else {
      pauseAudio();
    }
  }

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
    console.log("MUSIC RESUMED.");
    try {
      await invoke("resume_audio");
      isPaused = await invoke("is_audio_paused");
    } catch (error) {
      console.error("Something went wrong: " + error);
    }
  }
  async function queueAdd(song: Track) {
    console.log(`Added ${song.title} to queue`);
    try {
      await invoke("queue_add", { track: song });
    } catch (error) {
      console.error("Failed to add song to queue" + error);
    }
  }
  async function queueSkip() {
    console.log("Skipping");
    try {
      await invoke("queue_skip");
    } catch (error) {
      console.error("Failed to skip track" + error);
    }
  }

  async function playPrev() {
    try {
      const prev = await invoke<Track>("play_previous");
      currentTrack = prev;
      isPaused = false;
    } catch (error) {
      console.warn("No Previous Track");
    }
  }

  async function loadTracks(dir: string) {
    tracks = await invoke("get_library_tracks", { path: dir });
  }

  async function chooseDir() {
    const dir = await open({
      directory: true,
      multiple: false,
    });

    if (typeof dir === "string") {
      musicDir = dir;
      await store.set("musicDir", dir);
      await store.save();

      showFolderPicker = false;
      loadTracks(musicDir);
    }
  }
  onMount(async () => {
    console.log("Scanning Music Directory...");
    try {
      store = await load("settings.json");

      musicDir = (await store.get<string>("musicDir")) ?? null;

      if (!musicDir) {
        showFolderPicker = true;
      } else {
        loadTracks(musicDir);
      }

      currentTrack = await invoke("get_current_track");
      isPaused = await invoke("is_audio_paused");

      const unlistenAutoPlay = await listen<Track>("autoplay_next", (event) => {
        const nextTrack = event.payload;
        console.log("Autoplay is now playing: ", nextTrack.title);

        currentTrack = nextTrack;
        isPaused = false;

        return () => {
          unlistenAutoPlay();
        };
      });

      const unlistenEnded = await listen("playback_ended", () => {
        console.log("No More Songs to be played, playback finished");
        isPaused = true;
        ended = true;

        return () => {
          unlistenEnded();
        };
      });
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
  <header class="app-header">
    <h1>Kasumi</h1>

    <button class="choose-btn" onclick={chooseDir}> Choose Directory </button>
  </header>

  {#if showFolderPicker}
    <div>
      <h1>SELECCT MUSIC FOLDER</h1>
      <button onclick={chooseDir}>Choose</button>
    </div>
  {:else}
    <TrackGrid {tracks} onPlay={playAudio} onMenu={handleContext} />

    <PlayerBar
      {currentTrack}
      {isPaused}
      onPause={handlePlayPause}
      onResume={handlePlayPause}
      onNext={queueSkip}
      onPrev={playPrev}
    />
  {/if}
</main>

{#if showMenu}
  <ContextMenu
    track={menuTrack}
    position={mousePos}
    onQueueAdd={queueAdd}
    onClose={closeMenu}
  />
{/if}

<style>
  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;

    padding: 14px 22px;
    margin-bottom: 8px;

    backdrop-filter: blur(8px);
    background: rgba(255, 255, 255, 0.12);
    border-radius: 12px;

    user-select: none;
  }

  .app-header h1 {
    margin: 0;
    font-size: 1.4rem;
    font-weight: 600;
    color: white;
  }

  .choose-btn {
    padding: 8px 16px;
    border-radius: 10px;

    background: rgba(255, 255, 255, 0.25);
    border: 1px solid rgba(255, 255, 255, 0.3);

    color: white;
    font-size: 0.9rem;
    font-weight: 500;

    cursor: pointer;
    transition:
      background 0.2s ease,
      transform 0.15s ease;
  }

  .choose-btn:hover {
    background: rgba(255, 255, 255, 0.35);
    transform: translateY(-1px);
  }

  .choose-btn:active {
    transform: translateY(1px);
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
