// use two steps buf to read from file
use super::utils::*;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

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
    buf: Vec<u8>, //buf
    input_file: File,
    start_pos: CirNum<usize>,
    end_pos: CirNum<usize>,
    current_buf_end: CirNum<usize>,
}

impl TakenReader {
    pub fn new(file_name: &str) -> io::Result<TakenReader> {
        let mut tmp = TakenReader {
            buf: Vec::with_capacity(FULLBUFSIZE),
            input_file: File::open(Path::new(file_name))?,
            start_pos: CirNum::new(FIRSTBUF, FULLBUFSIZE, FIRSTBUF),
            end_pos: CirNum::new(FIRSTBUF, FULLBUFSIZE, FIRSTBUF),
            current_buf_end: CirNum::new(FIRSTBUF, FULLBUFSIZE, FIRSTBUF),
        };
        // fill value into buf to allocate mem from os
        for _i in 0..4096 {
            tmp.buf.push(0u8);
        }
        Ok(tmp)
    }

    fn read_to_buf(&mut self) -> io::Result<usize> {
        // if self.start_pos.get_value() > self.current_buf_end.get_value() ||
        // self.start_pos.get_value() < self.current_buf_end.get_value() + BUFSIZE{
        //     panic!("words too long");
        // }
        // bugs here
        if (self.start_pos != self.end_pos
            && (self.start_pos.get_value() >= SECONDBUF
                && self.current_buf_end.get_value() == SECONDBUF))
            || (self.start_pos != self.end_pos
                && (self.start_pos.get_value() < SECONDBUF
                    && self.current_buf_end.get_value() == FIRSTBUF))
        {
            eprintln!("WORDS TOO LONG");
            unimplemented!(); // TO simplfy problem
        };

        // position control
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
    pub fn pass(&mut self) {
        assert!(self.start_pos!=self.end_pos);
        self.start_pos.set_value(self.end_pos.get_value());
        self.start_pos.sub_1();
    }
    pub fn get_word(&mut self) -> Vec<u8> {
        assert!(self.start_pos!=self.end_pos);
        let mut word: Vec<u8> = Vec::new();
        // ugly use of match
        match (self.start_pos.get_value(), self.end_pos.get_value()) {
            (mut sp, ep) if sp != ep => {
                while sp != ep {
                    word.push(self.buf[sp]);
                    sp = if sp != (FULLBUFSIZE - 1) { sp + 1 } else { 0 };
                }
            }
            _ => panic!("startpos == endpos"),
        }
        // back 1 byte 
        self.start_pos.set_value(self.end_pos.get_value());
        self.start_pos.sub_1();
        word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_word() {
        //let file_name = env::args().skip(1).next().unwrap();
        let file_name = String::from("operator");
        let mut test = TakenReader::new(&file_name).unwrap();
        let mut count: u32 = 0;
        loop {
            match test.read_byte() {
                Ok(u) => {
                    if u == b' ' {
                        let mut tmp = test.get_word();
                        tmp.pop();
                        println!("{:?}", String::from_utf8(tmp).unwrap().replace("/n", ""));
                        count += 1;
                    }
                }
                Err(_re) => break,
            }
        }
        println!("count : {}", count);
        assert_ne!(0, count);
    }
}
