<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { Track } from "../lib/types";
    import { path } from "@tauri-apps/api";

  let songPath = "/home/chish/Downloads/O-Rangrez.flac" //placeholder

  let tracks = $state<Track[]>([]);

  async function playAudio(songPath: string) {
    console.log("Attempting to play: ", songPath);
    try{
      await invoke("play_audio", {path: songPath})
    }catch (error){
      console.error("Rust error:", error);
      alert("Error:" + error);
    }
  }

  async function pauseAudio(){
    console.log("MUSIC PAUSED.");
    try{
      await invoke("pause_audio");
    } catch (error){
      console.error("Something went wrong: " + e);
    }
  }

  onMount(
    async () => {
      console.log("Scanning Music Directory...");
      try{
        tracks = await invoke("get_library_tracks");
      } catch (error){
        console.error("Ooopsies... Something went wrong: ", error);
      }
    }
  )

</script>

<main class="container">
  <h1>Welcome to Kasumi</h1>

  <p>App should be fully transparent right now :D</p>

  <div class="tracks">
  <ul>
    {#each tracks as track}
      <li onclick={() => {playAudio(track.path)}}>{track.title} - {track.artist}</li>
    {/each}
  </ul>
  </div>

  <button onclick={pauseAudio}>
    PAUSE MUSIC
  </button>

  <!-- <button onclick={playAudio} style="padding: 10px 20px; margin-top: 20px; font-size:1.2rem;"> -->
  <!--   ▶ Play Music -->
  <!-- </button> -->



</main>

<style>
.container{
  background: rgba(255, 255, 255, 0.2);
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
  text-align: center;
  border-radius: 16px;
  margin: 20px;
  padding: 50px;
}

h1{
  color: white;
}

:global(body){
  background-color: transparent !important;
  margin: 0;
  font-family: sans-serif;
  overflow: hidden;
}

:global(html){
  background-color: transparent !important;
}

li{
  list-style-type: none;
  
}


li:hover{
  cursor: pointer;
}

</style>
