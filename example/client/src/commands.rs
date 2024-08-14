use std::fmt;
use std::fmt::Formatter;

pub struct Commands<'a>(pub &'a [(&'a str, &'a str)]);

impl fmt::Display for Commands<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Usage:")?;
        for &(cmd, desc) in self.0 {
            writeln!(f, "\t{}: {}", cmd, desc)?;
        }
        Ok(())
    }
}
