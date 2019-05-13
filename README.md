# Rust-OneDrive

[![Build Status](https://travis-ci.com/sreeise/rust-onedrive.svg?branch=master)](https://travis-ci.com/sreeise/rust-onedrive)

### OneDrive API - Built with Rust

Disclaimer:
This project is a work in progress and is not on yet on crates.io. Since a good part of this API is unstable, 
it should not be used in production.
 
Currently, OAuth2 authentication is the main priority.
The current implementation is fully built to handle web or native clients for
personal and business accounts. Most of the OAuth parts will stay the same.

Only Linux has been tested to work. Windows and MacOS may work but is not guaranteed.

### The following is a list of the current work next to be done:

    1. OAuth 2.0 grant types.
    2. Traits for downloading/moving/uploading and various other actions for drive files.

Requires Rust nightly:

    $ rustup install nightly && cd rust-onedrive && rustup override set nightly

### Formatting

Cargo fmt is used for formatting. 

    $ cargo fmt

### Tests and Docs

Of the portions that are implemented there are also examples and docs. Run: 

    $ cargo doc --no-deps --open

Tests are run on travis ci and can be run locally through cargo like normal Rust builds:

    $ cargo test -all

### Use - subject to change.

    See examples directory.