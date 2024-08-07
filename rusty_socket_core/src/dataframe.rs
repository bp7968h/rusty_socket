use std::convert::TryFrom;
use std::fmt;
use rand::RngCore;

use crate::{OpCode, RsError, ExtendedPayLoadLength};


#[derive(Debug)]
pub struct DataFrame {
    pub fin_rscv_opcode: u8, // 1 + 1 + 1 + 1 + 4 bits
    pub mask_payload_length: u8, // 1 + 7 bits
    // payload length in bytes if 0-125, this is the payload length,
    // if 126, the following 2 bytes interpreted as 16-bit unsigned integer is payload length,
    // if 127 the following 8 bytes interpreted as 62-bit unsigne integer is payload length
    pub extended_payload_length: Option<ExtendedPayLoadLength>, // 16 or 64 bits or None
    pub masking_key: Option<[u8; 4]>, // 0 or 32-bit, present if mask bit is 1 else absent
    pub payload: Vec<u8> // arbitary length
}

impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:02x} 0x{:02x} ", self.fin_rscv_opcode, self.mask_payload_length)?;
        if let Some(extended_payload_length) = &self.extended_payload_length {
            write!(f, "{} ", extended_payload_length)?;
        }else {
            write!(f, "None ")?;
        }
        
        if let Some(masking_key) = self.masking_key {
            for i in masking_key.iter() {
            write!(f, "0x{:02x} ", i)?;   
        }
        } else {
            write!(f, "None ")?;
        }
        
        for i in self.payload.iter() {
            write!(f, "0x{:02x} ", i)?;   
        }
        
        Ok(())
    }
}

impl DataFrame {
    pub fn from_data<T: AsRef<[u8]>>(data: T, opcode: OpCode, mask: bool) -> Option<Self> {
        let data_bytes : &[u8] = data.as_ref();
        let data_length = data_bytes.len();
        
        if !opcode.isValid() {
            return None;
        }
        
        let fin_rscv_opcode : u8 = 0b10000000 | u8::from(opcode);
        let mut masking_key: Option<[u8; 4]> = None;
        let mut mask_payload_length: u8 = if mask {
            let mut random_bytes = [0u8; 4];
            rand::thread_rng().fill_bytes(&mut random_bytes);
            masking_key = Some(random_bytes);
            0b10000000
        }else {
            0b0000000
        };
        
        let mut extended_payload_length : Option<ExtendedPayLoadLength> = None;
        match data_length {
            0..=125 => {
                mask_payload_length = mask_payload_length | data_length as u8;
            },
            126..=65535 => {
                mask_payload_length = mask_payload_length | 126u8;
                extended_payload_length = Some(ExtendedPayLoadLength::Medium(data_length as u16));
            },
            _ => {
                mask_payload_length = mask_payload_length | 127u8;
                extended_payload_length = Some(ExtendedPayLoadLength::Large(data_length as u64));
            }
        }
        
        let mut frame = DataFrame {
            fin_rscv_opcode,
            mask_payload_length,
            extended_payload_length,
            masking_key,
            payload: data_bytes.to_vec(),
        };
        
        frame.apply_mask();
        
        Some(frame)
    }
    
    pub fn is_final_fragment(&self) -> bool {
        ((self.fin_rscv_opcode >> 7) & 1) != 0
    }

    pub fn set_final_fragment(&mut self){
        self.fin_rscv_opcode = self.fin_rscv_opcode | 0b10000000;
    }
    
    pub fn unset_final_fragment(&mut self){
        self.fin_rscv_opcode = self.fin_rscv_opcode & 0b01111111;
    }

    pub fn is_masked(&self) -> bool {
        ((self.mask_payload_length >> 7) & 1) != 0
    }
    
    pub fn set_masked(&mut self) {
        self.mask_payload_length = self.mask_payload_length | 0b10000000;
    }
    
    pub fn unset_masked(&mut self) {
        self.mask_payload_length = self.mask_payload_length & 0b01111111;
    }

    pub fn get_opcode(&self) -> OpCode {
        let opcode_bits: u8 = self.fin_rscv_opcode & 0b00001111;
        
        OpCode::from(opcode_bits)
    }

    pub fn is_control_frame(&self) -> bool {
        let op_code = u8::from(self.get_opcode());
        
        (op_code >> 3) & 1 != 0
    }

    pub fn get_payload_length(&self) -> u8 {
        let payload_length_bits : u8 = self.mask_payload_length & 0b01111111;
        
        payload_length_bits
    }

    pub fn apply_mask(&mut self) {
        if let Some(masking_key) = self.masking_key {
            for (i, byte) in self.payload.iter_mut().enumerate() {
                *byte ^= masking_key[i % 4];
            }
        }
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

        frame.apply_mask();

        Ok(frame)
    }
}