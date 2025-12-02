<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container">
  <h1>Welcome to Kasumi</h1>

  <p>App should be fully transparent right now :D</p>
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
