use std::io::{Read, Write};

pub trait IO {
    fn read(&mut self) -> u8;
    fn write(&mut self, b: u8);
}

pub struct Stdio;

impl IO for Stdio {
    fn read(&mut self) -> u8 {
        let mut buf = [0];
        match std::io::stdin().lock().read_exact(buf.as_mut()) {
            Ok(_) => buf[0],
            Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => 0,
            Err(err) => panic!("{}", err),
        }
    }

    fn write(&mut self, b: u8) {
        std::io::stdout().lock().write_all([b].as_ref()).unwrap()
    }
}
