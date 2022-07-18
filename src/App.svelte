<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  function tauriGetFileThumbnailBase64() {
    console.log("tauriGetFileThumbnailBase64 pressed on the JS side");
    invoke("tauri_get_file_thumbnail_base64");
  }
  function tauriGetMounts() {
    console.log("tauriGetMounts pressed on the JS side");
    invoke("tauri_get_mounts");
  }
  function tauriReturnNullable() {
    console.log("tauriReturnNullable pressed on the JS side");
    invoke("tauri_return_nullable");
  }

  function tauriBasicInt() {
    console.log("tauriBasicInt pressed on the JS side");
    invoke("tauri_basic_int")
      .then((result) => console.log(result))
      .catch(console.error);
  }
</script>

<main>
  <h1>Hello Taurists!</h1>
  <p>
    Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn
    how to build Svelte apps.
  </p>
  <div>
    <button on:click={tauriGetFileThumbnailBase64}
      >tauriGetFileThumbnailBase64</button
    >
    <br />
    <button on:click={tauriGetMounts}>tauriGetMounts</button>
    <br />
    <button on:click={tauriReturnNullable}>tauriReturnNullable</button>
    <br />
    <button on:click={tauriBasicInt}>tauriBasicInt</button>
  </div>
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
