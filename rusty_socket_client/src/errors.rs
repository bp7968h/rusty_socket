use std::fmt;

#[derive(Debug)]
pub enum ScError {

}

impl fmt::Display for ScError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for ScError {}