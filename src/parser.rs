use crate::lexer::Token;

#[derive(Debug)]
pub enum Node {
    Bind(String, Box<Node>),
    Add(Box<Node>, Box<Node>),
    Number(i64),
    Ident(String),
    Print(Box<Node>),
    Noop,
}

pub fn parse(tokens: Vec<Token>) -> Vec<Node> {
    let mut nodes = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Ident(name) => {
                // print(x)
                if name == "print" {
                    if let Token::LParen = tokens[i + 1] {
                        let expr = match &tokens[i + 2] {
                            Token::Number(n) => Node::Number(*n),
                            Token::Ident(id) => Node::Ident(id.clone()),
                            _ => Node::Noop,
                        };
                        nodes.push(Node::Print(Box::new(expr)));
                        i += 4;
                        continue;
                    }
                }

                // x := 10
                if let Token::Bind = tokens[i + 1] {
                    let expr = match &tokens[i + 2] {
                        Token::Number(n) => Node::Number(*n),
                        Token::Ident(id) => Node::Ident(id.clone()),
                        _ => Node::Noop,
                    };
                    nodes.push(Node::Bind(name.clone(), Box::new(expr)));
                    i += 3;
                } else {
                    i += 1;
                }
            }

            _ => i += 1,
        }
    }

    nodes
}
