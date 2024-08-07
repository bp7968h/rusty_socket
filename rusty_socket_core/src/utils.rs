use std::fmt;

#[derive(Debug)]
pub enum ExtendedPayLoadLength {
    Medium(u16),
    Large(u64)
}

impl fmt::Display for ExtendedPayLoadLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExtendedPayLoadLength::Medium(data) => write!(f, "2byte-{}", data),
            ExtendedPayLoadLength::Large(data) => write!(f, "8byte-{}", data),
        }
    }
}