// タブで選択されているポート情報を管理するためのクラス
class PortStore {
  // key: clientId, value: selectedPort
  private tabPorts = $state<Map<string, string>>(new Map());

  // Client ID (タブ)と Port 情報の組みの登録
  // タブ側でポートを選択、変更した時に登録
  setPort(clientId: string, portName: string) {
    if (portName) {
      this.tabPorts.set(clientId, portName);
    }
  }

  // 登録した情報の削除
  // タブを閉じた際に呼び出す
  removePort(clientId: string) {
    this.tabPorts.delete(clientId);
  }

  getUserPortsExcept(currentClientId: string): Set<string> {
    const used = new Set<string>();
    for (const [id, port] of this.tabPorts.entries()) {
      if (id !== currentClientId && port) {
        used.add(port);
      }
    }
    return used;
  }
}

export const portStore = new PortStore();
