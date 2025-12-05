<script lang="ts">
  import type { Track } from "$lib/types";
  import TrackCard from "./TrackCard.svelte";

  let { tracks, onPlay, onMenu } = $props<{
    tracks: Track[];
    onPlay: (track: Track) => void;
    onMenu: (event: MouseEvent, track: Track) => void;
  }>();
</script>

<div class="track-scroll">
  <div class="track-grid">
    {#each tracks as track (track.path)}
      <TrackCard {track} {onPlay} onContextMenu={onMenu} />
    {/each}
  </div>
</div>

<style>
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
</style>
