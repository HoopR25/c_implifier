A library that makes coding in Rust just a little bit simpler. 

To set this up:
1. In the dependencies section of the Cargo.toml file, paste the following line in:
c_implifier = {git = "https://github.com/HoopR25/c_implifier", branch = "master"}
2. Then, in your .rs file, you can write: use c_implifier::[the gen of the commands]
3. Now, you can call the commands

Example:

Cargo.toml:

```toml
[package]
name = "example1"
version = "0.1.0"
edition = "2021"

[dependencies]
c_implifier = { git = "https://github.com/HoopR25/c_implifier", branch = "master"}

```

main.rs:

```rust
//  Adds the c_implifier dependency
use c_implifier::gen1cmds;

fn main() {
    print!("Hello World!");
    //  Prints an empty line (\n) to start a new line
    gen1cmds::endl!();
}
```





