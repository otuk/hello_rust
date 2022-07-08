/*!
This chapter is only about one thing; getting code running in rust.

This is the file structure for our project so far:
```bash
hello_rust
├── Cargo.lock
├── Cargo.toml
└── src
    ├── chapter_00
    │   └── mod.rs
    │
    └── main.rs
```
The src directory
is where our source code is.  We have edited the main.rs source file and
the chapter_00/mod.rs file we are in. The directory chapter_00 is our
_chapter 00_ module directory. Cargo.lock is not to be hand edited, it reflects
the status of Cargo.toml file contents at a certain point in time. 
Cargo.toml file is where we describe the hello_rust project and its
dependencies, for now it is just a boiler plate file that ```cargo new``` command
created for me when I started this project. More on this later.

Rust allows us to break our code in separate functions, files, modules and crates.
We are in hello_rust crate, chapter_00 module, and mod.rs file to execute the
print_the_ubiquitous_hw() function.

To run this project all we need to do is execute 
```bash
cargo run
``` 
from the project root hello_rust directory.

*/
/**
This function will use the *println!* macro to
print "Hello, world." to screen.
In rusts _macro_ names end with an exclamation mark character.

# Arguments
None

# Remarks
prints hello, world.

# Examples
execute```cargo run```to see Hello, world printed on the standarad output.

*/
pub fn print_the_ubiquitous_hw() {
    println!("Hello, world.");
}
