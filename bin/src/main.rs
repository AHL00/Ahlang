use std::{collections::VecDeque, io};

use ahlang::*;
use termion::{raw::IntoRawMode, screen::{AlternateScreen, ToAlternateScreen}, input::TermRead, cursor};
use std::io::Write;

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

fn next_line() {
    print!("\n");
}

fn line_start() {
    print!("\x1B[0G");
}

fn line_end() {
    print!("{}", termion::cursor::Right(2048));
}

fn move_right(n: u16) {
    print!("{}", termion::cursor::Right(n));
}

fn next_line_start() {
    print!("\n\x1B[0G");
}

fn delete_line() {
    print!("\x1B[2K");
}

fn last_line() {
    print!("\x1B[1F");
}


const continuation_line: &str = ".. ";
const start_line: &str = "> ";

fn repl() {
    let mut engine = ReplEngine::new();

    let mut history_stack: Vec<String> = Vec::new();
    let mut history_ptr = 0;

    println!("\x1B[34mAhlang REPL v{}\x1B[0m", env!("CARGO_PKG_VERSION"));
    println!("\x1B[32mType 'exit' to exit, 'help' for more information\x1B[0m");

    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdout = io::BufWriter::new(stdout);

    // enable raw mode
    let stdin = io::stdin();
    let mut events = stdin.events();

    'repl: loop {

        write!(stdout, "\x1B[32m{}\x1B[0m", start_line).unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        history_ptr = history_stack.len();

        let mut line_start_locs = vec![0];

        loop {
            if let Some(Ok(event)) = events.next() {
                match event {
                    termion::event::Event::Key(key) => {
                        match key {
                            termion::event::Key::Char(chr) => {
                                if chr == '\n' {
                                    next_line_start();
                                    // if input isnt ending in semicolon, don't break, go to next line and print ..
                                    let is_comment = input.starts_with("//");

                                    if input.ends_with(';') || is_comment || input.is_empty() {
                                        break;
                                    } else {
                                        // add \n to input
                                        input.push('\n');

                                        // save where in the input this line starts
                                        line_start_locs.push(input.len());

                                        write!(stdout, "{}", continuation_line).unwrap();
                                        stdout.flush().unwrap();
                                        continue;
                                    }
                                }

                                input.push(chr);
                                print!("{}", chr);
                                stdout.flush().unwrap();
                            },
                            termion::event::Key::Ctrl('c') => {
                                next_line_start();
                                print!("\x1B[33m! Exiting...\x1B[0m");
                                next_line_start();
                                break 'repl;
                            },
                            termion::event::Key::Backspace => {
                                // if we are at the start of the line, don't delete
                                if input.len() == *line_start_locs.last().unwrap_or(&0) {
                                    // if line_start_locs.len() > 1, this is a continuation line
                                    // so we need to delete the .. and go back to the previous line
                                    if line_start_locs.len() > 1 {
                                        // delete .. and go back to previous line
                                        delete_line();
                                        last_line();
                                        
                                        input.pop(); // pop the \n
                                        line_start_locs.pop(); // pop the line start location

                                        // check characters on this line
                                        let mut chars_on_line = input.len() - line_start_locs[line_start_locs.len() - 1];
                                        // account for line start
                                        if line_start_locs.len() == 1 {
                                            chars_on_line += start_line.len();
                                        } else {
                                            chars_on_line += continuation_line.len();
                                        }

                                        move_right(chars_on_line as u16);

                                        stdout.flush().unwrap();
                                    }
                                    continue;
                                }

                                // delete the last char and pop it from the input
                                input.pop();
                                write!(stdout, "\x08 \x08").unwrap();
                                stdout.flush().unwrap();
                            },
                            termion::event::Key::Left => {
                                print!("{}", termion::cursor::Left(1));
                                stdout.flush().unwrap();
                            },
                            termion::event::Key::Right => {
                                print!("{}", termion::cursor::Right(1));
                                stdout.flush().unwrap();
                            },
                            termion::event::Key::Up => {
                                if history_ptr == 0 {
                                    continue;
                                }

                                history_ptr -= 1;

                                delete_line();
                                line_start();

                                input = history_stack[history_ptr].clone();
                                print!("{}", input);
                                stdout.flush().unwrap();
                            },
                            termion::event::Key::Down => {
                                if history_ptr == history_stack.len() {
                                    continue;
                                }

                                history_ptr += 1;

                                delete_line();
                                line_start();

                                if history_ptr == history_stack.len() {
                                    input = String::new();
                                } else {
                                    input = history_stack[history_ptr].clone();
                                }

                                print!("{}", input);
                                stdout.flush().unwrap();
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }

        input = input.trim().to_owned();

        if input.starts_with("//") {
            continue;
        }

        if input == "exit" {
            write!(stdout, "\x1B[31m! Exiting\x1B[0m").unwrap();
            break;
        }

        if input == "help" {
            println!("Type 'exit' to exit, 'help' for more information");
            println!("If a line ends with a semicolon, it will be evaluated immediately.");
            println!("Otherwise, the statement will be continued on the next line with '.. '");
            continue;
        }

        if !input.is_empty() {
            history_stack.push(input.clone());
        }

        let res = engine.eval(input.as_str());

        if res.is_err() {
            println!("\x1B[31m! {}\x1B[0m", res.unwrap_err());
            line_start();
            continue;
        }

        line_start();
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