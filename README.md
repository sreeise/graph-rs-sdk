# Rust-OneDrive

[![Build Status](https://travis-ci.com/sreeise/rust-onedrive.svg?branch=master)](https://travis-ci.com/sreeise/rust-onedrive)
[![Build status](https://ci.appveyor.com/api/projects/status/llvpt7xiy53dmo7a/branch/master?svg=true)](https://ci.appveyor.com/project/sreeise/rust-onedrive)

### OneDrive API Client - Built with Rust

Disclaimer:
This project is a work in progress and is not on yet on crates.io. Since a good part of this API is unstable, 
it should not be used in production.

Linux and Windows x86_64 are confirmed to work. Other versions of Windows and Linux distributions
as well as osx have not been tested.

### Install and Building - Requires Rust nightly
For Windows install the Windows build tools (And related Visual Studio components for Rust to work on Windows).

Install OpenSSL - see https://docs.rs/openssl for install information.

Normal Rust build using cargo. The nightly version is set in the rust-toolchain file.

    $ cargo build

# Ci Tests
Travis CI runs test for Linux (Defaults to Ubuntu Xenial).
Appveyor runs tests for Windows x86_64

### Clippy Lints

Clippy runs in CI tests and is required to pass. More info on clippy can be found 
here: https://github.com/rust-lang/rust-clippy

    $ cd rust-onedrive
    $ rustup component add clippy
    $ cargo clippy --all

### Tests and Docs
Of the portions that are implemented there are also examples and docs. Run: 

    $ cargo doc --no-deps --open

### Benchmarks

Criterion.rs is being used to run benchmarks. The basic commands are the same;

    $ cargo bench
    
The benchmark tests are a work in progress and several tests still need to be
added/worked on.

### Use - subject to change.

    See examples directory.