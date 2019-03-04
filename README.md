# Rust-OneDrive

[![Build Status](https://travis-ci.com/sreeise/rust-onedrive.svg?branch=master)](https://travis-ci.com/sreeise/rust-onedrive)

### OneDrive API - Built with Rust

Still in very early development. OAuth2 authentication is the main priority.
The current implementation is fully built to handle web or native clients for
personal and business accounts. However, the api is still a work in progress
and subject to change.

### The following is a list of the current work next to be done:

    1. JWT validation. See issue/bug https://github.com/sreeise/rust-onedrive/issues/4
    2. Access Token updates based upon expires in field. See https://github.com/sreeise/rust-onedrive/issues/5

Requires Rust nightly:

    rustup install nightly
    cd rust-onedrive
    rustup override set nightly

### Tests and Docs

Of the portions that are implemented there are also examples and docs. Run: 

    cargo doc

Tests are run on travis ci and can be run locally through cargo like normal Rust builds:

    cargo test -all

### Use - subject to change.

    See examples directory.