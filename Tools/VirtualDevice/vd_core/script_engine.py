from __future__ import annotations

import importlib.util
import sys
import threading
import traceback
from typing import TYPE_CHECKING, Callable

from serial.serialposix import Serial

if TYPE_CHECKING:
    from pathlib import Path

    from vd_core.serial_driver import SerialDriver

LogCallback = Callable[[str], None]


class VirtualSerialPort:
    def __init__(self, driver: SerialDriver) -> None:
        self._driver: SerialDriver = driver

    def write(self, data: bytes) -> int:
        return self._driver.write(data)

    def read(self, size: int = 1024) -> bytes:
        return self._driver.read(size)

    def read_until(self, expected: bytes = b"\n") -> bytes:
        return self._driver.read_until(expected)


class ScriptEngine:
    def __init__(
        self, driver: SerialDriver, log_callback: LogCallback | None = None
    ) -> None:
        self._driver: SerialDriver = driver
        self._log_callback: LogCallback | None = log_callback
        self._thread: threading.Thread | None = None
        self._stop_event: threading.Event = threading.Event()

    def _log(self, message: str) -> None:
        if self._log_callback:
            self._log_callback(message)
        else:
            print(f"[ScriptEngine] {message}")

    @property
    def is_running(self) -> bool:
        return self._thread is not None and self._thread.is_alive()

    def run_script(self, script_path: Path) -> None:
        """Path で指定したスクリプトを動的にロードし、別スレッドで実行します"""
        if self.is_running:
            raise RuntimeError("Script is already running.")

        module_name = script_path.stem
        spec = importlib.util.spec_from_file_location(module_name, script_path)
        if spec is None or spec.loader is None:
            raise ImportError(f"Could not load script spec from {script_path}")

        module = importlib.util.module_from_spec(spec)
        sys.modules[module_name] = module
        spec.loader.exec_module(module)

        if not hasattr(module, "main") or not callable(module.main):
            raise AttributeError(
                f"Script'{script_path.name}' must define a 'main(port: VirtualSerialPort)' function."
            )

        main_func = module.main

        self._stop_event.clear()
        port_interface = VirtualSerialPort(self._driver)

        def worker() -> None:
            self._log(f"Started running script:{script_path.name}")
            try:
                main_func(port_interface)
                self._log(f"Finished script: {script_path.name}")
            except Exception as e:
                self._log(f"Script execution error :{e}\n{traceback.format_exc()}")
            finally:
                self._thread = None

        self._thread = threading.Thread(target=worker, daemon=True)
        self._thread.start()

    def stop_script(self) -> None:
        """実行中のスクリプト停止フラグをセットします"""
        self._stop_event.set()
