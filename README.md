# Rust-OneDrive

[![Build Status](https://travis-ci.com/sreeise/rust-onedrive.svg?branch=master)](https://travis-ci.com/sreeise/rust-onedrive)

### OneDrive API - Built with Rust

Disclaimer:
This project is a work in progress and is not on yet on crates.io. Since a good part of this API is unstable, 
it should not be used in production.
 
Only Linux has been tested to work. Windows and MacOS may work but is not guaranteed.

### Building
Normal Rust build using cargo

    $ cargo build

### Requires Rust nightly

The nightly version is set in the rust-toolchain file.

### Clippy Lints

Clippy runs in the CI tests and is required to pass. More info
on clippy can be found here: https://github.com/rust-lang/rust-clippy

    $ cd rust-onedrive
    $ rustup component add clippy
    $ cargo clippy --all

### Formatting

Cargo fmt is used for formatting. 

    $ cargo fmt

### Tests and Docs

Of the portions that are implemented there are also examples and docs. Run: 

    $ cargo doc --no-deps --open

Tests are run on travis ci and can be run locally through cargo like normal Rust builds:

    $ cargo test -all
    
### Benchmarks

Criterion.rs is being used to run benchmarks. The basic commands are the same;

    $ cargo bench
    
The benchmark tests are a work in progress and several tests still need to be
added/worked on.

### Use - subject to change.

    See examples directory.