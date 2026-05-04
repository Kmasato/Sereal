<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";

  import "@xterm/xterm/css/xterm.css";

  let ports: string[] = [];
  let selectedPort: string = "";
  let terminalElement: HTMLDivElement;
  let term: Terminal;
  let fitAddon: FitAddon;

  async function refreshPorts() {
    ports = await invoke("get_ports");
    console.log("Available ports:", ports);
    if (!ports.includes(selectedPort)) {
      selectedPort = ports.length > 0 ? ports[0] : "";
    }
  }

  async function Connect() {
    if (!selectedPort) return;
    try {
      await invoke("connect", { portName: selectedPort, baudRate: 115200 });
      console.log("Connected to", selectedPort);
    } catch (e) {
      console.error("Failed to connect:", e);
    }
  }

  onMount(() => {
    term = new Terminal({
      convertEol: true,
      cursorBlink: true,
      theme: {
        background: "#1e1e1e",
      },
    });

    fitAddon = new FitAddon();
    term.loadAddon(fitAddon);
    term.open(terminalElement);
    fitAddon.fit();

    const handleResize = () => fitAddon.fit();
    window.addEventListener("resize", handleResize);

    let unlisten: () => void;
    const setup = async () => {
      await refreshPorts();
      unlisten = await listen("serial-data", (event) => {
        term.write(event.payload as string);
      });
    };
    setup();

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  });

  onDestroy(() => {
    if (term) {
      term.dispose();
    }
  });
</script>

<main>
  <div class="controlls">
    <select bind:value={selectedPort} on:mousedown={refreshPorts}>
      {#if ports.length === 0}
        <option value="">(No ports detected)</option>
      {:else}
        {#if !selectedPort}<option value="">-- Select Port --</option>{/if}
        {#each ports as port}
          <option value={port}>{port}</option>
        {/each}
      {/if}
    </select>
    <button on:click={Connect} disabled={!selectedPort}> Connect </button>
  </div>
  <div class="terminal-container" bind:this={terminalElement}></div>
</main>

<style>
  main {
    width: 100vw;
    height: 100vh;
    background-color: #1e1e1e;
    margin: 0;
  }

  .terminal-container {
    width: 100%;
    height: 100%;
    padding: 10px;
    box-sizing: border-box;
  }

  :global(.xterm) {
    height: 100%;
  }
</style>
