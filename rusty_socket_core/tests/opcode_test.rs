use rusty_socket_core::OpCode;

#[test]
fn test_u8_to_opcode() {
    assert_eq!(OpCode::from(0), OpCode::ContinuationFrame);
    assert_eq!(OpCode::from(1), OpCode::Text);
    assert_eq!(OpCode::from(2), OpCode::Binary);
    assert_eq!(OpCode::from(8), OpCode::ConnectionClose);
    assert_eq!(OpCode::from(9), OpCode::Ping);
    assert_eq!(OpCode::from(10), OpCode::Pong);
    assert_eq!(OpCode::from(255), OpCode::Unknown);
}

#[test]
fn test_opcode_to_u8() {
    assert_eq!(u8::from(OpCode::ContinuationFrame), 0);
    assert_eq!(u8::from(OpCode::Text), 1);
    assert_eq!(u8::from(OpCode::Binary), 2);
    assert_eq!(u8::from(OpCode::ConnectionClose), 8);
    assert_eq!(u8::from(OpCode::Ping), 9);
    assert_eq!(u8::from(OpCode::Pong), 10);
    assert_eq!(u8::from(OpCode::Unknown), 255);
}
