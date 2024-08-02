use crate::OpCode;

enum ExtendedPayLoadLength {
    Medium(u16),
    Large(u64)
}

struct DataFrame {
    fin_rscv_opcode: u8, // 1 + 1 + 1 + 1 + 4 bits
    mask_payload_length: u8, // 1 + 7 bits
    // payload length in bytes if 0-125, this is the payload length,
    // if 26, the following 2 bytes interpreted as 16-bit unsigned integer is payload length,
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
}