<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let ports: string[] = [];

  onMount(async () => {
    try {
      ports = await invoke("get_ports");
      console.log("Available ports:", ports);
    } catch (error) {
      console.error("Failed to fetch ports:", error);
    }
  });
</script>

<main>
  <h1>Available Serial Ports</h1>
  {#if ports.length == 0}
    <p>No ports found.</p>
  {:else}
    <ul>
      {#each ports as port}
        <li>{ports}</li>
      {/each}
    </ul>
  {/if}
</main>
