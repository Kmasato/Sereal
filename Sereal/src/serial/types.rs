use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BaudRate {
    BaudRate9600 = 9600,
    BaudRate115200 = 115200,
}

impl BaudRate {
    pub fn iter() -> impl Iterator<Item = BaudRate> {
        [BaudRate::BaudRate9600, BaudRate::BaudRate115200]
            .iter()
            .copied()
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
