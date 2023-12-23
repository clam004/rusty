# Setup Your Rust Development Environment

[Installing Rust using rustup](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)

```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

```
1) Proceed with installation (default)
2) Customize installation
3) Cancel installation
>1

Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, run:
source "$HOME/.cargo/env"
```

```
rustc --version
```

```
rustc 1.74.1 (a28077b28 2023-12-04)
```

```
rustup update
```

```
rustup self uninstall
```

 open the local documentation in your browser
```
rustup doc
```

```
rusty/part1setup $ mkdir hello_world
cd hello_world
touch main.rs
```

```rust
// this code goes in source file  main.rs
/*
To compile this file:
   hello_world $ rustc main.rs
To run this file from your terminal while withing the same director
   hello_world $ ./main
*/

fn main() {
    println!("Hello, world!");
}
```

```
$ rustc main.rs
$ ./main
Hello, world!
```