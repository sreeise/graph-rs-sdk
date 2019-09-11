# graph-rs

[![Build Status](https://travis-ci.org/sreeise/graph-rs.svg?branch=master)](https://travis-ci.org/sreeise/graph-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/llvpt7xiy53dmo7a/branch/master?svg=true)](https://ci.appveyor.com/project/sreeise/rust-onedrive)

### Graph API Client in Rust

Disclaimer:
This project integrates mainly with the OneDrive API and much of this work is finished. Other API 
integrations are still being worked on. Since a good part of this API is unstable it should not 
be used in production. 

### Install and Building - Requires Rust nightly
For Windows install the Windows build tools (And related Visual Studio components for Rust to work on Windows).

Install OpenSSL - see https://docs.rs/openssl for install information.

Normal Rust build using cargo. The nightly version is set in the rust-toolchain file.

    $ cargo build

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