use std::io;

use ahlang::*;
use termion::{raw::IntoRawMode, input::TermRead};
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

// fn next_line() {
//     print!("\n");
// }

fn line_start() {
    print!("\x1B[0G");
}

// fn line_end() {
//     print!("{}", termion::cursor::Right(2048));
// }

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

/// Continuation line
const CONT_LINE: &str = "└─ ";
const START_LINE: &str = ">> ";

fn print_line_start() {
    print!("\x1B[32m{}\x1B[0m", START_LINE);
}

fn print_continuation_start() {
    print!("\x1B[32m{}\x1B[0m", CONT_LINE);
}

// TODO: Paste, move left and right, tab
fn repl() {
    let mut engine = ReplEngine::new();

    let mut history_stack: Vec<String> = Vec::new();
    #[allow(unused_assignments)]
    let mut history_ptr = 0;

    println!("\x1B[33;1;4;5mAhlang REPL v{}\x1B[0m", env!("CARGO_PKG_VERSION"));
    println!("\x1B[33;2mPress Ctrl + Z to exit, Ctrl + H for more information\x1B[0m");

    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdout = io::BufWriter::new(stdout);

    // enable raw mode
    let stdin = io::stdin();
    let mut events = stdin.events();

    'repl: loop {
        line_start();
        print_line_start();
        stdout.flush().unwrap();

        let mut input = String::new();
        history_ptr = history_stack.len();

        let mut line_start_locs = vec![0];
        let mut in_block = 0;

        loop {
            if let Some(Ok(event)) = events.next() {
                match event {
                    termion::event::Event::Key(key) => {
                        match key {
                            termion::event::Key::Char(chr) => {
                                if chr == '{' {
                                    in_block += 1;
                                } else if chr == '}' {
                                    in_block -= 1;
                                }

                                if chr == '\n' {
                                    next_line_start();
                                    // if input isnt ending in semicolon or rbrace, don't break, go to next line and print ..
                                    let is_comment = input.starts_with("//");
                                    if ((input.ends_with(';') || input.ends_with('}')) && in_block < 1) || is_comment || input.is_empty() {
                                        break;
                                    } else {
                                        // add \n to input
                                        input.push('\n');

                                        // save where in the input this line starts
                                        line_start_locs.push(input.len());

                                        print_continuation_start();
                                        stdout.flush().unwrap();
                                        continue;
                                    }
                                }

                                input.push(chr);
                                print!("{}", chr);
                                stdout.flush().unwrap();
                            },
                            termion::event::Key::Ctrl('z') => {
                                next_line_start();
                                print!("\x1B[33m! Exiting...\x1B[0m");
                                next_line_start();
                                break 'repl;
                            },
                            termion::event::Key::Ctrl('h') => {
                                next_line_start();
                                //print!("\x1B[33m! Help:\n! Ctrl + Z to exit\n! Ctrl + H for help \n To write multiline statements, press enter without ending the line in a semicolon.\x1B[0m");
                                print!("\x1B[33m");
                                print!("! Help:");
                                next_line_start();
                                print!("- Ctrl + Z to exit");
                                next_line_start();
                                print!("- Ctrl + H for help");
                                next_line_start();
                                print!("- Ctrl + T to inspect variables");
                                next_line_start();
                                print!("- To write multiline statements, press enter before ending the statement.");
                                next_line_start();
                                break;
                            },
                            termion::event::Key::Ctrl('t') => {
                                next_line_start();
                                print!("");
                                print!("\x1B[33;4m! Variables:\x1B[0m");
                                next_line_start();
                                
                                // Assuming you have an `engine` instance of `YourEngineType`
                                for (k, v) in engine.get_vars() {
                                    print!("\x1B[33m {}: \x1B[33;2m{:?}\x1B[0m", k, v);
                                    next_line_start();
                                }
                                break;
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
                                            chars_on_line += START_LINE.chars().count();
                                        } else {
                                            chars_on_line += CONT_LINE.chars().count();
                                        }

                                        move_right(chars_on_line as u16);

                                        stdout.flush().unwrap();
                                    }
                                    continue;
                                }

                                // delete the last char and pop it from the input
                                let popped = input.pop();
                                if popped.is_none() {
                                    continue;
                                }
                                let popped = popped.unwrap();

                                if popped == '{' {
                                    in_block -= 1;
                                } else if popped == '}' {
                                    in_block += 1;
                                }
                                write!(stdout, "\x08 \x08").unwrap();
                                stdout.flush().unwrap();
                            },
                            // termion::event::Key::Left => {
                            //     print!("{}", termion::cursor::Left(1));
                            //     stdout.flush().unwrap();
                            // },
                            // termion::event::Key::Right => {
                            //     print!("{}", termion::cursor::Right(1));
                            //     stdout.flush().unwrap();
                            // },
                            termion::event::Key::Up => {
                                // the latest input is at the end of the stack
                                // if we are at the start of the stack, don't do anything
                                if history_ptr == 0 {
                                    continue;
                                }

                                // clear all multiline input
                                for _ in 0..line_start_locs.len() - 1 {
                                    delete_line();
                                    last_line();
                                }
                                delete_line();

                                // move to start of line
                                line_start();

                                // seperate input by \n
                                let line: Vec<&str> = history_stack[history_ptr - 1].split('\n').collect();
                                
                                // fill line_start_locs with the start of each line
                                line_start_locs.clear();
                                line_start_locs.push(0);

                                for (i, c) in history_stack[history_ptr - 1].char_indices() {
                                    if c == '\n' {
                                        line_start_locs.push(i + 1);
                                    }
                                }
                                
                                // first line
                                print_line_start();
                                write!(stdout, "{}", line[0]).unwrap();
                                stdout.flush().unwrap();
                                
                                // other lines
                                for i in 1..line.len() {
                                    next_line_start();
                                    print_continuation_start();
                                    write!(stdout, "{}", line[i]).unwrap();
                                    stdout.flush().unwrap();
                                }


                                // set input to history stack string
                                input = history_stack[history_ptr - 1].clone();

                                // move the stack pointer up
                                if history_ptr > 0 {
                                    history_ptr -= 1;
                                }
                            },
                            termion::event::Key::Down => {
                                // if we are at the end of the stack, don't do anything
                                if history_ptr == history_stack.len() {
                                    continue;
                                }

                                if history_ptr == history_stack.len() - 1 {
                                    // clear all multiline input
                                    for _ in 0..line_start_locs.len() - 1 {
                                        delete_line();
                                        last_line();
                                    }
                                    delete_line();

                                    // clear input
                                    input.clear();

                                    break;
                                }

                                // clear all multiline input
                                for _ in 0..line_start_locs.len() - 1 {
                                    delete_line();
                                    last_line();
                                }
                                delete_line();

                                // move to start of line
                                line_start();

                                // seperate input by \n
                                let line: Vec<&str> = history_stack[history_ptr + 1].split('\n').collect(); 

                                // fill line_start_locs with the start of each line
                                line_start_locs.clear();
                                line_start_locs.push(0);

                                for (i, c) in history_stack[history_ptr + 1].char_indices() {
                                    if c == '\n' {
                                        line_start_locs.push(i + 1);
                                    }
                                }

                                // first line
                                print_line_start();
                                write!(stdout, "{}", line[0]).unwrap();
                                stdout.flush().unwrap();

                                // other lines
                                for i in 1..line.len() {
                                    next_line_start();
                                    print_continuation_start();
                                    write!(stdout, "{}", line[i]).unwrap();
                                    stdout.flush().unwrap();
                                }

                                // set input to history stack string
                                input = history_stack[history_ptr + 1].clone();

                                // move the stack pointer down
                                if history_ptr < history_stack.len() {
                                    history_ptr += 1;
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }

        input = input.trim().to_owned();

        if input == "" {
            continue;
        }
        
        history_stack.push(input.clone());
    
        if input.starts_with("//") {
            continue;
        }

        let res = engine.eval(input.as_str());
        //let res: Result<(), String> = Ok(());

        if res.is_err() {
            println!("\x1B[31m! {}\x1B[0m", res.unwrap_err());
            line_start();
            continue;
        }

        line_start();
    }
}