```
// installing
$ curl https://sh.rustup.rs -sSf | sh // install rust
$ source $HOME/.cargo/env // use rust now!

$ cargo new project-name // make new project, NOT library
$ cargo new project-name --bin // make new project, NOT library
$ cargo new project-name --lib // make new project, library

// run / build
$ cargo run // run project, run from directory with Cargo.toml
$ cargo build // build project, run from directory with Cargo.toml
$ cargo build --release // release build

// linter
$ rustup component add clippy // add linter
$ cargo clippy --all-targets --all-features // run linter
$ cargo clippy -- -D clippy::all // run linter
```

Rust is installed now. Great!
To get started you may need to restart your current shell.
This would reload its PATH environment variable to include
Cargo's bin directory (%USERPROFILE%\.cargo\bin).
Press the Enter key to continue.
