<script lang="ts">
    import { SearchAddon } from "@xterm/addon-search";
    import { tick, onMount, onDestroy } from "svelte";

    let { searchAddon, onClose } = $props<{
        searchAddon: SearchAddon | null;
        onClose: () => void;
    }>();

    let searchText = $state("");
    let inputElem: HTMLInputElement | null = null;

    let resultIndex = $state(-1);
    let resultCount = $state(0);

    let resultText = $derived.by(() => {
        if (!searchText || resultCount === 0 || resultIndex === -1) {
            return searchText ? "No results" : "";
        }
        return `${resultIndex + 1} / ${resultCount}`;
    });

    $effect(() => {
        if (!searchAddon) return;

        const disposable = searchAddon.onDidChangeResults(
            (e: { resultIndex: number; resultCount: number }) => {
                resultIndex = e.resultIndex;
                resultCount = e.resultCount;
            },
        );

        return () => {
            disposable.dispose();
        };
    });

    function getSearchOptions(incremental = false) {
        return {
            incremental, // 最初のヒット位置に移動するか
            caseSensitive: false, // 大文字・小文字の区別をするか
            decorations: {
                // 現在のジャンプ位置
                activeMatchBackground: "#666666",
                activeMatchBorder: "#ffaa00",

                // 他の候補の値
                matchBackground: "#666666",
            },
        };
    }

    function searchNext() {
        if (!searchText || !searchAddon) {
            return;
        }
        searchAddon.findNext(searchText, getSearchOptions(false));
    }

    function searchPrevious() {
        if (!searchText || !searchAddon) {
            return;
        }
        searchAddon.findPrevious(searchText, getSearchOptions(false));
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === "Enter") {
            event.preventDefault();
            if (event.shiftKey) {
                searchPrevious();
            } else {
                searchNext();
            }
        } else if (event.key === "Escape") {
            event.preventDefault();
            onClose();
        }
    }

    $effect(() => {
        if (!searchAddon) return;

        if (searchText) {
            searchAddon.findNext(searchText, getSearchOptions(true));
        } else {
            searchAddon.clearDecorations();
            resultIndex = -1;
            resultCount = 0;
        }
    });

    onMount(() => {
        tick().then(() => {
            inputElem?.focus();
            inputElem?.select();
        });
    });

    onDestroy(() => {
        if (searchAddon) {
            searchAddon.clearDecorations();
        }
    });
</script>

<div class="floating-search-bar">
    <input
        type="text"
        class="search-input"
        placeholder="Find"
        bind:value={searchText}
        bind:this={inputElem}
        onkeydown={handleKeyDown}
    />
    {#if resultText}
        <span class="result-count">{resultText}</span>
    {/if}
    <button
        class="search-btn"
        onclick={searchPrevious}
        title="Previous (Shift+Enter)">▲</button
    >
    <button class="search-btn" onclick={searchNext} title="Next (Enter)"
        >▼</button
    >
    <button class="search-btn close-btn" onclick={onClose} title="Close (Esc)"
        >✕</button
    >
</div>

<style>
    .floating-search-bar {
        position: absolute;
        top: 10px;
        right: 25px;
        z-index: 100;
        display: flex;
        align-items: center;
        gap: 5px;
        background-color: #3b3b3b;
        border: 1px solid #3c3c3c;
        border-radius: 4px;
        padding: 5px 8px;
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.4);
    }

    .search-input {
        background-color: #1e1e1e;
        color: #cccccc;
        border: 1px solid #3c3c3c;
        border-radius: 4px;
        padding: 4px 6px;
        font-size: 13px;
        outline: none;
        box-sizing: border-box;
        width: 180px;
    }

    .search-input:focus {
        border-color: #cccccc;
    }

    .result-count {
        font-size: 11px;
        color: #aaaaaa;
        padding: 0 4px;
        white-space: nowrap;
        user-select: none;
    }

    .search-btn {
        background-color: #6c6c6c;
        color: #ffffff;
        border: none;
        border-radius: 4px;
        padding: 4px 8px;
        font-size: 11px;
        cursor: pointer;
        transition:
            filter 0.2s,
            transform 0.1s;
        box-sizing: border-box;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .search-btn:hover {
        filter: brightness(1.5);
    }

    .search-btn:active {
        transform: scale(0.95);
        filter: brightness(0.9);
    }
</style>
