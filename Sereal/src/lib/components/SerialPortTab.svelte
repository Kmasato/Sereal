<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { Terminal } from "@xterm/xterm";
    import { FitAddon } from "@xterm/addon-fit";
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";

    import ConnectionButton from "$lib/components/ConnectionButton.svelte";
    import type { ConnectionState } from "./types";

    import "@xterm/xterm/css/xterm.css";

    let { initialPortName = "", onConnected = null } = $props<{
        initialPortName?: string;
        onConnected?: (portName: string) => void;
    }>();

    let ports: string[] = $state([]);
    let selectedPort: string = $state(initialPortName);
    let selectedBaudRate: number = $state(115200);
    let connectionState: ConnectionState = $state("invalid");

    let termiailElement: HTMLDivElement;
    let terminal: Terminal | null = null;
    let fitAddon: FitAddon | null = null;
    let unlisten: (() => void) | null = null;
    let unlistenStatus: (() => void) | null = null;

    async function refreshPorts() {
        if (connectionState == "connected") return;
        ports = await invoke("get_ports");
    }

    async function handleConnectToggle() {
        if (connectionState == "connected") {
            await disconnect();
        } else {
            await connect();
        }
    }

    async function connect() {
        if (!selectedPort) return;
        if (connectionState == "connected") return;
        if (connectionState == "invalid") {
            await invoke("register_handler", { portName: selectedPort });
        }
        try {
            // 接続処理
            await invoke("connect", {
                portName: selectedPort,
                baudRate: selectedBaudRate,
            });
            console.log("Connected to", selectedPort);

            // 接続成功を親に通知
            if (onConnected) {
                onConnected(selectedPort);
            }

            // 接続直後は Terminal 内で状態の変化を検知できないため、明示的に指定
            connectionState = "connected";

            if (terminal !== null) {
                return;
            }

            // 初回接続時のみ、DOM の hidden 解除をまってターミナルを初期化
            setTimeout(() => {
                initTerminal();
            }, 50);
        } catch (e) {
            console.error("Failed to connect:", e);
        }
    }

    async function disconnect() {
        if (!selectedPort) return;
        try {
            await invoke("disconnect", { portName: selectedPort });
            console.log("Disconnected from", selectedPort);
        } catch (e) {
            console.error("Failed to disconnect:", e);
        }
    }

    function initTerminal() {
        // ターミナルのインスタンス化 (C++ における new に相当)
        terminal = new Terminal({
            convertEol: true,
            disableStdin: true,
            theme: {
                background: "#1e1e1e",
            },
        });

        fitAddon = new FitAddon();
        terminal.loadAddon(fitAddon);
        terminal.open(termiailElement);
        fitAddon.fit();

        // データ受信リスナーの登録
        listen("serial-data", (event) => {
            const payload = event.payload as {
                port_name: string;
                text: string;
            };
            if (payload.port_name === selectedPort && terminal) {
                terminal.write(payload.text);
            }
        }).then((fn) => {
            unlisten = fn;
        });

        // 接続ステータス変更のリスナーの登録
        listen("connection-status-changed", (event) => {
            const payload = event.payload as {
                port_name: string;
                status: ConnectionState;
            };
            if (payload.port_name === selectedPort && terminal) {
                connectionState = payload.status;
                console.log("Connection status changed:", payload.status);
            }
        }).then((fn) => {
            unlistenStatus = fn;
        });
    }

    async function cleanupTerminal() {
        await invoke("unregister_handler", { portName: selectedPort });

        if (unlisten) {
            unlisten();
            unlisten = null;
        }
        if (terminal) {
            terminal.dispose();
            terminal = null;
        }
        if (fitAddon) {
            fitAddon = null;
        }
    }

    // Golden Layout からサイズ変更通知を受けた時に実行する関数
    export function fit() {
        if (fitAddon) {
            fitAddon.fit();
        }
    }

    onMount(() => {
        if (initialPortName) {
            connect();
        } else {
            refreshPorts();
        }
    });

    onDestroy(() => {
        // リソースの明示的な解放 (C++ における delete / デストラクタに相当)
        cleanupTerminal();
    });
</script>

<div class="tab-content">
    <!-- 上部メニューバー (ツールバー) -->
    <div class="menu-bar">
        <div class="menu-item">
            <label for="port-select">Port:</label>
            <select
                id="port-select"
                bind:value={selectedPort}
                onmousedown={refreshPorts}
            >
                {#if ports.length === 0}
                    <option value="">(No ports detected)</option>
                {:else}
                    {#if !selectedPort}<option value="" disabled hidden
                            >Select Port</option
                        >{/if}
                    {#each ports as port}
                        <option value={port}>{port}</option>
                    {/each}
                {/if}
            </select>
        </div>

        <div class="menu-item">
            <label for="baud-select">Baud Rate:</label>
            <select id="baud-select" bind:value={selectedBaudRate}>
                <option value={9600}>9600</option>
                <option value={19200}>19200</option>
                <option value={38400}>38400</option>
                <option value={57600}>57600</option>
                <option value={115200}>115200</option>
            </select>
        </div>

        <div onclick={handleConnectToggle}>
            <ConnectionButton state={connectionState} />
        </div>
    </div>

    <!-- 受信データの描画領域 -->
    <div class="terminal-area">
        <div
            class="serialport-tab-container"
            class:hidden={connectionState === "invalid"}
            bind:this={termiailElement}
        ></div>
    </div>
</div>

<style>
    .tab-content {
        width: 100%;
        height: 100%;
        display: flex;
        flex-direction: column;
        background-color: #1e1e1e;
        color: #cccccc;
        box-sizing: border-box;
    }

    /* 上部メニューバー */
    .menu-bar {
        display: flex;
        flex-direction: row;
        gap: 15px;
        align-items: center;
        padding: 8px 15px;
        background-color: #3b3b3b;
        border-bottom: 1px solid #3c3c3c;
        box-sizing: border-box;
    }

    .menu-item {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 5px;
    }

    .menu-item label {
        font-size: 0.8rem;
        color: #aaaaaa;
        white-space: nowrap;
    }

    select,
    button {
        background-color: #3c3c3c;
        color: #cccccc;
        border: 1px solid #555555;
        padding: 4px 8px;
        border-radius: 3px;
        font-size: 0.85rem;
        outline: none;
    }

    select:focus {
        border-color: #007acc;
    }

    select:disabled {
        opacity: 0.6;
        background-color: #252526;
        color: #888888;
        border-color: #444444;
        cursor: not-allowed;
    }

    button {
        background-color: #007acc;
        color: #ffffff;
        border: none;
        cursor: pointer;
        font-weight: bold;
        padding: 5px 12px;
        transition: background-color 0.2s;
    }

    button:hover:not(:disabled) {
        background-color: #0062a3;
    }

    button.connected {
        background-color: #a1260d;
    }

    button.connected:hover:not(:disabled) {
        background-color: #801d0a;
    }

    button:disabled {
        opacity: 0.5;
        cursor: not-allowed;
        background-color: #3c3c3c;
        color: #888888;
        border: 1px solid #555555;
    }

    /* 下部ターミナル領域 */
    .terminal-area {
        flex-grow: 1;
        width: 100%;
        position: relative;
        overflow: hidden;
    }

    .placeholder-message {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        text-align: center;
        color: #777777;
        font-size: 0.9rem;
        padding: 20px;
        pointer-events: none;
    }

    .serialport-tab-container {
        width: 100%;
        height: 100%;
        box-sizing: border-box;
        padding: 5px;
    }

    .hidden {
        display: none;
    }
</style>
