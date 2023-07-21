use std::{collections::VecDeque};
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEventKind},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};

use ahlang::*;

fn main() {
    // Binary entry point
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: ahlang <file>");
        return;
    }

    if args[1] == "help" {
        println!("Usage: ahlang <file>");
        println!("Version: ahlang version");
        println!("REPL: ahlang repl");
        return;
    }

    if args[1] == "version" {
        println!("ahlang {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args[1] == "repl" {
        repl();
        return;
    }

    let file = std::fs::read_to_string(&args[1]).unwrap();
    let source_str = file.as_str();
    
    let mut l = Lexer::new();
    l.set_input(source_str);

    let res = l.tokenize();
    let tokens: &lexer::Tokens;
    
    if res.is_err() {
        println!("Error: {}", res.unwrap_err());
        return;
    } else {
        tokens = res.unwrap();
    }
    
    let mut p = Parser::new();
    p.set_tokens(tokens);
    let res = p.parse();

    if res.is_err() {
        println!("Error: {}", res.unwrap_err());
        return;
    }

    let mut i = Interpreter::new();
    match i.run(res.unwrap()) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}

fn repl() {
    use std::io::Write;
    
    let mut engine = ReplEngine::new();

    let mut history_stack: Stack<String> = Stack::new(10);
    let mut history_ptr = 0;

    println!("\x1B[34mAhlang REPL v{}\x1B[0m", env!("CARGO_PKG_VERSION"));
    println!("\x1B[32mType 'exit' to exit, 'help' for more information\x1B[0m");

    execute!(
        std::io::stdout(),
        terminal::Clear(ClearType::FromCursorDown),
        terminal::SetTitle("Ahlang REPL"),

    )
    .unwrap();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        
        let mut input = String::new();
        history_ptr = history_stack.len();
        
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_owned();
        
        if input == "exit" {
            return;
        }
        
        if input == "help" {
            println!("Type 'exit' to exit, 'help' for more information");
            continue;
        }
        
        if input != "" {
            history_stack.push(input.clone());
        }

        let res = engine.eval(input.as_str());

        if res.is_err() {
            println!("\x1B[31m! {}\x1B[0m", res.unwrap_err());
            continue;
        }
    }
}

struct Stack<T> {
    deque: VecDeque<T>,
    size: usize,
}

impl<T> Stack<T> {
    fn new(size: usize) -> Stack<T> {
        Stack {
            deque: VecDeque::with_capacity(size),
            size,
        }
    }

    fn push(&mut self, item: T) {
        if self.deque.len() == self.size {
            self.deque.pop_front();
        }

        self.deque.push_back(item);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.deque.get(index)
    }

    fn pop(&mut self) -> Option<T> {
        self.deque.pop_back()
    }

    fn last(&self) -> Option<&T> {
        self.deque.back()
    }

    fn len(&self) -> usize {
        self.deque.len()
    }
}


impl<T> std::ops::Index<usize> for Stack<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.deque[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Stack<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.deque.index_mut(index)
    }
}