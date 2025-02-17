use crate::parser::AstNode;
use crate::types::{Type, TypeEnvironment};

pub struct TypeChecker {
    env: TypeEnvironment,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            env: TypeEnvironment::new(),
        }
    }

    pub fn check(&mut self, node: &AstNode) -> Result<Type, String> {
        match node {
            AstNode::Program(nodes) => {
                let mut last_type = Type::Void;
                for node in nodes {
                    last_type = self.check(node)?;
                }
                Ok(last_type)
            },
            AstNode::Function { name, params: _, body } => {
                let body_type = self.check(body)?;
                self.env.insert(name.clone(), Type::function(vec![], body_type.clone()));
                Ok(body_type)
            },
            AstNode::Number(_) => Ok(Type::Int),
            AstNode::StringLiteral(_) => Ok(Type::String),
            AstNode::Boolean(_) => Ok(Type::Bool),
            AstNode::Let { name, type_annotation, value } => {
                let value_type = self.check(value)?;
                
                // Convert type annotation if present
                if let Some(type_name) = type_annotation {
                    let expected_type = match type_name.as_str() {
                        "int" => Type::Int,
                        "float" => Type::Float,
                        "string" => Type::String,
                        "bool" => Type::Bool,
                        _ => return Err(format!("Unknown type: {}", type_name)),
                    };
                    if value_type != expected_type {
                        return Err(format!("Type mismatch: expected {:?}, got {:?}", expected_type, value_type));
                    }
                }
                
                self.env.insert(name.clone(), value_type.clone());
                Ok(value_type)
            }
            AstNode::Return(expr) => self.check(expr),
            _ => Err("Unsupported node type for type checking".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_type_checker() {
        let mut checker = TypeChecker::new();
        let node = AstNode::Let {
            name: "x".to_string(),
            type_annotation: Some("int".to_string()),
            value: Box::new(AstNode::Number(42)),
        };
        
        assert_eq!(checker.check(&node), Ok(Type::Int));
        
        let node_error = AstNode::Let {
            name: "y".to_string(),
            type_annotation: Some("string".to_string()),
            value: Box::new(AstNode::Number(42)),
        };
        
        assert!(checker.check(&node_error).is_err());
    }
}
