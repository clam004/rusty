# Managing growing projects with Packages, Crates and Modules

As a project grows, you should organize code by splitting it into multiple modules and then multiple files.

By grouping related functionality and separating code with distinct features, you’ll clarify where to find code that implements a particular feature and where to go to change how a feature works.

The way you write code defines which parts are public for other code to use and which parts are private implementation details that you reserve the right to change.

When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant, or other item and what that item means.
The nested context in which code is written has a set of names that are defined as “in scope. You can create scopes and change which names are in or out of scope. You can’t have two items with the same name in the same scope; tools are available to resolve name conflicts.

## the module system

collectively referred to as the module system are a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs:

1. Packages: A Cargo feature that lets you build, test, and share crates
2. Crates: A tree of modules that produces a library or executable
3. Modules and use: Let you control the organization, scope, and privacy of paths
4. Paths: A way of naming an item, such as a struct, function, or module



### Crates

A crate is the smallest amount of code that the Rust compiler considers at a time, there are 2 forms of crates:

    1. Binary crates are programs you can compile to an executable that you can run, such as a command-line program or a server, each crate must have a function called main that defines what happens when the executable runs.

    2. Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects. For example, the rand crate. The library crate is usually what programmers are referring to when they say only “crate” and the word "crate"  is used interchangeably with the general programming concept of a “library".


### Packages and Crates

A package is a bundle of one or more crates that provides a set of functionality. Packages use a Cargo.toml to describe how to build those crates. A package must contain at least one crate, that one can be a library or binary crate.  Packages can contain many binary crates, but only one library crate.

Cargo is actually a package that contains the binary crate for the command-line tool you’ve been using to build your code. The Cargo package also contains a library crate that the binary crate depends on. Other projects can depend on the Cargo library crate to use the same logic the Cargo command-line tool uses.

Remember in a previous lesson when we did `cargo new` ? If you look again at the output, it says that it created a package with a binary crate named my-project.

```rust
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
```

 The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate. A crate root is created for you when you do `cargo new <new-project>`, it is in `src/main.rs`.

 ```
 $ ls my-project/src
main.rs
 ```

 After we run `cargo new`, in the project directory, there’s a `Cargo.toml` file, but there’s no mention of `src/main.rs`, thats because Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.

 ```
 [package]
name = "my-project"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
 ```

 If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

 ### Example

 This tutorial  https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee  shows you the step by step evolution of how your functions/structs/etc can be laid out: 

 1. into a separate modules `mod` in the same `main.rs`` file
 2. into other files at the same level as `main.rs`
 3. into a folder at the same level as `main.rs`
 4. into `src/lib.rs` where src is at the smae level as `Cargo.toml` and there is no `main.rs` because `fn main() {` is in a file within `src/bin`
  - https://github.com/clam004/rust-split-example/commit/64efc45d1e3b3a42d69d2169ee8af6877a278850
 5. into a different crate. `main.rs` and `lib.rs` are in two separate folders with their own `Cargo.toml`. The `Cargo.toml` with a `src/main.rs` has under `[dependencies]` `de_lib = { path = "../de_lib" }`
  - https://github.com/clam004/rust-split-example

#### To build and run layouts 1 - 3 above: 

```
part3projects $ mkdir layout123
part3projects $ cd layout123
part3projects/layout123 $ cargo new interface
part3projects/layout123 $ cd interface
part3projects/layout123/interface $ ls
Cargo.toml      src
part3projects/layout123/interface $ cargo build
   Compiling interface v0.1.0 (/Users/part3projects/taxonomy/interface)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
part3projects/layout123/interface $ ls
Cargo.lock      Cargo.toml      src             target
part3projects/layout123/interface $ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/interface`
Hello, world!
part3projects/layout123/interface $ 
```

change `interface/src/main.rs` from 

```rust
fn main() {
    println!("Hello, world!");
}
```

to 

```rust
// interface/src/main.rs
mod hello_speaker {
    pub fn greet() {
        println!("Hello, world!");
    }
}

fn main() {
    crate::hello_speaker::greet();
}
```

or 

```rust
// interface/src/main.rs
mod hello_speaker {
    pub fn greet() {
        println!("hello world");
    }
}

use crate::hello_speaker::greet; // or `use crate::hello_speaker::*;` 

fn main() {
    greet();
}
```

to check your changes

```
interface $ cargo run
   Compiling interface v0.1.0 (/Users/part3projects/taxonomy/interface)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/interface`
hello world
```

in a file `sup.rs` at the same level as `main.rs` 

```
./
 |_Cargo.toml
 |_target/
 |_src/
    |_ main.rs
    |_ sup.rs
```

write in `interface/src/sup.rs`:

```rust
// interface/src/sup.rs
pub fn say_sup() {
    println!("sup");
}
```

test your changes:

```
cargo run
   Compiling interface v0.1.0 (/Users//interface)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/interface`
hello world
sup
```

#### To build and run layout 4 above

we are going to setup this directory structure:

```
./
 |_Cargo.toml
 |_target/
 |_src/
    |_bin/
    |   |_my_executable.rs
    |_module_abc/
    |   |_mod.rs
    |   |_module_a.rs
    |   |_module_b.rs
    |   |_module_c.rs
    |_ lib.rs
```

by 

```
$ mkdir layout4
$ cd layout4
$ git clone https://github.com/clam004/rust-split-example.git
$ cd rust-split-example
$ git checkout 64efc45d1e3b3a42d69d2169ee8af6877a278850
$ ls
-> Cargo.lock      Cargo.toml      src
$ cargo run
   Compiling desintegration_example v0.1.0 (/Users/layout4/rust-split-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.91s
     Running `target/debug/my_executable`
first one: 2, -1 = -2
second one: 4, -2 = -8
third one: 8, -4 = -32
$ ls
Cargo.lock      Cargo.toml      src             target
```

#### layout 5 is 

```
$ mkdir layout5
$ cd layout5
$ git clone https://github.com/clam004/rust-split-example.git
$ cd rust-split-example
$ git checkout 5c292b65c8fe61f17f55a673a5222d7bef8ff3f0
$ cargo run
```








