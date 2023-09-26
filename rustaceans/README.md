# Rust_tutorial
Tutorial to learn Rust programming language


## Trying to learn Rust
Quick example 

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
