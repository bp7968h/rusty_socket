use std::fmt;

#[derive(Debug)]
pub enum ExtendedPayLoadLength {
    Medium(u16),
    Large(u64),
}

impl fmt::Display for ExtendedPayLoadLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExtendedPayLoadLength::Medium(data) => write!(f, "2byte-{}", data),
            ExtendedPayLoadLength::Large(data) => write!(f, "8byte-{}", data),
        }
    }
}

impl ExtendedPayLoadLength {
    pub fn get_size(&self) -> usize {
        match self {
            ExtendedPayLoadLength::Medium(_) => 2,
            ExtendedPayLoadLength::Large(_) => 8,
        }
    }

    pub fn get_value(&self) -> usize {
        match self {
            ExtendedPayLoadLength::Medium(data) => *data as usize,
            ExtendedPayLoadLength::Large(data) => *data as usize,
        }
    }
}
