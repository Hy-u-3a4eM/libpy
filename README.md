# libpy
This library imports functions from the python standard library.

## Examples
### input! macro
```rust
use libpy::std::input;

fn main() {
    let a = input!();
    let b = input!("Enter your name: ");
    
    println!("{}", a);
    println!("{}", b);
}
```

### int! macro
```rust
use libpy::std::int;

fn main() {
    let a = int!();
    let b = int!("42".to_string());

    println!("{}", a);
    println!("{}", b);
}

## About
This project was created to make it easier for the programmer to work by importing functions familiar to him.
