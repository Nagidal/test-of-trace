use std::io::{Error, ErrorKind, Write};
use tracing::{info, trace, Level};

pub fn add(left: usize, right: usize) -> usize {
    trace!("I'm adding");
    let result = left + right;
    info!("The result is {}", result);
    result
}

pub fn print_levels() {
    for level in [
        Level::TRACE,
        Level::DEBUG,
        Level::INFO,
        Level::WARN,
        Level::ERROR,
    ] {
        println!("{}", level.as_str());
    }
}

#[derive(Debug)]
pub struct LogBuffer {
    data: String,
}

impl LogBuffer {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }
}

impl Write for LogBuffer {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        match std::str::from_utf8(buf) {
            Ok(decoded) => {
                self.data.push_str(decoded);
                Ok(buf.len())
            }
            Err(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "could not decode data as utf8",
            )),
        }
    }
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
