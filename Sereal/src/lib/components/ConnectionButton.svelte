<script lang="ts">
    import connectedIcon from "$lib/assets/connect.svg";
    import disconnectedIcon from "$lib/assets/disconnect.svg";

    import type { ConnectionState } from "./types";
    import { ColorPallet } from "$lib/constants/colorPallet";

    const stateConfig = {
        invalid: {
            icon: connectedIcon,
            color: ColorPallet.uiWhite,
        },
        connected: {
            icon: connectedIcon,
            color: ColorPallet.uiGreen,
        },
        disconnected: {
            icon: connectedIcon,
            color: ColorPallet.uiWhite,
        },
        physicalDisconnected: {
            icon: disconnectedIcon,
            color: ColorPallet.uiRed,
        },
    } satisfies Record<
        ConnectionState,
        {
            icon: string;
            color: string;
        }
    >;

    interface Props {
        state?: ConnectionState;
    }

    let { state = "invalid" }: Props = $props();
</script>

<button
    style:background-color={stateConfig[state].color}
    aria-label="Connection State"
    title="Connection State"
>
    <img
        src={stateConfig[state].icon}
        width="20"
        alt="Connection status icon"
    />
</button>

<style>
    button {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        width: 30px;
        height: 18px;
        border-radius: 10%;
        border: 1px solid rgba(255, 255, 255, 0.1);
        cursor: pointer;
        padding: 0;
        transition:
            filter 0.2s,
            transform 0.1s;
        outline: none;
    }

    button:hover {
        filter: brightness(1.15);
    }

    button:active {
        transform: scale(0.95);
        filter: brightness(0.9);
    }
</style>
