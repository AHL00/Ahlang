# Ahlang
Ahlang is a statically and strongly typed scripting language with a syntax *very* inspired by Rust. It's designed to be a general purpose scripting language that can be easily embedded in other Rust programs. Although performance is a major consideration, it's not expected to be able to compete in this aspect with other more established scripting languages like Lua.

[![Build](https://github.com/AHL00/Ahlang/workflows/Build/badge.svg)](https://github.com/AHL00/Ahlang/actions/workflows/build_check.yml)
[![Lines of code](https://tokei.rs/b1/github/AHL00/Ahlang?category=code)](https://github.com/AHL00/Ahlang)

## Features
- [ ] Static typing
- [ ] Object oriented programming
- [ ] REPL


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
};
```
## REPL
- [X] Multiline input
- [X] History

![REPL](/images/repl.png)

## Roadmap
- [x] Lexer
- [ ] Parser
  - [X] Pratt parser for expressions
  - [ ] Implement all statement types
- [ ] Abstract syntax tree walking interpreter
- [ ] Standard library
- [ ] Transition to bytecode interpreter?

