<script lang="ts">
  import { onMount } from "svelte";
  import type { Track } from "../lib/types";
  import ContextMenu from "../components/ContextMenu.svelte";
  import TrackGrid from "../components/TrackGrid.svelte";
  import PlayerBar from "../components/PlayerBar.svelte";
  import { library, loadLibrary } from "$lib/stores/library";
  import {
    player,
    initPlayerState,
    playAudio,
    pauseAudio,
    resumeAudio,
  } from "$lib/stores/player";

  // let currentTrack = $state<Track | null>(null);
  let mousePos = $state({ x: 0, y: 0 });
  let menuTrack = $state<Track | null>(null);
  let showMenu = $state<boolean>(false);

  // let isPaused = $state(false);

  async function handleContext(event: MouseEvent, track: Track) {
    event.preventDefault();

    menuTrack = track;
    mousePos.x = event.clientX;
    mousePos.y = event.clientY;
    showMenu = true;
    console.log(`Clicked at: ${mousePos.x},${mousePos.y}`);
  }

  function closeMenu() {
    showMenu = false;
  }

  onMount(() => {
    (async () => {
      try {
        await loadLibrary();
        await initPlayerState();
      } catch (error) {
        console.error("Failed to initialize the application:", error);
      }
    })();

    const handleKeyDown = (event: KeyboardEvent) => {
      if (showMenu && event.key === "Escape") closeMenu();
    };

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

  <TrackGrid tracks={library.tracks} onPlay={playAudio} onMenu={handleContext} />

  <PlayerBar
    currentTrack={player.currentTrack}
    isPaused={player.isPaused}
    onPause={pauseAudio}
    onResume={resumeAudio}
  />
</main>

{#if showMenu}
  <ContextMenu track={menuTrack} position={mousePos} />
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
