# Ahlang

Ahlang is a statically and strongly typed scripting language with a syntax *very* inspired by Rust. It's designed to be a general purpose scripting language that can be easily embedded in other Rust programs. Although performance is a major consideration, it's not expected to be able to compete in this aspect with other more established scripting languages like Lua.

## Features
- [ ] Static typing
- [ ] Object oriented programming

## Syntax example
```rust
fn example(y: i32) -> str {
    const x: i32 = 1;
    
    let y: f64 = y as f64;
    y += 1.2564;
    
    let z = x + y as i32;

    let a: bool = true;

    if a && z > 0 {
        print("Z is {}", z);
    }

    return "Hello, world!";
}
```

## Roadmap
- [x] Lexer
- [ ] Parser
  - [X] Pratt parser for expressions
  - [ ] Implement all statement types
- [ ] Abstract syntax tree walking interpreter
- [ ] Standard library
- [ ] Transition to bytecode interpreter?

