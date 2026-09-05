import sys
from pathlib import Path

# 親ディレクトリ (Tools/VirtualDevice) を Python のモジュール検索パスの最優先に追加
vd_root = Path(__file__).resolve().parent.parent
if str(vd_root) not in sys.path:
    sys.path.insert(0, str(vd_root))

import customtkinter as ctk

from vd_core import SerialDriver, SocatManager


class VirtualDevice(ctk.CTk):
    def __init__(self) -> None:
        super().__init__()
        self.title("Virtua lDevice")
        self.geometry("500x250")

        self.socat_manager = SocatManager()
        self.serial_driver = SerialDriver()

        self.status_label = ctk.CTkLabel(
            self, text="Status : Disconnected", text_color="red"
        )
        self.status_label.pack(pady=10)

        self.port_label = ctk.CTkLabel(
            self, text="Serial Port : N/A", font=("", 16, "bold")
        )
        self.port_label.pack(pady=10)

        self.connect_btn = ctk.CTkButton(
            self, text="Connect", command=self.toggle_connect
        )
        self.connect_btn.pack(pady=20)

    def toggle_connect(self) -> None:
        if not self.serial_driver.is_open:
            try:
                mock_port, user_port = self.socat_manager.start()
                self.serial_driver.open(mock_port)

                self.status_label.configure(
                    text="Status: Connected", text_color="green"
                )

                self.port_label.configure(text=f"Serial Target Port:{user_port}")
                self.connect_btn.configure(text="Disconnect")
            except Exception as e:
                self.status_label.configure(text=f"Error: {e}", text_color="red")

        else:
            self.serial_driver.close()
            self.socat_manager.stop()

            self.status_label.configure(text="Status: Disconnected", text_color="red")
            self.port_label.configure(text="Serial Port: N/A")
            self.connect_btn.configure(text="CConnect")


def main() -> None:
    ctk.set_appearance_mode("System")
    device = VirtualDevice()
    device.mainloop()


if __name__ == "__main__":
    main()
