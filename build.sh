#!/bin/bash

set -e

cargo build --release
cargo test
cargo doc
