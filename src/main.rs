mod buf_reader;
use buf_reader::*;
use std::env;
//  需要识别 标识符(id) 运算符(op) 关键字(kw) 无符号有符号数字(num)
//  pps: 问题可能并不能从一开始就看清楚 所以可以一边走一边看

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
mod lexer;
use lexer::*;
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

fn main() {
    let file_name = env::args().skip(1).next().expect(
        "{
            Usage: lexer [FileName]
        ",
    );
    let mut lexer: Lexer = Lexer::new(&file_name);
    let mut result: Vec<SymbolTable> = Vec::new();
    loop {
        let tmp = lexer.analyze();
        println!("{:#?}", tmp);
        match tmp {
            SymbolTable::Keyword(i) => result.push(SymbolTable::Keyword(i)),
            SymbolTable::Operator(i) => result.push(SymbolTable::Operator(i)),
            SymbolTable::Num(i) => result.push(SymbolTable::Num(i)),
            SymbolTable::Id(i) => result.push(SymbolTable::Id(i)),
            SymbolTable::EOF => break,
        }
    }
    println!("{:#?}", result);

    // println!("ids:\n{:?}", ids);
    // println!("nums:\n{:?}", nums);
    // println!("keywords:\n{:?}", keywords);
    // println!("ops:\n{:?}", operators);
}

/*
    let mut taken_reader = TakenReader::new(&file_name);
    let mut kw: Vec<String> = Vec::new();
    load_word(&mut kw, ",", "keyword");

    let mut keywords: Vec<&str> = Vec::new();
    let mut operators: Vec<String> = Vec::new();
    let mut nums: Vec<String> = Vec::new();
    let mut ids: Vec<String> = Vec::new();

    let mut ch: u8;
    ch = taken_reader.read_byte().unwrap();
    println!("{}", ch as char);

    'out: loop {
        match ch {
            // try to find id or kw
            b'a'...b'z' | b'A'...b'Z' | b'_' => 'kw_op_id: loop {
                match taken_reader.read_byte() {
                    Ok(a) => {
                        ch = a;
                    }
                    Err(_i) => break 'out,
                }
                match ch {
                    b'a'...b'z' | b'A'...b'Z' | b'0'...b'9' | b'_' => (),
                    _ => {
                        let mut kw_or_id = String::from_utf8(taken_reader.get_word()).unwrap();
                        kw_or_id.pop(); //last ch does not match
                        for i in kw.iter() {
                            if *i == kw_or_id {
                                keywords.push(i);
                                break 'kw_op_id;
                            }
                        }
                        if kw_or_id == "sizeof" {
                            operators.push(String::from("sizeof"));
                            break;
                        }
                        ids.push(kw_or_id);
                        break;
                    }
                }
            },
            // try to find num
            b'0'...b'9' => {
                let mut has_dot = false;
                loop {
                    match taken_reader.read_byte() {
                        Ok(a) => {
                            ch = a;
                        }
                        Err(_i) => break 'out,
                    }
                    match ch {
                        b'0'...b'9' => {}
                        b'.' => {
                            if has_dot {
                                let mut num = String::from_utf8(taken_reader.get_word()).unwrap();
                                num.pop();
                                nums.push(num);
                                break;
                            } else {
                                has_dot = true;
                            }
                        }
                        _ => {
                            let mut num = String::from_utf8(taken_reader.get_word()).unwrap();
                            num.pop();
                            nums.push(num);
                            break;
                        }
                    }
                }
            }
            // try to find op
            x => {
                match x {
                    // can be find by once
                    b'(' | b')' | b'[' | b']' | b'{' | b'}' | b'.' | b'?' | b':' | b',' => {
                        match taken_reader.read_byte() {
                            Ok(a) => {
                                ch = a;
                            }
                            Err(_i) => break 'out,
                        }
                        let mut ops = String::from_utf8(taken_reader.get_word()).unwrap();
                        ops.pop();
                        operators.push(ops);
                    }
                    b'*' | b'/' | b'%' | b'^' | b'!' | b'=' => {
                        match taken_reader.read_byte() {
                            Ok(a) => {
                                ch = a;
                            }
                            Err(_i) => break 'out,
                        }
                        if ch == b'=' {
                            match taken_reader.read_byte() {
                                Ok(a) => {
                                    ch = a;
                                }
                                Err(_i) => break 'out,
                            }
                        }
                        let mut ops = String::from_utf8(taken_reader.get_word()).unwrap();
                        ops.pop();
                        operators.push(ops);
                    }
                    x if x == b'<' || x == b'>' => {
                        match taken_reader.read_byte() {
                            Ok(a) => {
                                ch = a;
                            }
                            Err(_i) => break 'out,
                        }
                        if ch == b'=' {
                            match taken_reader.read_byte() {
                                Ok(a) => {
                                    ch = a;
                                }
                                Err(_i) => break 'out,
                            }
                        } else if (ch == b'<' && x == b'<') || (ch == b'>' && x == b'>') {
                            match taken_reader.read_byte() {
                                Ok(a) => {
                                    ch = a;
                                }
                                Err(_i) => break 'out,
                            }
                        }
                        let mut ops = String::from_utf8(taken_reader.get_word()).unwrap();
                        ops.pop();
                        operators.push(ops);
                    }
                    x if x == b'+' || x == b'-' => {
                        match taken_reader.read_byte() {
                            Ok(a) => {
                                ch = a;
                            }
                            Err(_i) => break 'out,
                        }
                        if ch == b'=' {
                            match taken_reader.read_byte() {
                                Ok(a) => {
                                    ch = a;
                                }
                                Err(_i) => break 'out,
                            }
                        } else if (ch == b'+' && x == b'+') || (ch == b'-' && x == b'-') {
                            match taken_reader.read_byte() {
                                Ok(a) => {
                                    ch = a;
                                }
                                Err(_i) => break 'out,
                            }
                        }
                        let mut ops = String::from_utf8(taken_reader.get_word()).unwrap();
                        ops.pop();
                        operators.push(ops);
                    }
                    _ => {
                        match taken_reader.read_byte() {
                            Ok(a) => {
                                ch = a;
                            }
                            Err(_i) => break 'out,
                        }
                        taken_reader.get_word();
                    }
                }
            }
        }
    }*/
