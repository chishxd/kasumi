<script lang="ts">
  import type { Track } from "$lib/types";

  let { track, onPlay, onContextMenu } = $props<{
    track: Track;
    onPlay: (track: Track) => void;
    onContextMenu: (event: MouseEvent, track: Track) => void;
  }>();

  function handlePlayClick() {
    onPlay(track);
  }

  function handleContextMenu(event: MouseEvent) {
    event.preventDefault();

    onContextMenu(event, track);
  }
</script>

<button
  class="track-card"
  onclick={handlePlayClick}
  oncontextmenu={handleContextMenu}
>
  <img src={track.coverArt ?? "/fallback.png"} alt={track.title} />

  <div class="overlay">
    <p class="track-title">{track.title}</p>
    <p class="track-artist">{track.artist}</p>
  </div>
</button>

<style>
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
  img {
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
</style>
