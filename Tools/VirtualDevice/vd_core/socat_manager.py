import subprocess
import time


class SocatManager:
    def __init__(self) -> None:
        self.process: subprocess.Popen[str] | None = None
        self.mock_port: str | None = None
        self.user_port: str | None = None

    def start(self) -> tuple[str, str]:
        if self.process is not None:
            raise RuntimeError("Soccat is already running.")
        cmd = ["socat", "-d", "-d", "pty,raw,echo=0", "pty,raw,echo=0"]
        self.process = subprocess.Popen(cmd, stderr=subprocess.PIPE, text=True)

        # ★ ヌルチェックを追加して Basedpyright の警告を解消
        if self.process.stderr is None:
            raise RuntimeError("Failed to open stderr stream of socat.")

        self.mock_port = "/tmp/vd_app"
        self.user_port = "/tmp/vd_port"

        cmd = [
            "socat",
            "-d",
            "-d",
            f"pty,raw,echo=0,link={self.mock_port}",
            f"pty,raw,echo=0,link={self.user_port}",
        ]
        self.process = subprocess.Popen(cmd, stderr=subprocess.PIPE, text=True)
        time.sleep(0.5)

        return self.mock_port, self.user_port

    def stop(self) -> None:
        if self.process is not None:
            self.process.terminate()
            try:
                _ = self.process.wait(timeout=2.0)
            except subprocess.TimeoutExpired:
                self.process.kill()

            self.process = None
            self.mock_port = None
            self.user_port = None
