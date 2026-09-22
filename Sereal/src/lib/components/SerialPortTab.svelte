<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { portStore } from "$lib/stores/portStore.svelte";
    import { Terminal } from "@xterm/xterm";
    import { FitAddon } from "@xterm/addon-fit";
    import { listen } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/core";

    import ConnectionButton from "$lib/components/ConnectionButton.svelte";
    import type { ConnectionState } from "./types";
    import scrollBottomIcon from "$lib/assets/scroll_bottom.svg";
    import clearIcon from "$lib/assets/eraser.svg";

    import "@xterm/xterm/css/xterm.css";

    let {
        clientId,
        initialPortName = "",
        onConnected = null,
    } = $props<{
        clientId: string;
        initialPortName?: string;
        onConnected?: (portName: string) => void;
    }>();

    let availablePorts: string[] = $state([]);
    let selectedPort: string = $state(initialPortName);
    let connectedPort: string = $state("");
    let selectedBaudRate: number = $state(115200);
    let connectionState: ConnectionState = $state("invalid");
    let sendText: string = $state("");

    let termiailElement: HTMLDivElement;
    let terminal: Terminal | null = null;
    let fitAddon: FitAddon | null = null;
    let resizeObserver: ResizeObserver | null = null;
    let unlisten: (() => void) | null = null;
    let unlistenStatus: (() => void) | null = null;

    async function sendData() {
        if (!sendText || connectionState !== "connected") return;

        try {
            await invoke("send", {
                clientId: clientId,
                data: sendText,
            });
            sendText = "";
        } catch (e) {
            console.error("Failed to send data:", e);
        }
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === "Enter" && !event.isComposing) {
            sendData();
        }
    }

    function scrollToBottom() {
        if (terminal) {
            terminal.scrollToBottom();
        }
    }

    function clearTerminal() {
        if (terminal) {
            terminal.clear();
        }
    }

    $effect(() => {
        portStore.setPort(clientId, selectedPort);
    });

    async function refreshPorts() {
        const allPorts: string[] = await invoke("get_ports");

        // 接続済みの Port は get_ports に含まれない
        // 自身のタブで接続している Port はドロップダウンリストに表示させたいため、allPortsに追加する
        const currentPort = connectedPort || selectedPort;
        if (currentPort) {
            allPorts.unshift(currentPort);
        }

        // 他のタブで選択済みの Port はドロップダウンリストから除外する
        const usedPorts = portStore.getUserPortsExcept(clientId);

        availablePorts = allPorts.filter((port) => {
            if (port === currentPort) {
                return true; // 自身のタブで選択済みのポートはリストに追加する
            }
            return !usedPorts.has(port); // 他のタブで選択済みのポートであれば除外する
        });
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
        if (connectionState == "connected") {
            console.log(
                "Call connect but connection station is already opend.",
            );
            return;
        }

        // 接続処理
        let connectionSuccess = await invoke("connect", {
            clientId: clientId,
            portName: selectedPort,
            baudRate: selectedBaudRate,
        });

        if (connectionSuccess) {
            console.log("Connected to", selectedPort);

            // 接続成功を親に通知
            if (onConnected) {
                onConnected(selectedPort);
            }

            connectionState = "connected";
            connectedPort = selectedPort;
        } else {
            console.error("Failed to connect to", selectedPort);
        }
    }

    async function disconnect() {
        if (!connectedPort) {
            console.warn("Not opened port");
            return;
        }
        let disconnectionSuccess = await invoke("disconnect", {
            clientId: clientId,
        });

        if (disconnectionSuccess) {
            console.log("Disconnected from", connectedPort);
            connectedPort = "";
            connectionState = "disconnected";
        } else {
            console.error("Failed to disconnect from", connectedPort);
        }
    }

    function initTerminal() {
        // ターミナルのインスタンス化
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

        resizeObserver = new ResizeObserver(() => {
            if (fitAddon) {
                fitAddon.fit();
            }
        });
        resizeObserver.observe(termiailElement);

        // データ受信リスナーの登録
        listen("serial-data", (event) => {
            const payload = event.payload as {
                clientId: string;
                text: string;
            };
            if (payload.clientId === clientId && terminal) {
                terminal.write(payload.text);
            }
        }).then((fn) => {
            unlisten = fn;
        });

        // 接続ステータス変更のリスナーの登録
        // ユーザーの操作以外で内部状態が変わった場合にバックグラウンドと同期するための処理
        listen("connection-status-changed", (event) => {
            const payload = event.payload as {
                clientId: string;
                status: ConnectionState;
            };
            if (payload.clientId === clientId && terminal) {
                connectionState = payload.status;
                console.log("Connection status changed:", payload.status);
            }
        }).then((fn) => {
            unlistenStatus = fn;
        });
    }

    async function cleanupTerminal() {
        await disconnect();
        await invoke("unregister_handler", { clientId: clientId });

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
        invoke("register_handler", { clientId: clientId });
        refreshPorts();
        initTerminal();
    });

    onDestroy(() => {
        portStore.removePort(clientId);
        cleanupTerminal();
    });
</script>

