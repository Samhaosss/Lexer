use super::buf_reader::*;

// reture type used by lexer
#[derive(Debug)]
pub enum SymbolTable {
    Keyword(String),
    Operator(String),
    Id(String),
    Num(String),
    EOF,
}
#[derive(Debug)]
pub struct Lexer {
    taken_reader: TakenReader,
    keywords: Vec<String>,
    last_ch: u8,
    finish: bool,
}
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

impl Lexer {
    fn load_word(v: &mut Vec<String>, splitor: &str, file_name: &str) {
        let mut reader = BufReader::new(File::open(file_name).unwrap());
        let mut kws = String::new();
        reader.read_to_string(&mut kws).unwrap();
        for i in kws.split(splitor) {
            let mut tmp = String::from(i).replace("\t", "").replace("\n", "");
            if tmp != "" {
                v.push(tmp);
            }
        }
    }

    pub fn new(file_name: &str) -> Lexer {
        let mut tmp: Vec<String> = Vec::new();
        Lexer::load_word(&mut tmp, ",", "keyword");
        let mut lex = Lexer {
            taken_reader: TakenReader::new(file_name),
            keywords: tmp,
            last_ch: 0,
            finish: false,
        };
        lex.last_ch = lex.taken_reader.read_byte().expect("empty file");
        lex
    }
    fn read_next(&mut self) {
        // just read next, ignore err
        match self.taken_reader.read_byte() {
            Ok(a) => {
                self.last_ch = a;
            }
            Err(_i) => {
                self.finish = true;
                println!("FINISH")
            }
        };
    }

