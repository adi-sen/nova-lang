use crate::types::Type;
use crate::lexer::Token;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Number(i64),
    Identifier(String),
    Let {
        name: String,
        type_annotation: Option<String>,
        value: Box<AstNode>,
    },
    Function {
        name: String,
        params: Vec<(String, String)>,
        body: Box<AstNode>,
    },
    Return(Box<AstNode>),
    BinaryOp {
        op: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    StringLiteral(String),
    Boolean(bool),
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<AstNode, String> {
        let mut program = vec![];
        while self.current < self.tokens.len() {
            program.push(self.parse_declaration()?);
        }
        Ok(AstNode::Program(program))
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.current_token() {
            Token::TypeInt => {
                self.advance();
                Ok(Type::Int)
            },
            Token::TypeFloat => {
                self.advance();
                Ok(Type::Float)
            },
            // TODO; add more types
            _ => Err("Expected type".to_string()),
        }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn parse_declaration(&mut self) -> Result<AstNode, String> {
        match self.current_token() {
            Token::Function => self.parse_function(),
            Token::Let => self.parse_let_statement(),
            _ => Err("Expected declaration".to_string()),
        }
    }

    fn parse_function(&mut self) -> Result<AstNode, String> {
        self.advance(); // consume 'fn'
        
        let name = match self.current_token() {
            Token::Identifier(id) => {
                let name = id.clone();
                self.advance();
                name
            },
            _ => return Err("Expected function name".to_string()),
        };

        if !matches!(self.current_token(), Token::LeftParen) {
            return Err("Expected '(' after function name".to_string());
        }
        self.advance();

        let mut params = Vec::new();
        while !matches!(self.current_token(), Token::RightParen) {
            match self.current_token() {
                Token::Identifier(param) => {
                    let param_name = param.clone();
                    self.advance();
                    
                    if !matches!(self.current_token(), Token::Colon) {
                        return Err("Expected ':' after parameter name".to_string());
                    }
                    self.advance();

                    let param_type = match self.current_token() {
                        Token::Identifier(type_name) => {
                            let type_name = type_name.clone();
                            self.advance();
                            type_name
                        },
                        _ => return Err("Expected type name after ':'".to_string()),
                    };

                    params.push((param_name, param_type));

                    if matches!(self.current_token(), Token::Comma) {
                        self.advance();
                    }
                },
                _ => return Err("Expected parameter name".to_string()),
            }
        }
        self.advance(); // consume ')'

        if !matches!(self.current_token(), Token::Colon) {
            return Err("Expected ':' after parameters".to_string());
        }
        self.advance();

        let _return_type = self.parse_type()?;

        if !matches!(self.current_token(), Token::LeftBrace) {
            return Err("Expected '{' to begin function body".to_string());
        }
        self.advance();

        let body = self.parse_block()?;

        Ok(AstNode::Function {
            name,
            params,
            body: Box::new(body),
        })
    }

    fn parse_block(&mut self) -> Result<AstNode, String> {
        let mut statements = Vec::new();
        
        while !matches!(self.current_token(), Token::RightBrace) {
            if matches!(self.current_token(), Token::Return) {
                self.advance();
                let expr = self.parse_expression()?;
                if !matches!(self.current_token(), Token::Semicolon) {
                    return Err("Expected ';' after return statement".to_string());
                }
                self.advance();
                statements.push(AstNode::Return(Box::new(expr)));
            } else {
                return Err("Unexpected token in function body".to_string());
            }
        }
        self.advance(); // consume '}'
        
        Ok(AstNode::Program(statements))
    }

    fn parse_expression(&mut self) -> Result<AstNode, String> {
        self.parse_binary_expression()
    }

    fn parse_binary_expression(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_primary()?;

        while let Token::Plus | Token::Minus | Token::Multiply | Token::Divide = self.current_token() {
            let op = match self.current_token() {
                Token::Plus => BinaryOperator::Add,
                Token::Minus => BinaryOperator::Subtract,
                Token::Multiply => BinaryOperator::Multiply,
                Token::Divide => BinaryOperator::Divide,
                _ => unreachable!(),
            };
            self.advance();

            let right = self.parse_primary()?;
            left = AstNode::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<AstNode, String> {
        match self.current_token() {
            Token::Number(n) => {
                let num = *n;
                self.advance();
                Ok(AstNode::Number(num))
            },
            Token::StringLiteral => {
                let value = self.current_token().to_string();
                self.advance();
                Ok(AstNode::StringLiteral(value))
            },
            Token::True => {
                self.advance();
                Ok(AstNode::Boolean(true))
            },
            Token::False => {
                self.advance();
                Ok(AstNode::Boolean(false))
            },
            Token::Identifier(name) => {
                let id = name.clone();
                self.advance();
                Ok(AstNode::Identifier(id))
            },
            _ => Err("Expected expression".to_string()),
        }
    }

    fn parse_let_statement(&mut self) -> Result<AstNode, String> {
        self.advance(); // consume 'let'
        
        let name = match self.current_token() {
            Token::Identifier(id) => {
                let name = id.clone();
                self.advance();
                name
            },
            _ => return Err("Expected variable name".to_string()),
        };

        let type_annotation = if matches!(self.current_token(), Token::Colon) {
            self.advance();
            match self.current_token() {
                Token::Identifier(type_name) => {
                    let type_name = Some(type_name.clone());
                    self.advance();
                    type_name
                },
                _ => return Err("Expected type name after ':'".to_string()),
            }
        } else {
            None
        };

        if !matches!(self.current_token(), Token::Equals) {
            return Err("Expected '=' after type annotation".to_string());
        }
        self.advance();

        let value = self.parse_expression()?;

        if !matches!(self.current_token(), Token::Semicolon) {
            return Err("Expected ';' after let statement".to_string());
        }
        self.advance();

        Ok(AstNode::Let {
            name,
            type_annotation,
            value: Box::new(value),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    #[test]
    fn test_parse_function() {
        let tokens = vec![
            Token::Function,
            Token::Identifier("main".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::Colon,
            Token::TypeInt,
            Token::LeftBrace,
            Token::Return,
            Token::Number(42),
            Token::Semicolon,
            Token::RightBrace,
        ];
        
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        
        match ast {
            AstNode::Program(nodes) => {
                assert_eq!(nodes.len(), 1);
                match &nodes[0] {
                    AstNode::Function { name, params, body } => {
                        assert_eq!(name, "main");
                        assert!(params.is_empty());
                        match &**body {
                            AstNode::Program(statements) => {
                                assert_eq!(statements.len(), 1);
                                match &statements[0] {
                                    AstNode::Return(expr) => {
                                        match &**expr {
                                            AstNode::Number(n) => assert_eq!(*n, 42),
                                            _ => panic!("Expected number in return statement"),
                                        }
                                    },
                                    _ => panic!("Expected return statement"),
                                }
                            },
                            _ => panic!("Expected program node for function body"),
                        }
                    },
                    _ => panic!("Expected function node"),
                }
            },
            _ => panic!("Expected program node"),
        }
    }
}
