# brainrust
A brainfuck interpreter in Rust, written as an exercise.

The actual code is in [src/main.rs](src/main.rs).

## How to run

You'll need [Rust](http://www.rust-lang.org) and [Cargo](http://doc.crates.io), both already come with the standard Rust installer.

Also, libncurses needs to be installed on your system since brainrust uses [rs-ncurses](https://github.com/jeaye/ncurses-rs). See [here](https://github.com/jeaye/ncurses-rs#building) for intructions.

1. clone this repository
1. change into its directory
1. execute `cargo run examples/hello_world.bf`