    pub fn analyze(&mut self) -> SymbolTable {
        //let mut ch = self.taken_reader.read_byte().unwrap();

        loop {
            if !self.finish {
                match self.last_ch {
                    // try to find id or kw
                    b'a'...b'z' | b'A'...b'Z' | b'_' => 'kw_op_id: loop {
                        match self.taken_reader.read_byte() {
                            Ok(a) => {
                                self.last_ch = a;
                            }
                            Err(_i) => {
                                let mut kw_or_id =
                                    String::from_utf8(self.taken_reader.get_word()).unwrap();
                                kw_or_id.pop();
                                self.finish = true;
                                return SymbolTable::Id(kw_or_id);
                            }
                        }
                        match self.last_ch {
                            b'a'...b'z' | b'A'...b'Z' | b'0'...b'9' | b'_' => (),
                            _ => {
                                let mut kw_or_id =
                                    String::from_utf8(self.taken_reader.get_word()).unwrap();
                                kw_or_id.pop(); //last ch does not match
                                for i in self.keywords.iter() {
                                    if *i == kw_or_id {
                                        return SymbolTable::Keyword(i.clone());
                                    }
                                }
                                if kw_or_id == "sizeof" {
                                    return SymbolTable::Operator(String::from("sizeof"));
                                }
                                return SymbolTable::Id(kw_or_id);
                            }
                        }
                    },
                    // try to find num
                    b'0'...b'9' => {
                        let mut has_dot = false;
                        loop {
                            match self.taken_reader.read_byte() {
                                Ok(a) => {
                                    self.last_ch = a;
                                }
                                Err(_i) => {
                                    let mut num =
                                        String::from_utf8(self.taken_reader.get_word()).unwrap();
                                    num.pop();
                                    self.finish = true;
                                    return SymbolTable::Num(num);
                                }
                            }
                            match self.last_ch {
                                b'0'...b'9' => {}
                                b'.' => {
                                    if has_dot {
                                        let mut num =
                                            String::from_utf8(self.taken_reader.get_word())
                                                .unwrap();
                                        num.pop();
                                        return SymbolTable::Num(num);
                                    } else {
                                        has_dot = true;
                                    }
                                }
                                _ => {
                                    let mut num =
                                        String::from_utf8(self.taken_reader.get_word()).unwrap();
                                    num.pop();
                                    return SymbolTable::Num(num);
                                }
                            }
                        }
                    }
                    // try to find op
                    x => {
                        match x {
                            // can be find by once
                            b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'.' | b'?' | b':' | b',' => {
                                self.read_next();
                                let mut ops =
                                    String::from_utf8(self.taken_reader.get_word()).unwrap();
                                ops.pop();
                                return SymbolTable::Operator(ops);
                            }
                            b'*' | b'/' | b'%' | b'^' | b'!' | b'=' => {
                                match self.taken_reader.read_byte() {
                                    Ok(a) => {
                                        self.last_ch = a;
                                    }
                                    Err(_i) => {
                                        let mut ops =
                                            String::from_utf8(self.taken_reader.get_word())
                                                .unwrap();
                                        ops.pop();
                                        self.finish = true;
                                        return SymbolTable::Operator(ops);
                                    }
                                }
                                if self.last_ch == b'=' {
                                    match self.taken_reader.read_byte() {
                                        Ok(a) => {
                                            self.last_ch = a;
                                        }
                                        Err(_i) => {
                                            let mut ops =
                                                String::from_utf8(self.taken_reader.get_word())
                                                    .unwrap();
                                            ops.pop();
                                            self.finish = true;
                                            return SymbolTable::Operator(ops);
                                        }
                                    }
                                }
                                let mut ops =
                                    String::from_utf8(self.taken_reader.get_word()).unwrap();
                                ops.pop();
                                return SymbolTable::Operator(ops);
                            }
                            x if x == b'<' || x == b'>' => {
                                match self.taken_reader.read_byte() {
                                    Ok(a) => {
                                        self.last_ch = a;
                                    }
                                    Err(_i) => {
                                        let mut ops =
                                            String::from_utf8(self.taken_reader.get_word())
                                                .unwrap();
                                        ops.pop();
                                        self.finish = true;
                                        return SymbolTable::Operator(ops);
                                    }
                                }
                                if self.last_ch == b'=' {
                                    match self.taken_reader.read_byte() {
                                        Ok(a) => {
                                            self.last_ch = a;
                                        }
                                        Err(_i) => {
                                            let mut ops =
                                                String::from_utf8(self.taken_reader.get_word())
                                                    .unwrap();
                                            ops.pop();
                                            self.finish = true;
                                            return SymbolTable::Operator(ops);
                                        }
                                    }
                                } else if (self.last_ch == b'<' && x == b'<')
                                    || (self.last_ch == b'>' && x == b'>')
                                {
                                    match self.taken_reader.read_byte() {
                                        Ok(a) => {
                                            self.last_ch = a;
                                        }
                                        Err(_i) => {
                                            let mut ops =
                                                String::from_utf8(self.taken_reader.get_word())
                                                    .unwrap();
                                            ops.pop();
                                            self.finish = true;
                                            return SymbolTable::Operator(ops);
                                        }
                                    }
                                }
                                let mut ops =
                                    String::from_utf8(self.taken_reader.get_word()).unwrap();
                                ops.pop();
                                return SymbolTable::Operator(ops);
                            }
                            x if x == b'+' || x == b'-' => {
                                match self.taken_reader.read_byte() {
                                    Ok(a) => {
                                        self.last_ch = a;
                                    }
                                    Err(_i) => {
                                        let mut ops =
                                            String::from_utf8(self.taken_reader.get_word())
                                                .unwrap();
                                        ops.pop();
                                        self.finish = true;
                                        return SymbolTable::Operator(ops);
                                    }
                                }
                                if self.last_ch == b'=' {
                                    match self.taken_reader.read_byte() {
                                        Ok(a) => {
                                            self.last_ch = a;
                                        }
                                        Err(_i) => {
                                            let mut ops =
                                                String::from_utf8(self.taken_reader.get_word())
                                                    .unwrap();
                                            ops.pop();
                                            self.finish = true;
                                            return SymbolTable::Operator(ops);
                                        }
                                    }
                                } else if (self.last_ch == b'+' && x == b'+')
                                    || (self.last_ch == b'-' && x == b'-')
                                {
                                    match self.taken_reader.read_byte() {
                                        Ok(a) => {
                                            self.last_ch = a;
                                        }
                                        Err(_i) => {
                                            let mut ops =
                                                String::from_utf8(self.taken_reader.get_word())
                                                    .unwrap();
                                            ops.pop();
                                            self.finish = true;
                                            return SymbolTable::Operator(ops);
                                        }
                                    }
                                }
                                let mut ops =
                                    String::from_utf8(self.taken_reader.get_word()).unwrap();
                                ops.pop();
                                return SymbolTable::Operator(ops);
                            }
                            _ => {
                                self.read_next();
                                self.taken_reader.get_word();
                            }
                        }
                    }
                };
            } else {
                break;
            }
        }

        SymbolTable::EOF
    }
}
