use crate::lexer::Token;
use crate::parser::Parser;
use crate::codegen::CodeGen;
use inkwell::context::Context;
use logos::Logos;
use std::process::Command;

mod lexer;
mod parser;
mod types;
mod codegen;
mod typecheck;

fn main() -> Result<(), String> {
    let source = r#"
        fn main(): i32 {
            return 42;
        }
    "#;

    // Lexing
    let lexer = Token::lexer(source);
    let tokens: Vec<Token> = lexer.collect();
    
    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    // Type checking
    let mut type_checker = typecheck::TypeChecker::new();
    type_checker.check(&ast)?;

    // Codegen
    let context = Context::create();
    let mut codegen = CodeGen::new(&context);
    codegen.generate(&ast)?;

    // Object file generation & executable linking
    codegen.write_object_file("output.o")?;

    let status = Command::new("cc")
        .args(&["output.o", "-o", "program"])
        .status()
        .map_err(|e| format!("Failed to link program: {}", e))?;

    if !status.success() {
        return Err("Linking failed".to_string());
    }

    println!("Successfully compiled to ./program");

    // Clean up
    std::fs::remove_file("output.o")
        .map_err(|e| format!("Failed to clean up object file: {}", e))?;

    Ok(())
}
