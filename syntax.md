## Built in types
- string
- int
- float
- bool
- array
- vector


### Function
```
fn main() -> () {

};
```

### Variable
```
let x: int = 5;
```

### Constant
```
const y: int = 5;
```

## Struct
```
struct Point {
    x: int,
    y: int,

    Point();
    ~Point();
    
};
```

## Enum
```
enum Direction {
    Up,
    Down,
    Left,
    Right,
};
```

## Conditional
```
if x == 5 {
    println!("x is five!");
} else if x == 6 {
    println!("x is six!");
} else {
    print("x is not five or six");
};
```

## Loop
```
loop {
    println!("Loop forever!");
}
```

## While
```
while x < 100 {
    x += 1;
}
```

## For
```
for x in range(0, 10) {
    println!("{}", x); // x: int
}
```

## Match
```
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

## String formatting
```
fmt("Hello, {}!", "world"); // => "Hello, world!"
```