# Nova Lang

A statically-typed programming language with LLVM-based compilation, implemented in Rust.

## Features

- Static type checking 
- Recursive descent parser with error reporting
- LLVM IR code generation via inkwell
- Support for:
  - Integer, float, boolean, and string types
  - Variables with mandatory type annotations
  - Functions with typed parameters and return values
  - Basic arithmetic operations (+, -, *, /)
  - Expression-based syntax

## Example

```nova
fn add(x: i32, y: i32): i32 {
    return x + y;
}

fn main(): i32 {
    let result: i32 = add(40, 2);
    return result;
}
```

## Project Structure

- `src/lexer.rs` - Token definitions and lexical analysis using logos
- `src/parser.rs` - AST definitions and recursive descent parser
- `src/types.rs` - Type system implementation
- `src/typecheck.rs` - Static type checking and inference
- `src/codegen.rs` - LLVM IR generation using inkwell

## Building

Requires Rust 1.70+ and LLVM 15.0+

```bash
cargo build
cargo test
```

## Testing

Run the test suite:
```bash
cargo test
```

## License

MIT
