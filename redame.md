```
$ cargo new project-name --bin // make new project, NOT library
$ cargo build // build project, run from directory with Cargo.toml
$ cargo build --release // release build
$ cargo run // run project, run from directory with Cargo.toml
$ rustup component add clippy // add linter
$ cargo clippy --all-targets --all-features // run linter
$ cargo clippy -- -D clippy::all // run linter
```
