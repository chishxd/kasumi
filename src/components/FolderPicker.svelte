<script lang="ts">
  export let chooseDir: () => void;
</script>

<div class="picker-screen">
  <h1>SELECT YOU MUSIC FOLDER</h1>
  <button onclick={chooseDir}>CHOOSE</button>
</div>

<style>
  .picker-screen {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100%;
    text-align: center;
    gap: 20px;
  }

  button {
    padding: 12px 24px;
    font-size: 1rem;
    border-radius: 10px;
  }
</style>

