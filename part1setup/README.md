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

Cargo
```
part1setup $ cargo new hello_cargo 
```

```
├── part1setup
│   ├── helloworld
│   │   └── main.rs
│   └── hello_cargo
│       ├── Cargo.toml
│       └── src
│           └── main.rs

```

```
part1setup $ cargo build
   Compiling hello_cargo v0.1.0 (/Users/part1setup/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.72s
part1setup $ ./target/debug/hello_cargo
Hello, world!
```
this is equivalent to 
```
part1setup $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
    Running `target/debug/hello_cargo`
Hello, world!
```

```
cargo check
```

Building for Release
```
cargo build --release
./target/release/hello_cargo
Hello, world!
```

this is equivalent to 
```
cargo run --release
```

To use Rust in a Jupyter Notebook
Install the EvCxR Jupyter Kernel
```
$ python3 -m venv venv
$ source venv/bin/activate
(venv) $ pip install jupyter
(venv) $ cargo install evcxr_jupyter
(venv) $ evcxr_jupyter --install
(venv) $ jupyter lab
```
After run the `jupyter lab` command you should see the tab below added to your browser:
<img src="https://datacrayon.com/assets/images/520468c29709badd52ffbfa30716d5f6.png">




