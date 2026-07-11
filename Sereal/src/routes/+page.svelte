<script lang="ts">
  import { onMount, mount, unmount } from "svelte";
  import { GoldenLayout, type ComponentItemConfig } from "golden-layout";
  import SerialPortTab from "$lib/components/SerialPortTab.svelte";
  import { invoke } from "@tauri-apps/api/core";

  import "golden-layout/dist/css/goldenlayout-base.css";
  import "golden-layout/dist/css/themes/goldenlayout-dark-theme.css";

  let layoutContainer: HTMLDivElement;
  let layout: GoldenLayout;

  // 動的マウントしたコンポーネントの参照を管理する Map
  const mountedComponents = new Map<
    string,
    {
      instance: ReturnType<typeof mount>;
      fit: () => void;
    }
  >();

  let tabCounter = 0;

  // 新規の接続用空タブを追加する関数
  function addConnectionTab(targetStack?: any) {
    tabCounter++;
    const tabId = `new-connection-${tabCounter}`;

    const itemConfig: ComponentItemConfig = {
      type: "component",
      componentType: "serial-port-tab",
      componentState: { tabId, portName: "" },
      title: "New Connection",
    };

    if (targetStack) {
      targetStack.addItem(itemConfig);
    } else if (layout.rootItem) {
      layout.addItem(itemConfig);
    } else {
      layout.loadLayout({
        root: {
          type: "row",
          content: [itemConfig],
        },
      });
    }
  }

  onMount(() => {
    // Golden Layout の初期化
    layout = new GoldenLayout(layoutContainer);

    // コンポーネントファクトリを登録
    layout.registerComponent("serial-port-tab", (container, state) => {
      const configState = state as { tabId: string; portName: string };
      let currentPortName = configState.portName;
      const initialTabId = configState.tabId;

      const instance = mount(SerialPortTab, {
        target: container.element,
        props: {
          initialPortName: currentPortName,
          onConnected: (connectedPortName: string) => {
            // 接続に成功した時の処理
            currentPortName = connectedPortName;

            // 1. タブのタイトルを接続先のポート名に変更
            container.setTitle(connectedPortName);

            // 2. マウント管理用の Map のキーを一時IDからポート名に変更
            const compInfo = mountedComponents.get(initialTabId);
            if (compInfo) {
              mountedComponents.delete(initialTabId);
              mountedComponents.set(connectedPortName, compInfo);
            }
          },
        },
      });

      // インスタンスの参照を保持 (最初は一時的な tabId をキーにする)
      const key = currentPortName || initialTabId;
      mountedComponents.set(key, {
        instance: instance as any,
        fit: () => {
          if (instance && typeof (instance as any).fit === "function") {
            (instance as any).fit();
          }
        },
      });

      // タブがリサイズされた時
      container.on("resize", () => {
        const activeKey = currentPortName || initialTabId;
        const comp = mountedComponents.get(activeKey);
        if (comp) {
          comp.fit();
        }
      });

      // タブが閉じられた(デストラクタ呼び出し)時のクリーンアップ処理
      container.on("destroy", () => {
        const activeKey = currentPortName || initialTabId;

        // 接続済みだった場合のみバックエンド側を切断
        if (currentPortName) {
          invoke("disconnect", { portName: currentPortName }).catch((e) => {
            console.error("Failed to disconnect during destroy:", e);
          });
        }

        // Svelteコンポーネントをアンマウント
        unmount(instance);
        mountedComponents.delete(activeKey);

        // 全てのタブが閉じられた場合、自動的に1つ「New Connection」タブを新規追加する
        setTimeout(() => {
          if (mountedComponents.size === 0) {
            addConnectionTab();
          }
        }, 50);
      });
    });

    // スタック(タブバー)生成イベントをリッスンし、タブの横に「＋」ボタンを挿入する
    layout.on("itemCreated", (event) => {
      const item = (event as any).target;
      if (item && item.isStack) {
        setTimeout(() => {
          const stack = item as any;
          const headerElement = stack.header?.element as HTMLElement;
          if (headerElement) {
            // タブが並ぶコンテナ（ul要素など）を取得
            const tabsContainer = headerElement.querySelector(".lm_tabs");
            // すでに「＋」ボタンが追加されていないかチェック
            if (tabsContainer && !tabsContainer.querySelector(".lm_tab_plus")) {
              const plusBtn = document.createElement("li");
              plusBtn.className = "lm_tab_plus"; // lm_tab クラスは外して完全独自デザインに
              plusBtn.innerHTML = "+";
              plusBtn.style.cursor = "pointer";
              plusBtn.style.display = "inline-flex";
              plusBtn.style.alignItems = "center";
              plusBtn.style.justifyContent = "center";
              plusBtn.style.width = "24px";
              plusBtn.style.height = "24px";
              plusBtn.style.borderRadius = "50%";
              plusBtn.style.marginLeft = "8px";
              plusBtn.style.marginRight = "8px";
              plusBtn.style.listStyle = "none";
              plusBtn.style.alignSelf = "center"; // 縦方向中央揃え
              plusBtn.style.userSelect = "none";

              // クリックで新規の空タブを追加
              plusBtn.addEventListener("click", (e) => {
                e.stopPropagation(); // イベントの親伝播を防ぐ
                addConnectionTab(stack);
              });

              tabsContainer.appendChild(plusBtn);
            }
          }
        }, 50);
      }
    });

    // 初期起動時に最初の空タブを1つ表示するレイアウトをロード
    tabCounter++;
    const initialTabId = `new-connection-${tabCounter}`;
    layout.loadLayout({
      root: {
        type: "row",
        content: [
          {
            type: "component",
            componentType: "serial-port-tab",
            componentState: { tabId: initialTabId, portName: "" },
            title: "New Connection",
          },
        ],
      },
    });

    // ウィンドウ全体がリサイズされたとき、Golden Layout 自体を追従させる
    const handleResize = () => {
      if (layout && layoutContainer) {
        layout.setSize(
          layoutContainer.clientWidth,
          layoutContainer.clientHeight,
        );
      }
    };
    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
      if (layout) {
        layout.destroy();
      }
    };
  });
