use nova_lang::{lexer::Token, parser::Parser, codegen::CodeGen, AstNode};
use inkwell::context::Context;

#[test]
fn test_full_compilation() {
    let source = r#"fn main() {
        let x: i32 = 42;
        return x;
    }"#;

    // Lexing
    let mut lexer = Token::lexer(source);
    let tokens: Vec<_> = lexer.collect();
    assert!(tokens.len() > 0);

    // Parsing
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    // Code generation
    let context = Context::create();
    let mut codegen = CodeGen::new(&context);
    assert!(codegen.generate(&ast).is_ok());

    // Optional: Write output to file
    let result = codegen.write_bitcode_to_file("output.bc");
    assert!(result.is_ok());
}
