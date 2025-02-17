#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
}

#[allow(dead_code)]
impl Type {
    pub fn function(params: Vec<Type>, return_type: Type) -> Self {
        Type::Function {
            params,
            return_type: Box::new(return_type),
        }
    }

    pub fn void() -> Self {
        Type::Void
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TypeEnvironment {
    symbols: std::collections::HashMap<String, Type>,
}

#[allow(dead_code)]
impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            symbols: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, type_: Type) {
        self.symbols.insert(name, type_);
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.symbols.get(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_environment() {
        let mut env = TypeEnvironment::new();
        env.insert("x".to_string(), Type::Int);
        assert_eq!(env.get("x"), Some(&Type::Int));
    }

    #[test]
    fn test_function_type() {
        let fn_type = Type::function(vec![Type::Int, Type::Bool], Type::void());
        match fn_type {
            Type::Function { params, return_type } => {
                assert_eq!(params, vec![Type::Int, Type::Bool]);
                assert_eq!(*return_type, Type::Void);
            },
            _ => panic!("Expected function type"),
        }
    }
}
