<script lang="ts">
  import type { Track } from "$lib/types";

  let { track, position } = $props<{
    track: Track | null;
    position: { x: number; y: number };
  }>();

  let menuElement = $state<HTMLDivElement | undefined>(undefined);
  $effect(() => {
    if (menuElement) {
      let menuWidth = menuElement.offsetWidth;
      let menuHeight = menuElement.offsetHeight;
      let windowWidth = window.innerWidth;
      let windowHeigth = window.innerHeight;

      if (position.x + menuWidth > windowWidth) {
        position.x = windowWidth - menuWidth - 5;
      }
      if (position.y + menuHeight > windowHeigth) {
        position.y = windowHeigth - menuHeight - 5;
      }
    }
  });
</script>

<div
  class="context-menu"
  bind:this={menuElement}
  style="position: absolute; top: {position.y}px; left: {position.x}px;"
>
  <!-- TODO: Add Functions for menu items -->
  <button class="menu-item">Add to Queue</button>
  <button class="menu-item">Add to Playlist</button>
  <button class="menu-item" style="color: rgba(255, 0, 0, 0.7);"
    >Delete from Library</button
  >
</div>


<style>
  .menu-item {
    all: unset;
    cursor: pointer;
    padding: 8px 10px;
    border-radius: 5px;
    font-size: 0.9rem;
    text-align: left;
    transition: background-color 0.15s ease;
  }
  .menu-item:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }

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
</style>