<div class="tab-content">
    <!-- 上部メニューバー (ツールバー) -->
    <div class="menu-bar">
        <div class="menu-item port-item">
            <label for="port-select">Port:</label>
            <!--
            refreshPorts() が非同期処理であり、
            表示処理が先行した場合に古い状態が表示される場合があるため、
            mousedownではなく、mouseenterで更新する
            -->
            <select
                id="port-select"
                bind:value={selectedPort}
                onmouseenter={refreshPorts}
                onchange={async () => {
                    if (connectedPort) {
                        await disconnect();
                    }
                    await connect();
                }}
            >
                {#if availablePorts.length === 0}
                    <option value="">No ports detected</option>
                {:else}
                    {#if !selectedPort}<option value="" disabled hidden
                            >Select Port</option
                        >{/if}
                    {#each availablePorts as port}
                        <option value={port}>{port}</option>
                    {/each}
                {/if}
            </select>
        </div>

        <div class="menu-item baud-item">
            <label for="baud-select">Baud Rate:</label>
            <select id="baud-select" bind:value={selectedBaudRate}>
                <option value={9600}>9600</option>
                <option value={19200}>19200</option>
                <option value={38400}>38400</option>
                <option value={57600}>57600</option>
                <option value={115200}>115200</option>
            </select>
        </div>

        <div class="connect-wrapper" onclick={handleConnectToggle}>
            <ConnectionButton state={connectionState} />
        </div>

        <button
            class="clear-button"
            onclick={clearTerminal}
            title="Clear terminal"
        >
            <img src={clearIcon} alt="Clear terminal" class="clear-icon" />
        </button>

        <button
            class="scroll-bottom-button"
            onclick={scrollToBottom}
            title="Scroll to bottom"
        >
            <img
                src={scrollBottomIcon}
                alt="Scroll to bottom"
                class="scroll-bottom-icon"
            />
        </button>
    </div>

    <!-- シリアル送信 -->
    <div class="send-bar">
        <input
            type="text"
            class="send-input"
            placeholder="Type in message to send to the serial port."
            bind:value={sendText}
            onkeydown={handleKeyDown}
        />
        <button class="send-button" onclick={sendData}>Send</button>
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
        gap: 10px;
        align-items: center;
        padding: 8px 15px;
        background-color: #3b3b3b;
        border-bottom: 1px solid #3c3c3c;
        box-sizing: border-box;
    }

    .send-bar {
        display: flex;
        flex-direction: row;
        gap: 5px;
        align-items: center;
        padding: 0px 15px 5px 10px;
        background-color: #3b3b3b;
        border-bottom: 1px solid #3c3c3c;
        box-sizing: border-box;
    }

    .send-input {
        flex: 1;
        max-width: 500px;
        background-color: #1e1e1e;
        color: #cccccc;
        border: 1px solid #3c3c3c;
        border-radius: 4px;
        padding: 4px 5px;
        font-size: 13px;
        outline: none;
        box-sizing: border-box;
    }

    .send-input:focus {
        border-color: #cccccc;
    }

    .send-input:disabled {
        opacity: 0.5;
        cursor: not-allowed;
    }

    .send-button {
        background-color: #6c6c6c;
        color: #ffffff;
        border: none;
        border-radius: 4px;
        padding: 5px 8px;
        font-size: 11px;
        cursor: pointer;
        transition:
            filter 0.2s,
            transform 0.1s;
        box-sizing: border-box;
    }

    .send-button:hover {
        filter: brightness(1.5);
    }

    .menu-item {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 5px;
    }

    .menu-item.port-item {
        flex: 0 1 auto;
        min-width: 120px;
        max-width: 280px;
    }

    #port-select {
        width: 100%;
        min-width: 0;
        text-overflow: ellipsis;
    }

    .menu-item.baud-item,
    .connect-wrapper,
    .clear-button,
    .scroll-bottom-button {
        flex-shrink: 0;
    }

    .clear-button,
    .scroll-bottom-button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 30px;
        height: 18px;
        background-color: #6c6c6c;
        border: 1px solid rgba(255, 255, 255, 0.1);
        border-radius: 10%;
        cursor: pointer;
        padding: 0;
        box-sizing: border-box;
        transition:
            filter 0.2s,
            transform 0.1s;
        outline: none;
    }

    .clear-button:hover,
    .scroll-bottom-button:hover {
        filter: brightness(1.5);
    }

    .clear-button:active,
    .scroll-bottom-button:active {
        transform: scale(0.95);
        filter: brightness(0.9);
    }

    .clear-icon,
    .scroll-bottom-icon {
        width: 14px;
        height: 14px;
    }

    .menu-item label {
        font-size: 0.8rem;
        color: #aaaaaa;
        white-space: nowrap;
    }

    select {
        background-color: #3c3c3c;
        color: #cccccc;
        border: 1px solid #555555;
        padding: 4px 8px;
        border-radius: 3px;
        font-size: 0.85rem;
        outline: none;
    }

    select:focus {
        border-color: #cccccc;
    }

    select:disabled {
        opacity: 0.6;
        background-color: #252526;
        color: #888888;
        border-color: #444444;
        cursor: not-allowed;
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
