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

## using dependencies in a cargo project
Add package name and version to *Cargo.toml* under *[dependencies]* e.g
```
[dependencies]
rand = "0.3.14"
```
then run
```
$ cargo build
```
in the project's root folder

## Reproducible builds
On the initial project build, Cargo figures out all the the versions of the specified dependencies and writes them to the *Cargo.lock* file. When you build your project in the future, Cargo will see that *Cargo.lock* file and use the versions specified there. This causes your application dependencies versions to remain the same unless explicitly upgraded.

To update a crate to the latest release of the current version, run
```
$ cargo update
```
To update a crate to a specific major version, edit the crate's entry in the *Cargo.toml* file to that version i.e from 0.3.0 to 0.4.0 then run
```
$ cargo update
```

## Building your project's dependencies documentations
You can build the documentations of your project's dependencies and open them on the browser running:
```r
$ cargo doc --open
```
in the project's root folder.
