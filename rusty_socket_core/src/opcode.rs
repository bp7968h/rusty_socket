#[derive(Debug, PartialEq)]
pub enum OpCode {
    ContinuationFrame,
    Text,
    Binary,
    ConnectionClose,
    Ping,
    Pong,
    Unknown,
}

impl OpCode {
    pub fn is_valid(&self) -> bool {
        match self {
            OpCode::Unknown => false,
            _ => true,
        }
    }
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0 => OpCode::ContinuationFrame,
            1 => OpCode::Text,
            2 => OpCode::Binary,
            8 => OpCode::ConnectionClose,
            9 => OpCode::Ping,
            10 => OpCode::Pong,
            _ => OpCode::Unknown,
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::ContinuationFrame => 0,
            OpCode::Text => 1,
            OpCode::Binary => 2,
            OpCode::ConnectionClose => 8,
            OpCode::Ping => 9,
            OpCode::Pong => 10,
            OpCode::Unknown => 255,
        }
    }
}
