from __future__ import annotations

import serial


class SerialDriver:
    def __init__(self) -> None:
        self._port: serial.Serial | None = None

    def open(
        self, port_name: str, baudrate: int = 115200, timeout: float = 0.1
    ) -> None:
        """指定したポート名のポートをオープンします"""
        if self.is_open:
            self.close()

        self._port = serial.Serial(port_name, baudrate=baudrate, timeout=timeout)

    def close(self) -> None:
        """指定したポート名のポートをクローズします"""
        if self._port is not None:
            if self._port.is_open:
                self._port.close()
            self._port = None

    @property
    def is_open(self) -> bool:
        """ポートがオープン済みかどうか"""
        return self._port is not None and self._port.is_open

    def write(self, data: bytes) -> int:
        """データを送信する"""
        if self._port is None or not self._port.is_open:
            raise RuntimeError("Serial port is not open.")

        written = self._port.write(data)
        self._port.flush()
        if written is not None:
            return written

        _ = RuntimeWarning("Failed to write data")
        return 0

    def read(self, size: int = 1024) -> bytes:
        """データを受信する"""
        if self._port is None or not self._port.is_open:
            raise RuntimeError("Serial port is not open.")

        return self._port.read(size)

    def read_until(self, expected: bytes = b"\n", size: int | None = None) -> bytes:
        """指定区切り文字まで読み込む(デフォルト\n)"""
        if self._port is None or not self._port.is_open:
            raise RuntimeError("Serial port is not open.")

        return self._port.read_until(expected=expected, size=size)
