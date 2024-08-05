use std::convert::TryFrom;

use crate::{OpCode, RsError};

#[derive(Debug)]
enum ExtendedPayLoadLength {
    Medium(u16),
    Large(u64)
}

#[derive(Debug)]
struct DataFrame {
    fin_rscv_opcode: u8, // 1 + 1 + 1 + 1 + 4 bits
    mask_payload_length: u8, // 1 + 7 bits
    // payload length in bytes if 0-125, this is the payload length,
    // if 126, the following 2 bytes interpreted as 16-bit unsigned integer is payload length,
    // if 127 the following 8 bytes interpreted as 62-bit unsigne integer is payload length
    extended_payload_length: Option<ExtendedPayLoadLength>, // 16 or 64 bits or None
    masking_key: Option<[u8; 4]>, // 0 or 32-bit, present if mask bit is 1 else absent
    payload: Vec<u8> // arbitary length
}

impl DataFrame {
    fn is_final_fragment(&self) -> bool {
        ((self.fin_rscv_opcode >> 7) & 1) != 0
    }

    fn set_final_fragment(&mut self){
        self.fin_rscv_opcode = self.fin_rscv_opcode | 0b10000000;
    }
    
    fn unset_final_fragment(&mut self){
        self.fin_rscv_opcode = self.fin_rscv_opcode & 0b01111111;
    }

    fn is_masked(&self) -> bool {
        ((self.mask_payload_length >> 7) & 1) != 0
    }
    
    fn set_masked(&mut self) {
        self.mask_payload_length = self.mask_payload_length | 0b10000000;
    }
    
    fn unset_masked(&mut self) {
        self.mask_payload_length = self.mask_payload_length & 0b01111111;
    }

    fn get_opcode(&self) -> OpCode {
        let opcode_bits: u8 = self.fin_rscv_opcode & 0b00001111;
        
        OpCode::from(opcode_bits)
    }

    fn is_control_frame(&self) -> bool {
        let op_code = u8::from(self.get_opcode());
        
        (op_code >> 3) & 1 != 0
    }

    fn get_payload_length(&self) -> u8 {
        let payload_length_bits : u8 = self.mask_payload_length & 0b01111111;
        
        payload_length_bits
    }

    fn apply_mask(&mut self) {
        if let Some(masking_key) = self.masking_key {
            for (i, byte) in self.payload.iter_mut().enumerate() {
                *byte ^= masking_key[i % 4];
            }
        }
    }

    fn unmask_payload(&mut self) {
        self.apply_mask()
    }
}

impl TryFrom<&[u8]> for DataFrame {
    type Error = RsError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data.len() < 2 {
            return Err(RsError::IncompleteData);
        }

        let fin_rscv_opcode = data[0];
        let mask_payload_length = data[1];

        if (fin_rscv_opcode >> 7) & 1 != 1 {
            return Err(RsError::FragmentationNotSupported);
        }
        if (fin_rscv_opcode & 0b00001111) > 0xA {
            return Err(RsError::InvalidOpCode);
        }
        
