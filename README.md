# Web server in Rust

This is the code I type from the book Zero2Prod in Rust.

## Objective

Create a web server to serve a mailing list.

## CI

Tests: `cargo test`
Linting: `cargo clippy` (`rustup component add clippy`)
Formatting: `cargo fmt` (`rustup component add rustfmt`)
Security: `cargo audit` (`cargo install cargo-audit`)

Add dependencies: `cargo add actix-web --vers 4.0.0` (`cargo install cargo-edit`)

Expand macros: `cargo +nightly expand` (`cargo install cargo-expand`). It only works in the nightly compiler, so `rustup toolchain install nightly --allow-downgrade`