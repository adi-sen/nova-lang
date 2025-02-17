use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("fn")]
    Function,

    #[token("let")]
    Let,

    #[token("return")]
    Return,

    #[token("if")]
    If,

    #[regex("[A-Za-z][A-Za-z0-9_]*", |lex| String::from(lex.slice()))]
    Identifier(String),

    #[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
    Number(i64),

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(";")]
    Semicolon,

    #[token("=")]
    Equals,

    #[token("i32")]
    TypeInt,

    #[token("f64")]
    TypeFloat,

    #[token("bool")]
    TypeBool,

    #[token("string")]
    TypeString,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token(":")]
    Colon,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[regex(r#""[^"]*""#)]
    StringLiteral,

    #[token(",")]
    Comma,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Function => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Return => write!(f, "return"),
            Token::If => write!(f, "if"),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::Number(n) => write!(f, "{}", n),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::Semicolon => write!(f, ";"),
            Token::Equals => write!(f, "="),
            Token::TypeInt => write!(f, "i32"),
            Token::TypeFloat => write!(f, "f64"),
            Token::TypeBool => write!(f, "bool"),
            Token::TypeString => write!(f, "string"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Colon => write!(f, ":"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::StringLiteral => write!(f, "string literal"),
            Token::Comma => write!(f, ","),
            Token::Error => write!(f, "error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lexer() {
        let source = r#"fn main(): i32 {
            let x: i32 = 42;
            let s: string = "hello";
            return x;
        }"#;
        
        let mut lexer = Token::lexer(source);
        assert_eq!(lexer.next(), Some(Token::Function));
        assert_eq!(lexer.next(), Some(Token::Identifier("main".to_string())));
        assert_eq!(lexer.next(), Some(Token::LeftParen));
        assert_eq!(lexer.next(), Some(Token::RightParen));
    }
}
