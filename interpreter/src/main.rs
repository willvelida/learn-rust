use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Let,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

#[derive(Debug, Clone)]
enum Expr {
    Number(f64),
    Variable(String),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Let {
        name: String,
        value: Box<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }

    // expr := "let" ident "=" expr | term (("+" | "-") term)*
    fn parse_expr(&mut self) -> Result<Expr, String> {
        if self.peek() == Some(&Token::Let) {
            return self.parse_let();
        }
        let mut node = self.parse_term()?;
        while let Some(op) = self.match_add_sub() {
            let right = self.parse_term()?;
            node = Expr::BinaryOp { left: Box::new(node), op, right: Box::new(right) };
        }
        Ok(node)
    }

    fn parse_let(&mut self) -> Result<Expr, String> {
        self.advance(); // consume `let`
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            other => return Err(format!("expected a name after let, found {other:?}")),
        };
        match self.advance() {
            Some(Token::Equals) => {}
            other => return Err(format!("expected = after the name, found {other:?}")),
        }
        let value = self.parse_expr()?;
        Ok(Expr::Let { name, value: Box::new(value) })
    }

    // term := factor (("*" | "/") factor)*
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut node = self.parse_factor()?;
        while let Some(op) = self.match_mul_div() {
            let right = self.parse_factor()?;
            node = Expr::BinaryOp { left: Box::new(node), op, right: Box::new(right) };
        }
        Ok(node)
    }

    // factor := number | identifier | "(" expr ")"
    fn parse_factor(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(Expr::Number(n)),
            Some(Token::Identifier(name)) => Ok(Expr::Variable(name)),
            Some(Token::LParen) => {
                let inner = self.parse_expr()?;
                match self.advance() {
                    Some(Token::RParen) => Ok(inner),
                    other => Err(format!("expected a closing paren, found {other:?}")),
                }
            }
            other => Err(format!("unexpected token: {other:?}")),
        }
    }

    fn match_add_sub(&mut self) -> Option<BinOp> {
        match self.peek() {
            Some(Token::Plus) => { self.advance(); Some(BinOp::Add) }
            Some(Token::Minus) => { self.advance(); Some(BinOp::Sub) }
            _ => None,
        }
    }

    fn match_mul_div(&mut self) -> Option<BinOp> {
        match self.peek() {
            Some(Token::Star) => { self.advance(); Some(BinOp::Mul) }
            Some(Token::Slash) => { self.advance(); Some(BinOp::Div) }
            _ => None,
        }
    }
}

#[derive(Debug)]
enum RuntimeError {
    UndefinedVariable(String),
    DivisionByZero,
}

fn main() {
    let mut env: HashMap<String, f64> = HashMap::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).unwrap() == 0 {
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        match run(line, &mut env) {
            Ok(value) => println!("{value}"),
            Err(message) => eprintln!("error: {message}"),
        }
    }
}

fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => { chars.next(); }
            '+' => { chars.next(); tokens.push(Token::Plus); }
            '-' => { chars.next(); tokens.push(Token::Minus); }
            '*' => { chars.next(); tokens.push(Token::Star); }
            '/' => { chars.next(); tokens.push(Token::Slash); }
            '(' => { chars.next(); tokens.push(Token::LParen); }
            ')' => { chars.next(); tokens.push(Token::RParen); }
            '=' => { chars.next(); tokens.push(Token::Equals); }
            c if c.is_ascii_digit() => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() || d == '.' {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let value: f64 = num.parse().map_err(|_| format!("bad number: {num}"))?;
                tokens.push(Token::Number(value));
            }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&l) = chars.peek() {
                    if l.is_alphanumeric() || l == '_' {
                        ident.push(l);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if ident == "let" {
                    tokens.push(Token::Let);
                } else {
                    tokens.push(Token::Identifier(ident));
                }
            }
            other => return Err(format!("unexpected character: {other}")),
        }
    }
    Ok(tokens)
}

fn eval(expr: &Expr, env: &mut HashMap<String, f64>) -> Result<f64, RuntimeError> {
    match expr {
        Expr::Number(n) => Ok(*n),

        Expr::Variable(name) => env
            .get(name)
            .copied()
            .ok_or_else(|| RuntimeError::UndefinedVariable(name.clone())),

        Expr::Let { name, value } => {
            let v = eval(value, env)?;
            env.insert(name.clone(), v);
            Ok(v)
        }

        Expr::BinaryOp { left, op, right } => {
            let l = eval(left, env)?;
            let r = eval(right, env)?;
            match op {
                BinOp::Add => Ok(l + r),
                BinOp::Sub => Ok(l - r),
                BinOp::Mul => Ok(l * r),
                BinOp::Div => {
                    if r == 0.0 {
                        Err(RuntimeError::DivisionByZero)
                    } else {
                        Ok(l / r)
                    }
                }
            }
        }
    }
}

fn run(line: &str, env: &mut HashMap<String, f64>) -> Result<f64, String> {
    let tokens = lex(line)?;
    let mut parser = Parser { tokens, pos: 0};
    let ast = parser.parse_expr()?;
    eval(&ast, env).map_err(|e| format!("{e:?}"))
}
