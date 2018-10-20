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
        match lexer.analyze() {
            SymbolTable::Keyword(i) => result.push(SymbolTable::Keyword(i)),
            SymbolTable::Operator(i) => result.push(SymbolTable::Operator(i)),
            SymbolTable::Num(i) => result.push(SymbolTable::Num(i)),
            SymbolTable::Id(i) => result.push(SymbolTable::Id(i)),
            SymbolTable::EOF => break,
        }
    }
    println!("{:#?}", result);
}
