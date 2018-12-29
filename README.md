# Rust-OneDrive

[![Build Status](https://travis-ci.com/sreeise/rust-onedrive.svg?branch=master)](https://travis-ci.com/sreeise/rust-onedrive)

### OneDrive API - Built with Rust

Still in very early development. OAuth2 authentication is the main priority.
Windows and Mac have much better support for OneDrive. This project aims to
bridge the gap between for Linux. Other platforms may be included in
the future and there are only a few places where this build would fail
for other systems.

Requires Rust nightly:

    rustup install nightly
    cd rust-onedrive
    rustup override set nightly

### Tests and Docs

Of the portions that are implemented there are also examples and docs. Run: 

    cargo doc

Tests are run on travis ci and can be run locally through cargo like normal Rust builds:

    cargo test
