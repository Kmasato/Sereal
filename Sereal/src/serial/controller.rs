use super::types::BaudRate;
use getset::{Getters, MutGetters};
use serialport;
use std::sync::atomic::Ordering;
use std::sync::{Arc, atomic::AtomicBool, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::u32;

#[derive(Getters, MutGetters)]
pub struct Controller {
    #[get = "pub"]
    #[get_mut = "pub"]
    port_name: String,
    #[get = "pub"]
    #[get_mut = "pub"]
    baud_rate: BaudRate,
    is_running_thread: Arc<AtomicBool>,
    is_available_port: Arc<AtomicBool>,
    pub receiver: Option<mpsc::Receiver<String>>, // とりあえず
    read_thread_handle: Option<JoinHandle<()>>,   // スレッドハンドル
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            port_name: String::default(),
            baud_rate: BaudRate::BaudRate115200, // TODO: serialport::SerialPortを用意し、そっちで管理する
            is_running_thread: Arc::default(),
            is_available_port: Arc::default(),
            receiver: None,
            read_thread_handle: None,
        }
    }
}

impl Controller {
    pub fn activate(&mut self) -> Result<(), serialport::Error> {
        let is_running_thread = Arc::new(AtomicBool::new(true));
        self.is_running_thread = is_running_thread.clone();

        let is_available_port = Arc::new(AtomicBool::new(false));
        self.is_available_port = is_available_port.clone();

        let (sender, receiver) = mpsc::channel();
        self.receiver = Some(receiver);

        let port_name = self.port_name.clone();
        let baud_rate = self.baud_rate as u32;

        let handle = thread::spawn(move || {
            connection_thread_main(
                port_name,
                baud_rate,
                is_running_thread,
                is_available_port,
                sender,
            );
        });

        self.read_thread_handle = Some(handle);
        println!("Connect {}", self.port_name);
        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.is_running_thread.store(false, Ordering::Relaxed);
        self.is_available_port.store(false, Ordering::Relaxed);

        if let Some(handle) = self.read_thread_handle.take() {
            // .join()はスレッドの終了を待ち、リソースをクリーンアップする
            if handle.join().is_err() {
                eprintln!("Read thread panicked")
            }
        }

        self.receiver = None;
        println!("Disconnected {}", self.port_name);
    }

    pub fn is_activate(&self) -> bool {
        self.is_running_thread.load(Ordering::Relaxed)
    }

    pub fn is_physical_connected(&self) -> bool {
        self.is_available_port.load(Ordering::Relaxed)
    }

    pub fn get_port_name(&self) -> String {
        self.port_name.clone()
    }
}

fn connection_thread_main(
    port_name: String,
    baud_rate: u32,
    is_running_thread: Arc<AtomicBool>,
    is_available_port: Arc<AtomicBool>,
    sender: mpsc::Sender<String>,
) {
    const RETRY_INTERVAL_MS: u64 = 500;
    let retry_interval = Duration::from_millis(RETRY_INTERVAL_MS);

    while is_running_thread.load(Ordering::Relaxed) {
        let mut port = match serialport::new(&port_name, baud_rate).open() {
            Ok(p) => {
                is_available_port.store(true, Ordering::Relaxed);
                println!("Port {} opened", port_name);
                p // 開いたポートを返す
            }
            Err(_) => {
                thread::sleep(retry_interval);
                continue;
            }
        };

        // ポートが開いている間、通信を続ける
        while is_available_port.load(Ordering::Relaxed) {
            match port.bytes_to_read() {
                Ok(bytes_to_read) if 0 < bytes_to_read => {
                    let mut receive_buffer = vec![0; bytes_to_read as usize];
                    match port.read(&mut receive_buffer) {
                        Ok(got_bytes) => {
                            let received =
                                String::from_utf8_lossy(&receive_buffer[..got_bytes]).to_string();
                            if sender.send(received).is_err() {
                                break;
                            };
                        }
                        Err(e) => {
                            match e.kind() {
                                std::io::ErrorKind::TimedOut => {
                                    // 何もしない
                                    continue;
                                }
                                _ => {
                                    eprintln!("Read Error: {e}");
                                    continue;
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    // デバイスと通信できなかった
                    is_available_port.store(false, Ordering::Relaxed);
                    break;
                }
                _ => {
                    // 0 バイトが返ってきた場合
                }
            }
        }
    }
}
