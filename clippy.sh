#!/bin/sh

# this file is an shorthand for the command below
cargo +nightly clippy --all-features -Z unstable-options "$@" -- -D clippy::correctness -W clippy::style -W clippy::complexity -W clippy::perf -W clippy::nursery -W clippy::pedantic -W clippy::cargo