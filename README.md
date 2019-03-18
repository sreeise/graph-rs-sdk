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

    1. JWT validation. See issue/bug https://github.com/sreeise/rust-onedrive/issues/4
    2. Access Token updates based upon expires in field. See https://github.com/sreeise/rust-onedrive/issues/5
            Timestamps were added to AccessTokens but there is not checks for expiration yet.
    3. Traits for downloading/moving/uploading and various other actions for drive files.

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