use once_cell::sync::Lazy;
use std::{
    io::{Error, ErrorKind, Write},
    sync::Mutex,
};

pub static GLOBAL_STRING: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));

#[derive(Debug)]
pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Self {}
    }
}

impl Write for Logger {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        match std::str::from_utf8(buf) {
            Ok(decoded) => {
                let mut s = GLOBAL_STRING.lock().unwrap();
                s.push_str(decoded);
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
