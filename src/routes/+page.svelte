<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let songPath = "/home/chish/Downloads/O-Rangrez.flac" //placeholder

  async function testAudio() {
    console.log("Attempting to play: ", songPath);
    try{
      await invoke("play_local_music", {path: songPath})
    }catch (error){
      console.error("Rust error:", error);
      alert("Error:" + error);
    }

  }
</script>

<main class="container">
  <h1>Welcome to Kasumi</h1>

  <p>App should be fully transparent right now :D</p>

  <button onclick={testAudio} style="padding: 10px 20px; margin-top: 20px; font-size:1.2rem;">
    ▶ Play Music
  </button>
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

</style>
