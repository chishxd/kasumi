<script lang="ts">
  import type { Track } from "$lib/types";

  let { currentTrack, isPaused, onPause, onResume, onNext } = $props<{
    currentTrack: Track | null;
    isPaused: boolean;
    onPause: () => void;
    onResume: () => void;
    onNext: () => void;
  }>();
</script>

<div class="player-bar">
  <div class="now-playing">
    <p class="now-title">{currentTrack?.title || "No Music"}</p>
    <p class="now-artist">{currentTrack?.artist || "No artist"}</p>
  </div>

  <div class="controls">
    {#if isPaused}
      <button onclick={onResume} class="player-btn">PLAY</button>
    {:else}
      <button onclick={onPause} class="player-btn">PAUSE</button>
    {/if}
    <button onclick={onNext} class="player-btn">NEXT</button>
  </div>
</div>

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
</style>
