use super::types::BaudRate;
use getset::{Getters, MutGetters};
use serialport;
use std::sync::Mutex;
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
    is_available_port: Arc<Mutex<Option<bool>>>, // ポートとのアクセスの可否と未試行を区別するためにOptionで宣言
    pub receiver: Option<mpsc::Receiver<String>>, // とりあえず
    read_thread_handle: Option<JoinHandle<()>>,  // スレッドハンドル
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

        let is_available_port = Arc::new(Mutex::new(None));
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
        if let Some(handle) = self.read_thread_handle.take() {
            // .join()はスレッドの終了を待ち、リソースをクリーンアップする
            if handle.join().is_err() {
                eprintln!("Read thread panicked")
            }
        }

        let mut is_available = self.is_available_port.lock().unwrap();
        *is_available = None;

        self.receiver = None;
        println!("Disconnected {}", self.port_name);
    }

    pub fn is_activate(&self) -> bool {
        let is_available = self.is_available_port.lock().unwrap();
        match *is_available {
            None => false,
            Some(_) => self.is_running_thread.load(Ordering::Relaxed),
        }
    }

    pub fn is_physical_connected(&self) -> bool {
        let is_available = self.is_available_port.lock().unwrap();
        is_available.unwrap_or(false)
    }

    pub fn get_port_name(&self) -> String {
        self.port_name.clone()
    }
}

fn connection_thread_main(
    port_name: String,
    baud_rate: u32,
    is_running_thread: Arc<AtomicBool>,
    is_available_port: Arc<Mutex<Option<bool>>>,
    sender: mpsc::Sender<String>,
) {
    const RETRY_INTERVAL_MS: u64 = 500;
    let retry_interval = Duration::from_millis(RETRY_INTERVAL_MS);

    while is_running_thread.load(Ordering::Relaxed) {
        let mut port = match serialport::new(&port_name, baud_rate).open() {
            Ok(p) => {
                let mut is_available = is_available_port.lock().unwrap();
                *is_available = Some(true);
                println!("Port {} opened", port_name);
                p // 開いたポートを返す
            }
            Err(_) => {
                thread::sleep(retry_interval);
                let mut is_available = is_available_port.lock().unwrap();
                *is_available = Some(false);
                continue;
            }
        };

        // ポートが開いている間、通信を続ける
        while {
            is_running_thread.load(Ordering::Relaxed) && {
                let guard = is_available_port.lock().unwrap();
                matches!(*guard, Some(true))
            }
        } {
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
                    let mut is_available = is_available_port.lock().unwrap();
                    *is_available = Some(false);
                    break;
                }
                _ => {
                    // 0 バイトが返ってきた場合
                }
            }
        }
    }
}
