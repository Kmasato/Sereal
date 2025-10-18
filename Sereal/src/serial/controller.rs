use super::types::BaudRate;
use getset::{Getters, MutGetters};
use serialport;
use std::sync::atomic::Ordering;
use std::sync::{Arc, atomic::AtomicBool, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Getters, MutGetters)]
pub struct Controller {
    #[get = "pub"]
    #[get_mut = "pub"]
    port_name: String,
    #[get = "pub"]
    #[get_mut = "pub"]
    baud_rate: BaudRate,
    is_running_thread: Arc<AtomicBool>,
    pub receiver: Option<mpsc::Receiver<String>>, // とりあえず
    read_thread_handle: Option<JoinHandle<()>>,   // スレッドハンドル
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            port_name: String::default(),
            baud_rate: BaudRate::BaudRate115200, // TODO: serialport::SerialPortを用意し、そっちで管理する
            is_running_thread: Arc::default(),
            receiver: None,
            read_thread_handle: None,
        }
    }
}

impl Controller {
    pub fn connect(&mut self) -> Result<(), serialport::Error> {
        let serial_port = serialport::new(self.port_name.clone(), self.baud_rate as u32)
            .timeout(Duration::from_millis(10));

        // port の Open に失敗した場合は ? 演算子で即座にエラーを返す
        let mut port = serial_port.open()?;

        let (sender, receiver) = mpsc::channel();
        let is_running = Arc::new(AtomicBool::new(true));
        self.is_running_thread = is_running.clone();
        self.receiver = Some(receiver);

        let handle = thread::spawn(move || {
            while is_running.load(Ordering::Relaxed) {
                if let Ok(bytes_to_read) = port.bytes_to_read() {
                    // 0 バイトであれば何もしない
                    if bytes_to_read <= 0 {
                        continue;
                    }

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
            }
        });

        self.read_thread_handle = Some(handle);

        println!("Connect {}", self.port_name);

        Ok(())
    }

    pub fn disconnect(&mut self) {
        self.is_running_thread.store(false, Ordering::Relaxed);

        if let Some(handle) = self.read_thread_handle.take() {
            // .join()はスレッドの終了を待ち、リソースをクリーンアップする
            if handle.join().is_err() {
                eprintln!("Read thread panicked")
            }
        }

        self.receiver = None;
        println!("Disconnected {}", self.port_name);
    }

    pub fn is_connect(&self) -> bool {
        self.is_running_thread.load(Ordering::Relaxed)
    }

    pub fn get_port_name(&self) -> String {
        self.port_name.clone()
    }
}
