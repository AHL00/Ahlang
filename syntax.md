// built in type
- string
- int
- float
- bool
- char
- tuple
- array
- vector


// function
fn main() -> () {

}

// variable
let x: int = 5;

// constant
const y: int = 5;

// struct
struct Point {
    x: int,
    y: int,
}

impl Point {
    fn new(x: int, y: int) -> Point {
        Point { x: x, y: y }
    }
}

// enum
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// conditional
if x == 5 {
    println!("x is five!");
} else if x == 6 {
    println!("x is six!");
} else {
    print("x is not five or six");
}

// loop
loop {
    println!("Loop forever!");
}

// while
while x < 100 {
    x += 1;
}

// for
for x in range(0, 10) {
    println!("{}", x); // x: int
}

// match
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}

// string formatting
fmt("Hello, {}!", "world"); // => "Hello, world!"