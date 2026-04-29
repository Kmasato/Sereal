<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, onDestroy } from "svelte";
  import { Terminal } from "@xterm/xterm";
  import { FitAddon } from "@xterm/addon-fit";

  import "@xterm/xterm/css/xterm.css";

  let terminalElement: HTMLDivElement;
  let term: Terminal;
  let fitAddon: FitAddon;

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

    term.write("Hello Sereal");

    const handleResize = () => fitAddon.fit();
    window.addEventListener("resize", handleResize);

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
