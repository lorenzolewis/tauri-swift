<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  var tauriGetFileThumbnailBase64Result = "";
  function tauriGetFileThumbnailBase64() {
    console.log("tauriGetFileThumbnailBase64 pressed on the JS side");
    invoke("tauri_get_file_thumbnail_base64")
      .then((result) => console.log(result))
      .catch(console.error);
  }

  var tauriGetMountsResult = "";
  function tauriGetMounts() {
    console.log("tauriGetMounts pressed on the JS side");
    invoke("tauri_get_mounts")
      .then((result) => console.log(result))
      .catch(console.error);
  }

  var tauriReturnNullableResult = "";
  function tauriReturnNullable() {
    console.log("tauriReturnNullable pressed on the JS side");
    invoke("tauri_return_nullable")
      .then((result) => console.log(result))
      .catch(console.error);
  }

  var tauriBasicIntResult = "";
  function tauriBasicInt() {
    console.log("tauriBasicInt pressed on the JS side");
    invoke("tauri_basic_int")
      .then((result) => {
        console.log(result);
        tauriBasicIntResult = result as string;
      })
      .catch(console.error);
  }

  var tauriCheckNotificationPermissionResult = "";
  function tauriCheckNotificationPermission() {
    console.log("tauriCheckNotificationPermission pressed on the JS side");
    invoke("tauri_check_notification_permission")
      .then((result) => {
        console.log(result);
      })
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
      >tauriGetFileThumbnailBase64 (Crashes)</button
    >
    Result: {tauriGetFileThumbnailBase64Result}
    <br />
    <button on:click={tauriGetMounts}>tauriGetMounts (Crashes)</button>
    Result: {tauriGetMountsResult}
    <br />
    <button on:click={tauriReturnNullable}>tauriReturnNullable (Crashes)</button
    >
    Result: {tauriReturnNullableResult}
    <br />
    <button on:click={tauriBasicInt}>tauriBasicInt</button>
    Result: {tauriBasicIntResult}
    <br />
    <button on:click={tauriCheckNotificationPermission}
      >tauriCheckNotificationPermission</button
    >
    Result: {tauriCheckNotificationPermissionResult}
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