        let payload_length_indicator : u8 = data[1] & 0b01111111;
        let extended_payload_length = match payload_length_indicator {
            126 => {
                if data.len() < 4 {
                    return Err(RsError::IncompleteData);
                }
                let mut bytes = [0u8; 2];
                bytes.copy_from_slice(&data[2..=3]);
                Some(ExtendedPayLoadLength::Medium(u16::from_be_bytes(bytes)))
            },
            127 => {
                if data.len() < 10 {
                    return Err(RsError::IncompleteData);
                }
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&data[2..10]);
                Some(ExtendedPayLoadLength::Large(u64::from_be_bytes(bytes)))
            },
            _ => None
        };

        let mut payload_start = match extended_payload_length {
            Some(ExtendedPayLoadLength::Medium(_)) => 4,
            Some(ExtendedPayLoadLength::Large(_)) => 10,
            None => 2,
        };

        let is_masked : bool = (mask_payload_length & 0b10000000) != 0;
        let masking_key = if is_masked {
            if data.len() < payload_start + 4 {
                return Err(RsError::IncompleteData);
            }
            let mut key = [0u8; 4];
            key.copy_from_slice(&data[payload_start..payload_start + 4]);
            payload_start += 4;
            Some(key)
        } else {
            None
        };

        let payload_length = match extended_payload_length {
            Some(ExtendedPayLoadLength::Medium(len)) => len as usize,
            Some(ExtendedPayLoadLength::Large(len)) => len as usize,
            None => payload_length_indicator as usize,
        };

        if data.len() < payload_start + payload_length {
            return Err(RsError::IncompleteData);
        }

        let mut payload = data[payload_start..payload_start + payload_length].to_vec();

        let mut frame = DataFrame {
            fin_rscv_opcode,
            mask_payload_length,
            extended_payload_length,
            masking_key,
            payload,
        };

        if is_masked {
            frame.apply_mask();
        }

        Ok(frame)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fin_bit_modification(){
        let mut frame = DataFrame {
            fin_rscv_opcode: 0b00101011,
            mask_payload_length: 141,
            extended_payload_length: None,
            masking_key: None,
            payload: Vec::new()
        };

        frame.set_final_fragment();
        assert_eq!(true, frame.is_final_fragment());
        
        frame.unset_final_fragment();
        assert_eq!(false, frame.is_final_fragment());
    }

    #[test]
    fn test_mask_bit_modification(){
        let mut frame = DataFrame {
            fin_rscv_opcode: 0b00101011,
            mask_payload_length: 0b10010111,
            extended_payload_length: None,
            masking_key: None,
            payload: Vec::new()
        };

        frame.unset_masked();
        assert_eq!(false, frame.is_masked());
        
        frame.set_masked();
        assert_eq!(true, frame.is_masked());
    }

    #[test]
    fn test_opcode_parser(){
        let frame = DataFrame {
            fin_rscv_opcode: 0b00101010,
            mask_payload_length: 0b10010111,
            extended_payload_length: None,
            masking_key: None,
            payload: Vec::new()
        };

        assert_eq!(OpCode::from(10), frame.get_opcode());
        assert_eq!(OpCode::Pong, frame.get_opcode());
    }

    #[test]
    fn test_control_frame(){
        let frame = DataFrame {
            fin_rscv_opcode: 0b00101000,
            mask_payload_length: 0b10010111,
            extended_payload_length: None,
            masking_key: None,
            payload: Vec::new()
        };

        assert_eq!(true, frame.is_control_frame());
    }

    #[test]
    fn test_payload_length(){
        let frame = DataFrame {
            fin_rscv_opcode: 0b00101000,
            mask_payload_length: 0b10010111,
            extended_payload_length: None,
            masking_key: None,
            payload: Vec::new()
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
        let raw_data : &[u8] = &[0x81, 0x05, 0x48, 0x65, 0x6c, 0x6c, 0x6f];
        
        let frame: DataFrame = raw_data.try_into().expect("Failed to deserialize");

        assert_eq!(String::from_utf8(frame.payload).unwrap(), "Hello".to_string());
    }

    #[test]
    fn test_valid_frame_string_masked() {
        let raw_data : &[u8] = &[0x81, 0x85, 0x37, 0xfa, 0x21, 0x3d, 0x7f, 0x9f, 0x4d, 0x51, 0x58];
        
        let frame: DataFrame = raw_data.try_into().expect("Failed to deserialize");
        
        assert_eq!(String::from_utf8(frame.payload).unwrap(), "Hello".to_string());
    }

    #[test]
    fn test_invalid_frame_insufficient_data() {
        let raw_data: &[u8] = &[0b10000001]; // Only 1 byte

        let result: Result<DataFrame, RsError> = raw_data.try_into();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Insufficient Data");
    }
}