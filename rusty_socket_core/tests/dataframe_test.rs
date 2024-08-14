use rusty_socket_core::DataFrame;
use rusty_socket_core::OpCode;
use rusty_socket_core::RsError;

#[test]
fn test_fin_bit_modification() {
    let mut frame = DataFrame {
        fin_rscv_opcode: 0b00101011,
        mask_payload_length: 141,
        extended_payload_length: None,
        masking_key: None,
        payload: Vec::new(),
    };
    frame.set_final_fragment();
    assert_eq!(true, frame.is_final_fragment());

    frame.unset_final_fragment();
    assert_eq!(false, frame.is_final_fragment());
}

#[test]
fn test_mask_bit_modification() {
    let mut frame = DataFrame {
        fin_rscv_opcode: 0b00101011,
        mask_payload_length: 0b10010111,
        extended_payload_length: None,
        masking_key: None,
        payload: Vec::new(),
    };
    frame.unset_masked();
    assert_eq!(false, frame.is_masked());

    frame.set_masked();
    assert_eq!(true, frame.is_masked());
}

#[test]
fn test_opcode_parser() {
    let frame = DataFrame {
        fin_rscv_opcode: 0b00101010,
        mask_payload_length: 0b10010111,
        extended_payload_length: None,
        masking_key: None,
        payload: Vec::new(),
    };
    assert_eq!(OpCode::from(10), frame.get_opcode());
    assert_eq!(OpCode::Pong, frame.get_opcode());
}

#[test]
fn test_control_frame() {
    let frame = DataFrame {
        fin_rscv_opcode: 0b00101000,
        mask_payload_length: 0b10010111,
        extended_payload_length: None,
        masking_key: None,
        payload: Vec::new(),
    };
    assert_eq!(true, frame.is_control_frame());
}

#[test]
fn test_payload_length() {
    let frame = DataFrame {
        fin_rscv_opcode: 0b00101000,
        mask_payload_length: 0b10010111,
        extended_payload_length: None,
        masking_key: None,
        payload: Vec::new(),
    };
    assert_eq!(23, frame.get_payload_length());
}

#[test]
fn test_valid_frame() {
    // FIN + Text, Not Mask + 1-byte payload length, payload: 1
    let raw_data: &[u8] = &[0b10000001, 0b00000001, 1];
    let frame: DataFrame = raw_data.try_into().expect("Failed to deserialize");

    assert_eq!(frame.fin_rscv_opcode, 0b10000001);
    assert_eq!(frame.mask_payload_length, 0b00000001);
    assert_eq!(frame.payload, vec![1]);
}

#[test]
fn test_valid_frame_string() {
    let raw_data: &[u8] = &[0x81, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f];

    let frame: DataFrame = raw_data.try_into().expect("Failed to deserialize");
    assert_eq!(
        String::from_utf8(frame.payload).unwrap(),
        "Hello".to_string()
    );
}

#[test]
fn test_valid_frame_string_masked() {
    let raw_data: &[u8] = &[
        0x81, 0x85, 0x37, 0xfa, 0x21, 0x3d, 0x7f, 0x9f, 0x4d, 0x51, 0x58,
    ];

    let frame: DataFrame = raw_data.try_into().expect("Failed to deserialize");

    assert_eq!(
        String::from_utf8(frame.payload).unwrap(),
        "Hello".to_string()
    );
}

#[test]
fn test_invalid_frame_insufficient_data() {
    let raw_data: &[u8] = &[0b10000001]; // Only 1 byte
    let result: Result<DataFrame, RsError> = raw_data.try_into();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Insufficient Data");
}
