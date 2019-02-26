# Rust-OneDrive

[![Build Status](https://travis-ci.com/sreeise/rust-onedrive.svg?branch=master)](https://travis-ci.com/sreeise/rust-onedrive)

### OneDrive API - Built with Rust

Still in very early development. OAuth2 authentication is the main priority.
The current implementation is fully built to handle web or native clients for
personal and business accounts. However, the api is still a work in progress
and subject to change.

Requires Rust nightly:

    rustup install nightly
    cd rust-onedrive
    rustup override set nightly

### Tests and Docs

Of the portions that are implemented there are also examples and docs. Run: 

    cargo doc

Tests are run on travis ci and can be run locally through cargo like normal Rust builds:

    cargo test

### Use

    See examples directory.