</script>

<main>
  <div class="layout-container" bind:this={layoutContainer}></div>
</main>

<style>
  main {
    width: 100vw;
    height: 100vh;
    background-color: #1e1e1e;
    margin: 0;
    overflow: hidden;
  }

  .layout-container {
    width: 100%;
    height: 100%;
  }

  /* Golden Layout の各ペインのスクロール等を抑制するためのスタイル */
  :global(.lm_content) {
    overflow: hidden;
    border: none !important;
  }

  /* ヘッダー右端のコントロール部にある全画面・閉じる・ポップアウトアイコンを非表示にする */
  :global(.lm_controls .lm_maximise),
  :global(.lm_controls .lm_close),
  :global(.lm_controls .lm_popout) {
    display: none !important;
  }

  /* ==========================================
   * タブデザイン & 「＋」ボタンのスタイリング
   * ========================================== */

  /* 1. ヘッダー全体の高さ調整 */
  :global(.lm_header) {
    height: 40px !important;
    background-color: #2d2d2d !important; /* タブバー背景 */
    box-sizing: border-box;
  }

  :global(.lm_header .lm_tabs) {
    height: 40px !important;
    display: flex !important;
    align-items: flex-end !important; /* タブを下揃えにして重ねる */
  }

  /* 2. 各タブの基本デザイン */
  :global(.lm_tab) {
    height: 32px !important; /* タブの高さを大きく */
    font-size: 0.8rem !important;
    padding: 0 12px 0 16px !important; /* 右側の余白を調整 */
    border-radius: 6px 6px 0 0 !important; /* 上部に丸み */
    background-color: #252526 !important; /* 非アクティブ時は奥に沈む色 */
    color: #969696 !important;
    border: none !important;
    margin-right: 3px !important;
    transition:
      background-color 0.15s,
      color 0.15s;
    box-sizing: border-box;

    /* Flexbox を使って、タブタイトルと閉じるボタンを綺麗に並べる */
    display: inline-flex !important;
    align-items: center !important;
    justify-content: space-between !important;
    gap: 8px !important;
  }

  /* タブ内のタイトル */
  :global(.lm_tab .lm_title) {
    display: inline-block !important;
    line-height: normal !important;
    vertical-align: middle !important;
  }

  /* 個別タブの「閉じる」ボタン (位置を修正し、高さをタイトルと揃える) */
  :global(.lm_tab .lm_close_tab),
  :global(.lm_tab .lm_close) {
    position: static !important; /* 絶対配置を解除してFlexboxに従わせる */
    width: 14px !important;
    height: 14px !important;
    display: inline-flex !important;
    align-items: center !important;
    justify-content: center !important;
    margin: 0 !important;
    padding: 0 !important;
    order: 2 !important; /* タイトルの右側に配置 */
    cursor: pointer;
    vertical-align: middle !important;
  }

  /* 非アクティブタブのホバー時 */
  :global(.lm_tab:hover:not(.lm_active)) {
    background-color: #2d2d2d !important;
    color: #cccccc !important;
  }

  /* 3. アクティブ（選択中）なタブのデザイン（最前面） */
  :global(.lm_tab.lm_active) {
    background-color: #3b3b3b !important;
    color: #ffffff !important;
    font-weight: bold;
    z-index: 2; /* 手前に表示 */
  }

  /* 4. 「＋」追加ボタン */
  :global(.lm_tab_plus) {
    background-color: transparent !important;
    color: #aaaaaa !important;
    font-size: 1.3rem !important;
    font-weight: normal !important;
    display: inline-flex !important;
    align-items: center;
    justify-content: center;
    align-self: center; /* 縦中央揃え */
    transition:
      background-color 0.2s,
      color 0.2s;
    box-sizing: border-box;
  }

  :global(.lm_tab_plus:hover) {
    background-color: #3e3e3f !important; /* ホバー時に丸い影が浮き出る */
    color: #ffffff !important;
  }
</style>
