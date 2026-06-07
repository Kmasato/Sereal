<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { Terminal } from "@xterm/xterm";
    import { FitAddon } from "@xterm/addon-fit";
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";

    import "@xterm/xterm/css/xterm.css";

    let { initialPortName = "", onConnected = null } = $props<{
        initialPortName?: string;
        onConnected?: (portName: string) => void;
    }>();

    let ports: string[] = $state([]);
    let selectedPort: string = $state(initialPortName);
    let selectedBaudRate: number = $state(115200);
    let isConnected: boolean = $state(false);

    let termiailElement: HTMLDivElement;
    let terminal: Terminal | null = null;
    let fitAddon: FitAddon | null = null;
    let unlisten: (() => void) | null = null;

    async function refreshPorts() {
        if (isConnected) return;
        ports = await invoke("get_ports");
        if (!ports.includes(selectedPort)) {
            selectedPort = ports.length > 0 ? ports[0] : "";
        }
    }

    async function handleConnectToggle() {
        if (isConnected) {
            await disconnect();
        } else {
            await connect();
        }
    }

    async function connect() {
        if (!selectedPort) return;
        try {
            // 1. バックエンド接続
            await invoke("connect", {
                portName: selectedPort,
                baudRate: selectedBaudRate,
            });
            console.log("Connected to", selectedPort);
            isConnected = true;

            // 2. 接続成功を親に通知 (タイトル変更や次の空タブ生成をトリガー)
            if (onConnected) {
                onConnected(selectedPort);
            }

            // 3. Svelte が DOM を更新して .serialport-tab-container の hidden (display: none)
            // が解除されるのを待ってからターミナルを初期化し fit() を実行します
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
            // 1. バックエンド切断
            await invoke("disconnect", { portName: selectedPort });
            console.log("Disconnected from", selectedPort);
            isConnected = false;

            // 2. ターミナルリソースのクリーンアップ
            cleanupTerminal();
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
    }

    function cleanupTerminal() {
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
                disabled={isConnected}
            >
                {#if ports.length === 0}
                    <option value="">(No ports detected)</option>
                {:else}
                    {#if !selectedPort}<option value=""
                            >-- Select Port --</option
                        >{/if}
                    {#each ports as port}
                        <option value={port}>{port}</option>
                    {/each}
                {/if}
            </select>
        </div>

        <div class="menu-item">
            <label for="baud-select">Baud Rate:</label>
            <select
                id="baud-select"
                bind:value={selectedBaudRate}
                disabled={isConnected}
            >
                <option value={9600}>9600</option>
                <option value={19200}>19200</option>
                <option value={38400}>38400</option>
                <option value={57600}>57600</option>
                <option value={115200}>115200</option>
            </select>
        </div>

        <button
            onclick={handleConnectToggle}
            class:connected={isConnected}
            disabled={!selectedPort}
        >
            {isConnected ? "Disconnect" : "Connect"}
        </button>
    </div>

    <!-- 下部ターミナル領域 -->
    <div class="terminal-area">
        {#if !isConnected}
            <div class="placeholder-message">
                <p>
                    Not Connected. Select a port and click "Connect" to start
                    monitoring.
                </p>
            </div>
        {/if}
        <!-- ターミナル描画領域 (接続後のみ表示) -->
        <div
            class="serialport-tab-container"
            class:hidden={!isConnected}
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
