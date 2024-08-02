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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fin_modification(){
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
}