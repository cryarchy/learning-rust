# Cargo
Takes care of:  
- Building your code
- Downloading the libraries your code depends on (dependencies)
- Building those libraries

## Creating a cargo project.
```
$ cargo new hello_cargo --bin
```
The `--bin` argument to `cargo new` specifies that you want to create an
executable as opposed to a library.
The command creates the folders:  
- hello_cargo - The project root folder
- hello_cargo/src - Where all your code lives

The files:  
- hello_cargo/Cargo.toml - configuration file
- hello_cargo/src/main.rs - source file

It also initializes a new git repository in the *hello_cargo* directory.

## Building
```
$ cargo build
```
Executable is created in *target/debug/*.  
A *Cargo.lock* file is also created in the project's root folder to keep track
of the dependencies in your application.

## To compile and run
```
$ cargo run
```
Compiles and runs your application.

## Building for release
```
$ cargo build --release
```
Compiles the project with optimizations. Executable is created in
*target/release*. The optimizations result in faster execution time but slower
compile time.
