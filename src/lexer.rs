use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Ident(String),
    Bind,     // :=
    Plus,
    LParen,
    RParen,
    Comma,
    Newline,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let bind = Regex::new(r":=").unwrap();
    let plus = Regex::new(r"\+").unwrap();
    let number = Regex::new(r"[0-9]+").unwrap();
    let ident = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").unwrap();

    for line in input.lines() {
        let mut s = line.trim().to_string();

        while !s.is_empty() {
            if let Some(m) = bind.find(&s) {
                tokens.push(Token::Bind);
                s = s[m.end()..].trim().to_string();
            } else if let Some(m) = plus.find(&s) {
                tokens.push(Token::Plus);
                s = s[m.end()..].trim().to_string();
            } else if s.starts_with("(") {
                tokens.push(Token::LParen);
                s = s[1..].trim().to_string();
            } else if s.starts_with(")") {
                tokens.push(Token::RParen);
                s = s[1..].trim().to_string();
            } else if s.starts_with(",") {
                tokens.push(Token::Comma);
                s = s[1..].trim().to_string();
            } else if let Some(m) = number.find(&s) {
                tokens.push(Token::Number(m.as_str().parse().unwrap()));
                s = s[m.end()..].trim().to_string();
            } else if let Some(m) = ident.find(&s) {
                tokens.push(Token::Ident(m.as_str().into()));
                s = s[m.end()..].trim().to_string();
            } else {
                break;
            }
        }

        tokens.push(Token::Newline);
    }

    tokens
}
