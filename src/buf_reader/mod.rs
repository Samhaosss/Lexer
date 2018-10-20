// use two steps buf to read from file
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
mod utils;
use self::utils::*;

const BUFSIZE: usize = 2048;
const FULLBUFSIZE: usize = BUFSIZE * 2;
const FIRSTBUF: usize = 0;
const SECONDBUF: usize = 2048;

// Context will be readed into buf, if end_pos equals to current_end_pos
// , it means last buf have been comsumed and we need read content to next
// buf. Reading from file to buf is impled by read_to_buf function which
// will return OK with actual bytes it read or Err if file comes to EOF
#[derive(Debug)]
pub struct TakenReader {
    buf: Vec<u8>,
    input_file: File,
    start_pos: CirNum<usize>,
    end_pos: CirNum<usize>,
    current_buf_end: CirNum<usize>,
}

impl TakenReader {
    pub fn new(file_name: &str) -> TakenReader {
        let mut tmp = TakenReader {
            buf: Vec::with_capacity(FULLBUFSIZE),
            input_file: File::open(Path::new(file_name)).expect("TakenReader consturction failed"),
            start_pos: CirNum::new(FIRSTBUF, FULLBUFSIZE, FIRSTBUF),
            end_pos: CirNum::new(FIRSTBUF, FULLBUFSIZE, FIRSTBUF),
            current_buf_end: CirNum::new(FIRSTBUF, FULLBUFSIZE, FIRSTBUF),
        };
        // fill value into buf to allocate mem from os
        for _i in 0..4096 {
            tmp.buf.push(0u8);
        }
        tmp
    }

    fn read_to_buf(&mut self) -> io::Result<usize> {
        let read_size = match self.current_buf_end.get_value() {
            x if x == FIRSTBUF || x == SECONDBUF => {
                self.input_file.read(&mut self.buf[x..x + BUFSIZE])
            }
            _ => Err(io::Error::from(io::ErrorKind::Interrupted)), // file already came to EOF
        }?;
        self.current_buf_end.add(read_size);
        Ok(read_size)
    }

    pub fn read_byte(&mut self) -> io::Result<u8> {
        if self.end_pos == self.current_buf_end {
            self.read_to_buf()?;
        };
        let by = self.buf[self.end_pos.get_value()];
        self.end_pos.add_1();
        Ok(by)
    }

    pub fn get_word(&mut self) -> Vec<u8> {
        let mut word: Vec<u8> = Vec::new();
        match (self.start_pos.get_value(), self.end_pos.get_value()) {
            (mut sp, ep) if sp != ep => {
                while sp != ep {
                    word.push(self.buf[sp]);
                    sp = if sp != (FULLBUFSIZE - 1) { sp + 1 } else { 0 };
                }
            }
            _ => panic!("startpos == endpos"),
        }
        self.start_pos.set_value(self.end_pos.get_value());
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn read_word() {
        //let file_name = env::args().skip(1).next().unwrap();
        let file_name = String::from("Cargo.toml");
        let mut test = TakenReader::new(&file_name);
        let mut count: u32 = 0;
        loop {
            match test.read_byte() {
                Ok(u) => {
                    if u == b' ' {
                        println!(
                            "{:?}",
                            String::from_utf8(test.get_word())
                                .unwrap()
                                .replace("/n", "")
                        )
                    }
                }
                Err(_re) => break,
            }
            count += 1;
        }
        assert_ne!(0, count);
    }
}
