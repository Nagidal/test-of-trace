use once_cell::sync::Lazy;
use std::{
    io::{Error, ErrorKind, Write},
    sync::Mutex,
};
use tracing::Level;

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

pub fn clear() {
    let mut s = GLOBAL_STRING.lock().unwrap();
    s.clear()
}

pub fn level_contains(level: &Level, pattern: &str) -> Result<bool, std::io::Error> {
    let s = GLOBAL_STRING.lock().unwrap();
    Ok(s.lines()
        .filter(|l| l.starts_with(&("{\"level\":\"".to_owned() + level.as_str())))
        .any(|l| l.contains(pattern)))
}
