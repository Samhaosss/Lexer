use std::env;
//  需要识别 标识符(id) 运算符(op) 关键字(kw) 无符号有符号数字(num)
//  pps: 问题可能并不能从一开始就看清楚 所以可以一边走一边看
extern crate lexer;
use lexer::lexer::*;

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
            SymbolTable::Other(i) => {
                result.push(SymbolTable::Other(i));
                println!("LINE:{},{:?}", lexer.get_line_no(), result);
                result.clear();
            }
            SymbolTable::EOF => break,
        }
    }
}
