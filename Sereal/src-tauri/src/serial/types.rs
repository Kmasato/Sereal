use serde::Serialize;
use std::fmt;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub enum BaudRate {
    BaudRate9600 = 9600,
    #[default]
    BaudRate115200 = 115200,
}

impl BaudRate {
    pub fn iter() -> impl Iterator<Item = BaudRate> {
        [BaudRate::BaudRate9600, BaudRate::BaudRate115200]
            .iter()
            .copied()
    }

    pub fn from_u32(baud_rate: u32) -> Self {
        match baud_rate {
            9600 => BaudRate::BaudRate9600,
            115200 => BaudRate::BaudRate115200,
            _ => BaudRate::default(),
        }
    }
}

impl fmt::Display for BaudRate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rate = match self {
            BaudRate::BaudRate9600 => 9600,
            BaudRate::BaudRate115200 => 115200,
        };
        write!(f, "{}", rate)
    }
}

#[derive(Default, Clone)]
pub struct ReceivedData {
    pub id: u64,
    pub text: String,
}

pub const MAX_RECEIVED_DATA_SIZE: usize = 2048;

#[derive(Serialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionStatus {
    Connected,            // 接続状態 (通信可能)
    Disconnected,         // アイドル状態 (物理的に接続、受信を停止)
    PhysicalDisconnected, // 切断状態 (物理的に切断)
}
