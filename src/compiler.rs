use crate::scanner::{Scanner, TokenType};

pub fn compile(source: &str) {
    let mut scanner = Scanner::new(source);

    let mut line: i32 = -1;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{number:>width$} ", number = token.line, width = 4);
            line = token.line;
        } else {
            print!("    | ");
        }
        print!(
            "{token_number:>width$} '{literal}'\n",
            token_number = token.ttype as i32,
            width = 2,
            literal = scanner
                .source
                .chars()
                .skip(token.start)
                .take(token.length)
                .collect::<String>(),
        );

        if let TokenType::TokenEOF = token.ttype {
            break;
        }
    }
}
