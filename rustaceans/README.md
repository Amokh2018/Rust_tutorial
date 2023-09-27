## Rust_tutorial
Tutorial to learn Rust programming language
Trying to learn Rust

## How to Install
* [Install Steps](https://doc.rust-lang.org/book/ch01-01-installation.html)
* Pro-top, run `rustup update` to update your installation
* Check version do this: `rustc --version`

## How to compile
* `rustc myfunc.rs` in this case, the `<yourfile>.rs` is compiled
* Next you run it: `./myfunc.rs`

## How to use Cargo?

* `cargo new hello_cargo && cd hello_cargo`

## Quick example 

```rust
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}
```


* [learn by example guide] (https://doc.rust-lang.org/rust-by-example/)


## Make a function

* `cargo new hello_func && cd hello_func` <br>
and then, change `main.rs` to:

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```
* and then: `cargo build && cargo run`


