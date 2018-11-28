use super::{buf_reader::*, error_handler::ErrType, error_handler::ErrorHandler};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub enum SymbolTable {
    Keyword(String),
    Operator(String),
    Id(String),
    Num(String),
    Other(String),
    EOF,
}
#[derive(Debug)]
pub struct Lexer {
    taken_reader: TakenReader,
    keywords: Vec<String>,
    last_ch: u8,
    finish: bool,
    line_no: u32,
}

impl Lexer {
    // read keywords list from file
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
    // create new lexer instance
    pub fn new(file_name: &str) -> Lexer {
        let mut tmp: Vec<String> = Vec::new();
        Lexer::load_word(&mut tmp, ",", "keyword");
        let mut lex = Lexer {
            taken_reader: TakenReader::new(file_name).unwrap(),
            keywords: tmp,
            last_ch: 0,
            finish: false,
            line_no: 1,
        };
        lex.last_ch = lex.taken_reader.read_byte().expect("empty file");
        lex
    }
    // read one byte, set finish state to ture if fail to read
    fn read_next(&mut self) {
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
    // get word from taken reader, check if word if kw or sizeof or id
    fn get_id_or_keyword(&mut self) -> SymbolTable {
        let mut kw_or_id = String::from_utf8(self.taken_reader.get_word()).unwrap();
        if !self.finish {
            kw_or_id.pop();
        };
        if self.keywords.iter().any(|v| v == &kw_or_id) {
            SymbolTable::Keyword(kw_or_id)
        } else if kw_or_id == "sizeof" {
            SymbolTable::Operator(String::from("sizeof"))
        } else {
            SymbolTable::Id(kw_or_id)
        }
    }
    // dfa to analyze id,kw
    fn analyze_words(&mut self) -> SymbolTable {
        loop {
            match self.taken_reader.read_byte() {
                Ok(a) => {
                    self.last_ch = a;
                }
                Err(_i) => {
                    self.finish = true;
                    return self.get_id_or_keyword();
                }
            };
            match self.last_ch {
                b'a'...b'z' | b'A'...b'Z' | b'0'...b'9' | b'_' => (),
                _ => {
                    return self.get_id_or_keyword();
                }
            };
        }
    }
    fn get_num(&mut self) -> SymbolTable {
        let mut num = String::from_utf8(self.taken_reader.get_word()).unwrap();
        if !self.finish {
            num.pop();
        };
        SymbolTable::Num(num)
    }
    // dfs to analyze num
    fn analyze_nums(&mut self) -> SymbolTable {
        let mut has_dot = false;
        loop {
            match self.taken_reader.read_byte() {
                Ok(a) => {
                    self.last_ch = a;
                }
                Err(_i) => {
                    self.finish = true;
                    return self.get_num();
                }
            }
            match self.last_ch {
                b'0'...b'9' => {}
                b'.' => {
                    if has_dot {
                        return self.get_num();
                    } else {
                        has_dot = true;
                    }
                }
                _ => {
                    return self.get_num();
                }
            }
        }
    }
    fn get_op(&mut self) -> SymbolTable {
        let mut ops = String::from_utf8(self.taken_reader.get_word()).unwrap();
        if !self.finish {
            ops.pop();
        };
        return SymbolTable::Operator(ops);
    }
    // dfs for <= < << > >> >=
    fn analyze_por(&mut self) -> SymbolTable {
        let x = self.last_ch;
        match self.taken_reader.read_byte() {
            Ok(a) => {
                self.last_ch = a;
            }
            Err(_i) => {
                self.finish = true;
                return self.get_op();
            }
        }
        if self.last_ch == b'='
            || (self.last_ch == b'<' && x == b'<')
            || (self.last_ch == b'>' && x == b'>')
        {
            match self.taken_reader.read_byte() {
                Ok(a) => {
                    self.last_ch = a;
                }
                Err(_i) => {
                    self.finish = true;
                    return self.get_op();
                }
            }
        }
        return self.get_op();
    }
    // *= /= * / ...
    fn analyze_sample_as(&mut self) -> SymbolTable {
        match self.taken_reader.read_byte() {
            Ok(a) => {
                self.last_ch = a;
            }
            Err(_i) => {
                self.finish = true;
                return self.get_op();
            }
        }
        if self.last_ch == b'=' {
            match self.taken_reader.read_byte() {
                Ok(a) => {
                    self.last_ch = a;
                }
                Err(_i) => {
                    self.finish = true;
                    return self.get_op();
                }
            }
        }
        return self.get_op();
    }
    // ++ -- + -
    fn analyze_aass(&mut self) -> SymbolTable {
        let x = self.last_ch;
        match self.taken_reader.read_byte() {
            Ok(a) => {
                self.last_ch = a;
            }
            Err(_i) => {
                self.finish = true;
                return self.get_op();
            }
        }
        if self.last_ch == b'='
            || (self.last_ch == b'+' && x == b'+')
            || (self.last_ch == b'-' && x == b'-')
        {
            match self.taken_reader.read_byte() {
                Ok(a) => {
                    self.last_ch = a;
                }
                Err(_i) => {
                    self.finish = true;
                    return self.get_op();
                }
            }
        }
        return self.get_op();
    }
    pub fn analyze(&mut self) -> SymbolTable {
        loop {
            if !self.finish {
                match self.last_ch {
                    // id or kw
                    b'a'...b'z' | b'A'...b'Z' | b'_' => return self.analyze_words(),
                    // num -> num[.(num)*]
                    b'0'...b'9' => return self.analyze_nums(),
                    // operator
                    b'(' | b')' | b'"' | b'\'' | b'[' | b']' | b'{' | b'}' | b'.' | b'?' | b':'
                    | b',' => {
                        self.read_next();
                        return self.get_op();
                    }
                    b'*' | b'/' | b'%' | b'^' | b'!' | b'=' => return self.analyze_sample_as(),
                    b'<' | b'>' => return self.analyze_por(),
                    b'+' | b'-' => return self.analyze_aass(),
                    // seperator
                    b';' => {
                        self.read_next();
                        return SymbolTable::Other(";".to_string());
                    }
                    // just pass
                    b'\r' | b'\t' | b' ' => {
                        self.read_next();
                        self.taken_reader.pass();
                    }
                    b'\n' => {
                        self.read_next();
                        self.taken_reader.pass();
                        self.line_no += 1;
                    }
                    // illegal ch
                    _ => {
                        let msg = format!("ch[{}] at line:{} ", self.last_ch as char, self.line_no);
                        ErrorHandler::handle(ErrType::IllegalCh, &msg);
                        // eprintln!(
                        //     "ERROR:Illegal ch[{}] at line:{} ",
                        //     self.last_ch as char, self.line_no
                        // );
                        self.read_next();
                        self.taken_reader.pass();
                    }
                }
            } else {
                break;
            }
        }
        SymbolTable::EOF
    }

    pub fn get_line_no(&self) -> u32 {
        self.line_no
    }
}
