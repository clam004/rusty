

```
$ cargo new guessing_game
$ cd guessing_game
$ cargo run
   Compiling guessing_game v0.1.0 (/Users/carsonlam/Desktop/Projects/rusty/part2basics/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 79
Please input your guess.
90
You guessed: 90
Too big!
Please input your guess.
*
Please input your guess.
1
You guessed: 1
Too small!
Please input your guess.

Please input your guess.
79
You guessed: 79
You win!
```

to update the crates, aka dependencies, in Cargo.toml
```
cargo update
```

to see docs for your dependencies, from the same level as the `Cargo.toml``
```
cargo doc --open
```